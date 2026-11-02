/** Shape of errors serialized by the Rust backend (see src-tauri/src/error.rs). */
export interface AppErrorPayload {
  code: string;
  message: string;
}

export class AppError extends Error {
  readonly code: string;

  constructor(payload: AppErrorPayload) {
    super(payload.message);
    this.name = 'AppError';
    this.code = payload.code;
  }
}

function isAppErrorPayload(value: unknown): value is AppErrorPayload {
  return (
    typeof value === 'object' &&
    value !== null &&
    typeof (value as AppErrorPayload).code === 'string' &&
    typeof (value as AppErrorPayload).message === 'string'
  );
}

/** Normalizes anything thrown by `invoke` into an {@link AppError}. */
export function toAppError(error: unknown): AppError {
  if (error instanceof AppError) return error;
  if (isAppErrorPayload(error)) return new AppError(error);
  if (error instanceof Error) return new AppError({ code: 'unknown', message: error.message });
  return new AppError({ code: 'unknown', message: String(error) });
}
