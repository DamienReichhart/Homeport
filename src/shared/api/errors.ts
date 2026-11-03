import type { AppError as AppErrorPayload, ErrorCode, Hint } from './bindings';

export type { AppErrorPayload };

/** Where a failure happened, for the Details dialog and bug reports. */
export interface ErrorContext {
  /** The backend command that failed, e.g. `containers_act`. */
  command?: string;
  hostId?: string;
  at: Date;
}

/**
 * A failure as the user sees it: what happened (`message`), what to do
 * (`hint`) and the raw cause (`detail`). Mirrors `src-tauri/src/error/mod.rs`.
 */
export class AppError extends Error {
  readonly code: ErrorCode;
  readonly hint: Hint | null;
  readonly detail: string | null;
  readonly context: ErrorContext;

  constructor(payload: ErrorFields, context?: Partial<ErrorContext>) {
    super(payload.message);
    this.name = 'AppError';
    this.code = payload.code;
    this.hint = payload.hint ?? null;
    this.detail = payload.detail ?? null;
    this.context = { at: new Date(), ...context };
  }

  /** The same failure, tagged with where it happened. */
  withContext(context: Partial<ErrorContext>): AppError {
    return new AppError(this, { ...this.context, ...context });
  }
}

/** A backend payload, or a failure that started in the frontend (`unknown`). */
export interface ErrorFields {
  code: ErrorCode;
  message: string;
  hint?: Hint | null;
  detail?: string | null;
}

function isAppErrorPayload(value: unknown): value is AppErrorPayload {
  return (
    typeof value === 'object' &&
    value !== null &&
    typeof (value as AppErrorPayload).code === 'string' &&
    typeof (value as AppErrorPayload).message === 'string'
  );
}

/** Normalizes anything thrown by `invoke` (or by our own code) into an {@link AppError}. */
export function toAppError(error: unknown): AppError {
  if (error instanceof AppError) return error;
  if (isAppErrorPayload(error)) return new AppError(error);
  if (error instanceof Error)
    return new AppError({
      code: 'unknown',
      message: error.message || 'Something went wrong.',
      detail: error.stack ?? null,
    });
  return new AppError({ code: 'unknown', message: String(error) });
}
