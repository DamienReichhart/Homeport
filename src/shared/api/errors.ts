import type { AppError as AppErrorPayload, AppInfo, ErrorCode, Hint } from './bindings';

export type { AppErrorPayload };

/** Where a failure happened, for the Details dialog and bug reports. */
export interface ErrorContext {
  /** The backend command that failed, as named in `commands`, e.g. `containersAct`. */
  command?: string;
  at: Date;
}

/**
 * A failure as the user sees it: what happened (`message`), what to do
 * (`hint`) and the raw cause (`detail`). Mirrors `crates/core/src/error/mod.rs`.
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
  // Anything else is a bug or an unclassified plugin failure: its raw text
  // goes to the details, not the message.
  if (error instanceof Error) {
    const stack = error.stack ?? '';
    const head = `${error.name}: ${error.message}`;
    return new AppError({
      code: 'unknown',
      message: UNEXPECTED,
      detail: stack.startsWith(head) ? stack : `${head}\n${stack}`.trim(),
    });
  }
  return new AppError({ code: 'unknown', message: UNEXPECTED, detail: String(error) });
}

/** The message of a failure Homeport didn't expect; the details say more. */
export const UNEXPECTED = 'Something unexpected went wrong.';

/** The hint's commands as shell prompt lines (`  $ …`), one per line of each command. */
export function commandLines(hint: Hint): string[] {
  return hint.commands.flatMap((command) => command.split('\n').map((l) => `  $ ${l}`));
}

/**
 * Everything known about a failure as plain text, ready to paste into a bug
 * report. `app` adds the version and platform when known.
 */
export function errorReport(error: AppError, app?: AppInfo): string {
  const lines = [
    app ? `${app.name} ${app.version} on ${app.os} ${app.arch}` : 'Homeport',
    `When: ${error.context.at.toISOString()}`,
    ...(error.context.command ? [`Command: ${error.context.command}`] : []),
    `Code: ${error.code}`,
    '',
    error.message,
  ];
  if (error.hint) {
    lines.push('', `What to do: ${error.hint.text}`);
    lines.push(...commandLines(error.hint));
  }
  if (error.detail) lines.push('', 'Detail:', error.detail);
  return lines.join('\n');
}
