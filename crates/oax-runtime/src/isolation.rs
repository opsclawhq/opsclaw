use std::path::{Path, PathBuf};

use bollard::models::{ContainerCreateBody, HostConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MountSpec {
    pub source: PathBuf,
    pub target: String,
    pub read_only: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AgentContainerSpec {
    pub agent_id: String,
    pub image: String,
    pub network_mode: String,
    pub read_only_root_fs: bool,
    pub workspace_root: PathBuf,
    pub mounts: Vec<MountSpec>,
}

pub fn build_agent_container_spec(
    agent_id: &str,
    image: &str,
    workspace_root: &Path,
) -> Result<AgentContainerSpec, String> {
    if agent_id.trim().is_empty() {
        return Err("agent_id cannot be empty".to_string());
    }
    if image.trim().is_empty() {
        return Err("image cannot be empty".to_string());
    }
    if workspace_root.as_os_str().is_empty() {
        return Err("workspace_root cannot be empty".to_string());
    }

    let spec = AgentContainerSpec {
        agent_id: agent_id.to_string(),
        image: image.to_string(),
        network_mode: "none".to_string(),
        read_only_root_fs: true,
        workspace_root: workspace_root.to_path_buf(),
        mounts: vec![MountSpec {
            source: workspace_root.to_path_buf(),
            target: "/workspace".to_string(),
            read_only: true,
        }],
    };

    validate_isolation_spec(&spec)?;
    Ok(spec)
}

pub fn validate_isolation_spec(spec: &AgentContainerSpec) -> Result<(), String> {
    if spec.network_mode == "host" {
        return Err("host network mode is forbidden".to_string());
    }
    if spec.network_mode != "none" {
        return Err(format!(
            "invalid network mode '{}': only 'none' is allowed",
            spec.network_mode
        ));
    }
    if spec.mounts.is_empty() {
        return Err("at least one scoped mount is required".to_string());
    }

    for mount in &spec.mounts {
        if !mount.target.starts_with('/') {
            return Err(format!(
                "mount target '{}' must be an absolute container path",
                mount.target
            ));
        }
        if mount.target == "/" {
            return Err("mount target '/' is not allowed".to_string());
        }
        if !mount.source.starts_with(&spec.workspace_root) {
            return Err(format!(
                "mount source '{}' escapes workspace root '{}'",
                mount.source.display(),
                spec.workspace_root.display()
            ));
        }
    }

    Ok(())
}

pub fn to_bollard_config(spec: &AgentContainerSpec) -> ContainerCreateBody {
    let binds: Vec<String> = spec
        .mounts
        .iter()
        .map(|m| {
            format!(
                "{}:{}:{}",
                m.source.display(),
                m.target,
                if m.read_only { "ro" } else { "rw" }
            )
        })
        .collect();

    ContainerCreateBody {
        image: Some(spec.image.clone()),
        working_dir: Some("/workspace".to_string()),
        host_config: Some(HostConfig {
            network_mode: Some(spec.network_mode.clone()),
            readonly_rootfs: Some(spec.read_only_root_fs),
            binds: Some(binds),
            ..Default::default()
        }),
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{
        build_agent_container_spec, to_bollard_config, validate_isolation_spec, AgentContainerSpec,
    };

    #[test]
    fn build_spec_enforces_no_host_network_and_read_only_root() {
        let workspace = PathBuf::from("/tmp/opsclaw-workspace");
        let spec =
            build_agent_container_spec("sre-agent", "ghcr.io/opsclaw/agent:latest", &workspace)
                .expect("spec should build");

        assert_eq!(spec.network_mode, "none");
        assert!(spec.read_only_root_fs);
        assert_eq!(spec.mounts.len(), 1);
        assert_eq!(spec.mounts[0].target, "/workspace");
    }

    #[test]
    fn validate_rejects_host_network_mode() {
        let spec = AgentContainerSpec {
            agent_id: "sre-agent".to_string(),
            image: "ghcr.io/opsclaw/agent:latest".to_string(),
            network_mode: "host".to_string(),
            workspace_root: PathBuf::from("/tmp/opsclaw-workspace"),
            ..Default::default()
        };

        let err = validate_isolation_spec(&spec).expect_err("host network must be rejected");
        assert!(err.contains("host network"));
    }

    #[test]
    fn bollard_config_contains_expected_isolation_flags() {
        let workspace = PathBuf::from("/tmp/opsclaw-workspace");
        let spec =
            build_agent_container_spec("deploy-agent", "ghcr.io/opsclaw/agent:latest", &workspace)
                .expect("spec should build");
        let config = to_bollard_config(&spec);
        let host = config.host_config.expect("host config should exist");

        assert_eq!(host.network_mode.as_deref(), Some("none"));
        assert_eq!(host.readonly_rootfs, Some(true));
        assert!(host
            .binds
            .unwrap_or_default()
            .iter()
            .any(|b| b.contains("/tmp/opsclaw-workspace:/workspace:ro")));
    }
}
