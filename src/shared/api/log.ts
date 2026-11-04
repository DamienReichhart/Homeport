import { error as logError, warn as logWarn } from '@tauri-apps/plugin-log';
import type { AppError } from './errors';

/** One line for the app log: where, code, message and the raw detail. */
export function logLine(error: AppError): string {
  const where = error.context.command ?? 'frontend';
  const detail = error.detail ? ` (${error.detail.replace(/\s+/g, ' ').trim()})` : '';
  return `${where}: [${error.code}] ${error.english}${detail}`;
}

/**
 * Writes a failure to Homeport's log file, next to the backend's entries.
 * Logging must never fail the caller, so errors are swallowed.
 */
export function logFailure(error: AppError, level: 'warn' | 'error' = 'warn'): void {
  const write = level === 'error' ? logError : logWarn;
  try {
    void write(logLine(error)).catch(() => {});
  } catch {
    // Outside Tauri (tests, screenshots) there is no log to write to.
  }
}
