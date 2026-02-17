use oax_core::types::IpcEnvelope;
use oax_runtime::ipc::{
    handle_control_message, handle_runtime_message, malformed_line_error_response, parse_ipc_line,
    serialize_ipc_line, IPC_SCHEMA_VERSION,
};
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
enum SocketRole {
    Runtime,
    Control,
}

#[derive(Clone)]
pub struct IpcSocketServerConfig {
    pub socket_dir: PathBuf,
    pub max_messages_per_socket: Option<usize>,
    pub poll_interval_ms: u64,
}

impl IpcSocketServerConfig {
    pub fn new(socket_dir: PathBuf) -> Self {
        Self {
            socket_dir,
            max_messages_per_socket: None,
            poll_interval_ms: 20,
        }
    }
}

pub fn runtime_socket_path(socket_dir: &Path) -> PathBuf {
    socket_dir.join("runtime.sock")
}

pub fn control_socket_path(socket_dir: &Path) -> PathBuf {
    socket_dir.join("control.sock")
}

pub fn serve_ipc_sockets(socket_dir: &Path) -> io::Result<()> {
    serve_ipc_sockets_with_config(IpcSocketServerConfig::new(socket_dir.to_path_buf()))
}

pub(crate) fn serve_ipc_sockets_with_config(config: IpcSocketServerConfig) -> io::Result<()> {
    fs::create_dir_all(&config.socket_dir)?;

    let runtime_path = runtime_socket_path(&config.socket_dir);
    let control_path = control_socket_path(&config.socket_dir);

    let stop = Arc::new(AtomicBool::new(false));

    let runtime_thread = {
        let stop = Arc::clone(&stop);
        let config = config.clone();
        thread::spawn(move || {
            serve_single_socket(
                &runtime_path,
                SocketRole::Runtime,
                stop,
                config.max_messages_per_socket,
                config.poll_interval_ms,
            )
        })
    };

    let control_thread = {
        let stop = Arc::clone(&stop);
        let config = config.clone();
        thread::spawn(move || {
            serve_single_socket(
                &control_path,
                SocketRole::Control,
                stop,
                config.max_messages_per_socket,
                config.poll_interval_ms,
            )
        })
    };

    let runtime_result = runtime_thread
        .join()
        .map_err(|_| io::Error::other("runtime socket thread panicked"))?;
    let control_result = control_thread
        .join()
        .map_err(|_| io::Error::other("control socket thread panicked"))?;

    runtime_result?;
    control_result?;

    Ok(())
}

fn serve_single_socket(
    socket_path: &Path,
    role: SocketRole,
    stop: Arc<AtomicBool>,
    max_messages: Option<usize>,
    poll_interval_ms: u64,
) -> io::Result<()> {
    if socket_path.exists() {
        fs::remove_file(socket_path)?;
    }

    let listener = UnixListener::bind(socket_path)?;
    listener.set_nonblocking(true)?;

    let mut handled = 0usize;

    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((stream, _)) => {
                handle_connection(stream, role, &stop, &mut handled, max_messages)?;
            }
            Err(err) if err.kind() == io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(poll_interval_ms));
            }
            Err(err) => return Err(err),
        }

        if max_messages.is_some_and(|max| handled >= max) {
            stop.store(true, Ordering::SeqCst);
        }
    }

    drop(listener);
    if socket_path.exists() {
        let _ = fs::remove_file(socket_path);
    }

    Ok(())
}

fn handle_connection(
    mut stream: UnixStream,
    role: SocketRole,
    stop: &Arc<AtomicBool>,
    handled: &mut usize,
    max_messages: Option<usize>,
) -> io::Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        if stop.load(Ordering::SeqCst) {
            break;
        }

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        let trimmed = line.trim_end_matches(['\n', '\r']);
        if trimmed.is_empty() {
            continue;
        }

        let (response, should_stop) = handle_socket_line(role, trimmed);
        let response_line = serialize_ipc_line(&response)
            .unwrap_or_else(|_| fallback_error_line("failed to serialize response envelope"));

        stream.write_all(response_line.as_bytes())?;
        stream.write_all(b"\n")?;
        stream.flush()?;

        *handled += 1;

        if should_stop || max_messages.is_some_and(|max| *handled >= max) {
            stop.store(true, Ordering::SeqCst);
            break;
        }
    }

    Ok(())
}

fn handle_socket_line(role: SocketRole, line: &str) -> (IpcEnvelope, bool) {
    match parse_ipc_line(line) {
        Ok(request) => match role {
            SocketRole::Runtime => (handle_runtime_message(&request), false),
            SocketRole::Control => {
                let should_stop = request.schema_version == IPC_SCHEMA_VERSION
                    && request.message_type == "control.stop";
                (handle_control_message(&request), should_stop)
            }
        },
        Err(err) => (malformed_line_error_response(&err), false),
    }
}

fn fallback_error_line(error: &str) -> String {
    format!(
        "{{\"schema_version\":\"{}\",\"message_type\":\"error\",\"run_id\":null,\"payload_json\":\"{{}}\",\"ok\":false,\"error\":\"{}\"}}",
        IPC_SCHEMA_VERSION,
        error.replace('"', "\\\"")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn wait_for_socket(path: &Path) {
        for _ in 0..100 {
            if path.exists() {
                return;
            }
            thread::sleep(Duration::from_millis(10));
        }
        panic!("socket was not created in time: {}", path.display());
    }

    fn send_line(path: &Path, request_line: &str) -> String {
        let mut stream = UnixStream::connect(path).expect("should connect to socket");
        stream
            .write_all(format!("{}\n", request_line).as_bytes())
            .expect("should write request line");
        stream.flush().expect("flush should succeed");

        let mut reader = BufReader::new(stream);
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .expect("should read response line");
        response.trim_end_matches(['\n', '\r']).to_string()
    }

    #[test]
    fn runtime_socket_handles_ping_request() {
        let dir = tempdir().expect("tempdir should be created");
        let config = IpcSocketServerConfig {
            socket_dir: dir.path().to_path_buf(),
            max_messages_per_socket: Some(1),
            poll_interval_ms: 5,
        };

        let runtime_path = runtime_socket_path(&config.socket_dir);

        let server = thread::spawn(move || serve_ipc_sockets_with_config(config));
        wait_for_socket(&runtime_path);

        let request = IpcEnvelope {
            schema_version: IPC_SCHEMA_VERSION.to_string(),
            message_type: "runtime.ping".to_string(),
            run_id: Some("run-123".to_string()),
            payload_json: "{}".to_string(),
            ok: None,
            error: None,
        };
        let request_line = serialize_ipc_line(&request).expect("request should serialize");
        let response_line = send_line(&runtime_path, &request_line);
        let response = parse_ipc_line(&response_line).expect("response should parse");

        assert_eq!(response.ok, Some(true));
        assert_eq!(response.message_type, "runtime.pong");

        server
            .join()
            .expect("server thread should not panic")
            .expect("server should exit cleanly");
    }

    #[test]
    fn control_socket_handles_health_request() {
        let dir = tempdir().expect("tempdir should be created");
        let config = IpcSocketServerConfig {
            socket_dir: dir.path().to_path_buf(),
            max_messages_per_socket: Some(1),
            poll_interval_ms: 5,
        };

        let control_path = control_socket_path(&config.socket_dir);

        let server = thread::spawn(move || serve_ipc_sockets_with_config(config));
        wait_for_socket(&control_path);

        let request = IpcEnvelope {
            schema_version: IPC_SCHEMA_VERSION.to_string(),
            message_type: "control.health".to_string(),
            run_id: None,
            payload_json: "{}".to_string(),
            ok: None,
            error: None,
        };
        let request_line = serialize_ipc_line(&request).expect("request should serialize");
        let response_line = send_line(&control_path, &request_line);
        let response = parse_ipc_line(&response_line).expect("response should parse");

        assert_eq!(response.ok, Some(true));
        assert_eq!(response.message_type, "control.health.ok");

        server
            .join()
            .expect("server thread should not panic")
            .expect("server should exit cleanly");
    }

    #[test]
    fn control_stop_message_requests_server_shutdown() {
        let dir = tempdir().expect("tempdir should be created");
        let config = IpcSocketServerConfig {
            socket_dir: dir.path().to_path_buf(),
            max_messages_per_socket: Some(10),
            poll_interval_ms: 5,
        };

        let control_path = control_socket_path(&config.socket_dir);

        let server = thread::spawn(move || serve_ipc_sockets_with_config(config));
        wait_for_socket(&control_path);

        let request = IpcEnvelope {
            schema_version: IPC_SCHEMA_VERSION.to_string(),
            message_type: "control.stop".to_string(),
            run_id: None,
            payload_json: "{}".to_string(),
            ok: None,
            error: None,
        };
        let request_line = serialize_ipc_line(&request).expect("request should serialize");
        let response_line = send_line(&control_path, &request_line);
        let response = parse_ipc_line(&response_line).expect("response should parse");

        assert_eq!(response.ok, Some(true));
        assert_eq!(response.message_type, "control.stop.ack");

        server
            .join()
            .expect("server thread should not panic")
            .expect("server should exit cleanly");
    }
}
