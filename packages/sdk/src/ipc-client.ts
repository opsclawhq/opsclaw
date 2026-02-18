import net from "node:net";
import type { IpcEnvelope } from "./generated/types";

export const IPC_SCHEMA_VERSION = "opsclaw.ipc.v1alpha1";

export function buildIpcEnvelope(params: {
  messageType: string;
  runId?: string;
  payloadJson?: string;
}): IpcEnvelope {
  return {
    schema_version: IPC_SCHEMA_VERSION,
    message_type: params.messageType,
    run_id: params.runId,
    payload_json: params.payloadJson ?? "{}",
    ok: undefined,
    error: undefined,
  };
}

export async function sendIpcRequest(
  socketPath: string,
  envelope: IpcEnvelope,
  timeoutMs = 5_000,
): Promise<IpcEnvelope> {
  return new Promise((resolve, reject) => {
    const socket = net.createConnection(socketPath);
    let settled = false;
    let buffer = "";

    const timeout = setTimeout(() => {
      if (settled) return;
      settled = true;
      socket.destroy();
      reject(new Error(`ipc request timed out after ${timeoutMs}ms`));
    }, timeoutMs);

    socket.on("connect", () => {
      socket.write(`${JSON.stringify(envelope)}\n`);
    });

    socket.on("data", (chunk) => {
      if (settled) return;
      buffer += chunk.toString("utf8");
      const newLineIndex = buffer.indexOf("\n");
      if (newLineIndex === -1) {
        return;
      }

      const line = buffer.slice(0, newLineIndex).trim();
      try {
        const parsed = JSON.parse(line) as IpcEnvelope;
        settled = true;
        clearTimeout(timeout);
        socket.end();
        resolve(parsed);
      } catch (error) {
        settled = true;
        clearTimeout(timeout);
        socket.destroy();
        reject(error);
      }
    });

    socket.on("error", (error) => {
      if (settled) return;
      settled = true;
      clearTimeout(timeout);
      reject(error);
    });

    socket.on("end", () => {
      if (settled) return;
      settled = true;
      clearTimeout(timeout);
      reject(new Error("ipc socket ended before a full response line was received"));
    });
  });
}
