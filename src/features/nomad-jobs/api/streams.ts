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

/** Opens the logs of task `task` of allocation `allocId`, for the log viewer. */
export function nomadLogs(clusterId: string, allocId: string, task: string): LogStreamer {
  return (options, onEvent, onError) =>
    subscribe<LogEvent>(
      (ch) => commands.clustersNomadLogsSubscribe(clusterId, allocId, task, options, ch),
      onEvent,
      onError,
    );
}

/** An open terminal in a task. Input before it opens is dropped. */
export interface NomadTerminalSession {
  input: (data: string) => void;
  resize: (cols: number, rows: number) => void;
  close: () => void;
}

/**
 * Opens a terminal in a task of an allocation. Its output goes to
 * `onEvent`; `onOpen` runs once the backend has it, at the latest size.
 */
export function startNomadTerminal(
  clusterId: string,
  allocId: string,
  request: { task: string; command: string; cols: number; rows: number },
  handlers: {
    onEvent: (event: ExecEvent) => void;
    onOpen: () => void;
    onError: (error: AppError) => void;
  },
): NomadTerminalSession {
  let id: number | null = null;
  let disposed = false;
  let size = { cols: request.cols, rows: request.rows };
  const channel = new Channel<ExecEvent>();
  channel.onmessage = handlers.onEvent;
  commands
    .clustersNomadExecStart(
      clusterId,
      allocId,
      {
        task: request.task,
        command: request.command || null,
        cols: size.cols,
        rows: size.rows,
      },
      channel,
    )
    .then((terminal) => {
      if (disposed) {
        void commands.clustersNomadExecClose(terminal);
        return;
      }
      id = terminal;
      void commands.clustersNomadExecResize(terminal, size.cols, size.rows);
      handlers.onOpen();
    })
    .catch((e: unknown) => handlers.onError(toAppError(e)));
  return {
    input: (data) => {
      if (id !== null) void commands.clustersNomadExecInput(id, data);
    },
    resize: (cols, rows) => {
      size = { cols, rows };
      if (id !== null) void commands.clustersNomadExecResize(id, cols, rows);
    },
    close: () => {
      disposed = true;
      channel.onmessage = () => {};
      if (id !== null) void commands.clustersNomadExecClose(id);
    },
  };
}
