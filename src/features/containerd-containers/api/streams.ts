import { Channel } from '@tauri-apps/api/core';
import type { LogStreamer } from '@/features/containers';
import {
  commands,
  subscribe,
  toAppError,
  type AppError,
  type ExecEvent,
  type LogEvent,
} from '@/shared/api';

/** The logs of a containerd container, read from the file its creator wrote. */
export function containerdLogs(hostId: string, namespace: string, id: string): LogStreamer {
  return (options, onEvent, onError) =>
    subscribe<LogEvent>(
      (ch) => commands.containerdLogsSubscribe(hostId, namespace, id, options, ch),
      onEvent,
      onError,
    );
}

export interface ContainerdExecRequest {
  command: string;
  user: string;
  workingDir: string;
  cols: number;
  rows: number;
}

/** An open terminal in a containerd container. Input before it opens is dropped. */
export interface ContainerdExecSession {
  input: (data: string) => void;
  resize: (cols: number, rows: number) => void;
  close: () => void;
}

/**
 * Opens a terminal in a containerd container's task. Its output goes to
 * `onEvent`; `onOpen` runs once the backend has it, at the latest size.
 */
export function startContainerdExec(
  hostId: string,
  namespace: string,
  id: string,
  request: ContainerdExecRequest,
  handlers: {
    onEvent: (event: ExecEvent) => void;
    onOpen: () => void;
    onError: (error: AppError) => void;
  },
): ContainerdExecSession {
  let session: number | null = null;
  let disposed = false;
  let size = { cols: request.cols, rows: request.rows };
  const channel = new Channel<ExecEvent>();
  channel.onmessage = handlers.onEvent;
  commands
    .containerdExecStart(
      hostId,
      namespace,
      id,
      {
        command: request.command || null,
        user: request.user || null,
        workingDir: request.workingDir || null,
        cols: size.cols,
        rows: size.rows,
      },
      channel,
    )
    .then((opened) => {
      if (disposed) {
        void commands.containerdExecClose(opened);
        return;
      }
      session = opened;
      void commands.containerdExecResize(opened, size.cols, size.rows);
      handlers.onOpen();
    })
    .catch((e: unknown) => handlers.onError(toAppError(e)));
  return {
    input: (data) => {
      if (session !== null) void commands.containerdExecInput(session, data);
    },
    resize: (cols, rows) => {
      size = { cols, rows };
      if (session !== null) void commands.containerdExecResize(session, cols, rows);
    },
    close: () => {
      disposed = true;
      channel.onmessage = () => {};
      if (session !== null) void commands.containerdExecClose(session);
    },
  };
}
