import { commands as generated } from './bindings';
import { toAppError } from './errors';

type Commands = typeof generated;

/**
 * The single gateway to the Rust backend: the generated command bindings, with
 * every rejection normalized into an {@link AppError}. Tests mock IPC with
 * `@tauri-apps/api/mocks` (see `src/test/ipc.ts`).
 */
export const commands = Object.fromEntries(
  Object.entries(generated).map(([name, fn]) => [
    name,
    async (...args: unknown[]) => {
      try {
        return await (fn as (...a: unknown[]) => Promise<unknown>)(...args);
      } catch (error) {
        throw toAppError(error);
      }
    },
  ]),
) as Commands;
