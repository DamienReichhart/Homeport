import { Channel } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import { useEffect, useState } from 'react';
import {
  commands,
  subscribe,
  toAppError,
  type AppError,
  type ContainerUsage,
  type ExecEvent,
  type LogEvent,
  type LogOptions,
  type LogSource,
  type StatsSample,
  type TransferEvent,
} from '@/shared/api';
import { STATS_WINDOW, type StatsPoint } from '../model/stats';

/** Live CPU and memory of every running container, keyed by id. */
export function useUsage(hostId: string): Map<string, ContainerUsage> {
  const [usage, setUsage] = useState(new Map<string, ContainerUsage>());
  useEffect(
    () =>
      subscribe<ContainerUsage[]>(
        (ch) => commands.usageSubscribe(hostId, ch),
        (list) => setUsage(new Map(list.map((u) => [u.id, u]))),
      ),
    [hostId],
  );
  return usage;
}

/** The last few minutes of a running container's stats, one point per sample. */
export function useStats(hostId: string, containerId: string, running: boolean): StatsPoint[] {
  const [points, setPoints] = useState<StatsPoint[]>([]);
  useEffect(() => {
    if (!running) return;
    return subscribe<StatsSample>(
      (ch) => commands.statsSubscribe(hostId, containerId, ch),
      (sample) =>
        setPoints((list) => [
          ...list.slice(-(STATS_WINDOW - 1)),
          { time: Date.now() / 1000, sample },
        ]),
    );
  }, [hostId, containerId, running]);
  return points;
}

/** Streams the logs of `sources`; the returned function stops the stream. */
export function streamLogs(
  hostId: string,
  sources: LogSource[],
  options: LogOptions,
  onEvent: (event: LogEvent) => void,
  onError: (error: AppError) => void,
): () => void {
  return subscribe<LogEvent>(
    (ch) => commands.logsSubscribe(hostId, sources, options, ch),
    onEvent,
    onError,
  );
}

export interface ExecRequest {
  command: string;
  user: string;
  workingDir: string;
  cols: number;
  rows: number;
}

/** An interactive exec session. Input before it opens is dropped. */
export interface ExecSession {
  input: (data: string) => void;
  resize: (cols: number, rows: number) => void;
  close: () => void;
}

/**
 * Starts an interactive exec session in a container. Its output goes to
 * `onEvent`; `onOpen` runs once the backend has it, at the latest size.
 */
export function startExec(
  hostId: string,
  containerId: string,
  request: ExecRequest,
  handlers: {
    onEvent: (event: ExecEvent) => void;
    onOpen: () => void;
    onError: (error: AppError) => void;
  },
): ExecSession {
  let session: number | null = null;
  let disposed = false;
  let size = { cols: request.cols, rows: request.rows };
  const channel = new Channel<ExecEvent>();
  channel.onmessage = handlers.onEvent;
  commands
    .execStart(
      hostId,
      containerId,
      {
        command: request.command || null,
        user: request.user || null,
        workingDir: request.workingDir || null,
        cols: size.cols,
        rows: size.rows,
      },
      channel,
    )
    .then((id) => {
      if (disposed) {
        void commands.execClose(id);
        return;
      }
      session = id;
      void commands.execResize(id, size.cols, size.rows);
      handlers.onOpen();
    })
    .catch((e: unknown) => handlers.onError(toAppError(e)));
  return {
    input: (data) => {
      if (session !== null) void commands.execInput(session, data);
    },
    resize: (cols, rows) => {
      size = { cols, rows };
      if (session !== null) void commands.execResize(session, cols, rows);
    },
    close: () => {
      disposed = true;
      channel.onmessage = () => {};
      if (session !== null) void commands.execClose(session);
    },
  };
}

/** Exports a container's filesystem to a tar on this machine; returns the canceller. */
export function exportContainer(
  hostId: string,
  containerId: string,
  destination: string,
  onEvent: (event: TransferEvent) => void,
  onError: (error: AppError) => void,
): () => void {
  return subscribe<TransferEvent>(
    (ch) => commands.containersExportSubscribe(hostId, containerId, destination, ch),
    (event) => {
      onEvent(event);
      if (event.type === 'done') toast.success(`Exported to ${destination}`);
    },
    onError,
  );
}
