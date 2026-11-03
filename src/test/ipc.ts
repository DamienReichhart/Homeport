import { mockIPC } from '@tauri-apps/api/mocks';

type Handler = (args: Record<string, unknown>) => unknown;

/**
 * Mocks the Tauri IPC layer for a test. Keys are Rust command names
 * (`get_settings`), values compute the response from the invoke arguments.
 * Unknown commands reject so missing mocks fail loudly.
 */
export function mockCommands(handlers: Record<string, Handler>) {
  mockIPC(
    (cmd, args) => {
      // Frontend failures are written to the app log; tests don't check it.
      if (cmd === 'plugin:log|log') return null;
      const handler = handlers[cmd];
      if (!handler) throw new Error(`unexpected command ${cmd}`);
      return handler((args ?? {}) as Record<string, unknown>);
    },
    { shouldMockEvents: true },
  );
}

const sent = new Map<number, number>();

/** Pushes `messages`, in order, into a Tauri channel a mocked command received. */
export function sendToChannel(channel: unknown, ...messages: unknown[]) {
  const { id } = channel as { id: number };
  const internals = (
    window as unknown as {
      __TAURI_INTERNALS__: { runCallback: (id: number, data: unknown) => void };
    }
  ).__TAURI_INTERNALS__;
  for (const message of messages) {
    const index = sent.get(id) ?? 0;
    sent.set(id, index + 1);
    internals.runCallback(id, { index, message });
  }
}
