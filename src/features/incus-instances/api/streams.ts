import { Channel } from '@tauri-apps/api/core';
import {
  commands,
  subscribe,
  toAppError,
  type AppError,
  type ExecEvent,
  type InstanceSpec,
  type PullEvent,
} from '@/shared/api';

/** Makes an Instance; returns what cancels following it (Incus finishes anyway). */
export function startCreateInstance(
  hostId: string,
  project: string,
  spec: InstanceSpec,
  onEvent: (event: PullEvent) => void,
  onError: (error: AppError) => void,
): () => void {
  return subscribe<PullEvent>(
    (ch) => commands.incusInstanceCreateSubscribe(hostId, project, spec, ch),
    onEvent,
    onError,
  );
}

/** An open terminal on an Instance. Input before it opens is dropped. */
export interface IncusTerminalSession {
  input: (data: string) => void;
  resize: (cols: number, rows: number) => void;
  close: () => void;
}

interface Handlers {
  onEvent: (event: ExecEvent) => void;
  onOpen: () => void;
  onError: (error: AppError) => void;
}

/** Wires a terminal the backend opens with `open` to its session commands. */
function session(
  open: (channel: Channel<ExecEvent>) => Promise<number>,
  size: { cols: number; rows: number },
  handlers: Handlers,
): IncusTerminalSession {
  let id: number | null = null;
  let disposed = false;
  let latest = size;
  const channel = new Channel<ExecEvent>();
  channel.onmessage = handlers.onEvent;
  open(channel)
    .then((opened) => {
      if (disposed) {
        void commands.incusTerminalClose(opened);
        return;
      }
      id = opened;
      void commands.incusTerminalResize(opened, latest.cols, latest.rows);
      handlers.onOpen();
    })
    .catch((e: unknown) => handlers.onError(toAppError(e)));
  return {
    input: (data) => {
      if (id !== null) void commands.incusTerminalInput(id, data);
    },
    resize: (cols, rows) => {
      latest = { cols, rows };
      if (id !== null) void commands.incusTerminalResize(id, cols, rows);
    },
    close: () => {
      disposed = true;
      channel.onmessage = () => {};
      if (id !== null) void commands.incusTerminalClose(id);
    },
  };
}

export interface IncusExecRequest {
  command: string;
  user: string;
  cwd: string;
  cols: number;
  rows: number;
}

/** Runs a command (a shell when empty) in a running Instance, with a terminal. */
export function startIncusExec(
  hostId: string,
  project: string,
  name: string,
  request: IncusExecRequest,
  handlers: Handlers,
): IncusTerminalSession {
  const user = request.user.trim() === '' ? null : Number.parseInt(request.user, 10);
  return session(
    (ch) =>
      commands.incusExecStart(
        hostId,
        project,
        name,
        {
          command: request.command || null,
          user: user !== null && Number.isFinite(user) ? user : null,
          cwd: request.cwd || null,
          cols: request.cols,
          rows: request.rows,
        },
        ch,
      ),
    request,
    handlers,
  );
}

/** Attaches to a running Instance's text console. */
export function startIncusConsole(
  hostId: string,
  project: string,
  name: string,
  size: { cols: number; rows: number },
  handlers: Handlers,
): IncusTerminalSession {
  return session(
    (ch) => commands.incusConsoleStart(hostId, project, name, size.cols, size.rows, ch),
    size,
    handlers,
  );
}
