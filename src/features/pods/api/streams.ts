import { Channel } from '@tauri-apps/api/core';
import type { LogStreamer } from '@/features/containers';
import {
  commands,
  subscribe,
  toAppError,
  type AppError,
  type ExecEvent,
  type LogEvent,
  type PodLogSource,
} from '@/shared/api';

/** Opens the logs of one or more Pods' `container` (or its previous instance), for the log viewer. */
export function podLogs(
  clusterId: string,
  namespace: string,
  sources: PodLogSource[],
  container: string | null,
  previous: boolean,
): LogStreamer {
  return (options, onEvent, onError) =>
    subscribe<LogEvent>(
      (ch) =>
        commands.clustersPodLogsSubscribe(
          clusterId,
          namespace,
          sources,
          { container, previous },
          options,
          ch,
        ),
      onEvent,
      onError,
    );
}

/** An open terminal in a Pod. Input before it opens is dropped. */
export interface PodTerminalSession {
  input: (data: string) => void;
  resize: (cols: number, rows: number) => void;
  close: () => void;
}

/**
 * Opens a terminal in a Pod's container. Its output goes to `onEvent`;
 * `onOpen` runs once the backend has it, at the latest size.
 */
export function startPodTerminal(
  clusterId: string,
  namespace: string,
  pod: string,
  request: { container: string; command: string; cols: number; rows: number },
  handlers: {
    onEvent: (event: ExecEvent) => void;
    onOpen: () => void;
    onError: (error: AppError) => void;
  },
): PodTerminalSession {
  let id: number | null = null;
  let disposed = false;
  let size = { cols: request.cols, rows: request.rows };
  const channel = new Channel<ExecEvent>();
  channel.onmessage = handlers.onEvent;
  commands
    .clustersExecStart(
      clusterId,
      namespace,
      pod,
      {
        container: request.container || null,
        command: request.command || null,
        cols: size.cols,
        rows: size.rows,
      },
      channel,
    )
    .then((terminal) => {
      if (disposed) {
        void commands.clustersExecClose(terminal);
        return;
      }
      id = terminal;
      void commands.clustersExecResize(terminal, size.cols, size.rows);
      handlers.onOpen();
    })
    .catch((e: unknown) => handlers.onError(toAppError(e)));
  return {
    input: (data) => {
      if (id !== null) void commands.clustersExecInput(id, data);
    },
    resize: (cols, rows) => {
      size = { cols, rows };
      if (id !== null) void commands.clustersExecResize(id, cols, rows);
    },
    close: () => {
      disposed = true;
      channel.onmessage = () => {};
      if (id !== null) void commands.clustersExecClose(id);
    },
  };
}
