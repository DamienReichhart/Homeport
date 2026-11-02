import { invoke } from '@tauri-apps/api/core';
import { toAppError } from './errors';

/**
 * The single gateway to the Rust backend. Every feature talks to Tauri through
 * this function so errors are normalized and IPC can be mocked in one place.
 */
export async function invokeCommand<T>(
  command: string,
  args?: Record<string, unknown>,
): Promise<T> {
  try {
    return await invoke<T>(command, args);
  } catch (error) {
    throw toAppError(error);
  }
}
