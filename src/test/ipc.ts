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
