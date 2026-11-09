import { formatKey, hasMessage } from '@/shared/i18n';
import type { AppError as AppErrorPayload, AppInfo, ErrorCode, Hint, Param } from './bindings';

export type { AppErrorPayload };

type Params = { [key in string]: Param };

/** A message from the backend: its catalogue key and parameters. */
interface Keyed {
  key?: string;
  params?: Params;
}

/** Keys the catalogue lacked, reported once each. */
const reported = new Set<string>();

/**
 * The text for `key` from the catalogue, or `english` when the key is empty
 * (text not in the catalogue yet) or unknown — a backend newer than this
 * frontend, or a missing entry, which is worth a warning.
 */
function render({ key, params }: Keyed, english: string): string {
  if (!key) return english;
  if (!hasMessage(key)) {
    if (!reported.has(key)) {
      reported.add(key);
      console.warn(`The catalogue has no ${key}; showing the backend's English.`);
    }
    return english;
  }
  return formatKey(key, params);
}

/** Where a failure happened, for the Details dialog and bug reports. */
export interface ErrorContext {
  /** The backend command that failed, as named in `commands`, e.g. `containersAct`. */
  command?: string;
  at: Date;
}

/**
 * A failure as the user sees it: what happened (`message`), what to do
 * (`hint`) and the raw cause (`detail`). Mirrors `crates/core/src/error/mod.rs`:
 * message and hint are rendered from the catalogue by key (ADR 0011);
 * `english` keeps the backend's own words for the log and bug reports.
 */
export class AppError extends Error {
  readonly code: ErrorCode;
  readonly key: string;
  readonly params: Params;
  /** The message in English, as the backend wrote it. */
  readonly english: string;
  readonly hint: (Hint & { english: string }) | null;
  readonly detail: string | null;
  readonly context: ErrorContext;

  constructor(payload: ErrorFields, context?: Partial<ErrorContext>) {
    const english = payload.english ?? payload.message;
    super(english);
    this.name = 'AppError';
    this.code = payload.code;
    this.key = payload.key ?? '';
    this.params = payload.params ?? {};
    this.english = english;
    // Rendered on every read, so an error on screen follows a change of language.
    const keyed = { key: this.key, params: this.params };
    Object.defineProperty(this, 'message', {
      get: () => render(keyed, english),
      enumerable: true,
      configurable: true,
    });
    const hint = payload.hint;
    if (hint) {
      const hintEnglish = hint.english ?? hint.text;
      const hintKeyed = { key: hint.key ?? '', params: hint.params ?? {} };
      this.hint = {
        ...hintKeyed,
        // `text` is shown; the key stays so the hint renders again in context.
        get text() {
          return render(hintKeyed, hintEnglish);
        },
        commands: hint.commands,
        english: hintEnglish,
      };
    } else {
      this.hint = null;
    }
    this.detail = payload.detail ?? null;
    this.context = { at: new Date(), ...context };
  }

  /** The same failure, tagged with where it happened. */
  withContext(context: Partial<ErrorContext>): AppError {
    return new AppError({ ...this, message: this.english }, { ...this.context, ...context });
  }
}

/**
 * A backend payload, or a failure that started in the frontend (`unknown`).
 * Without a key, `message` and the hint's text are shown as they are.
 */
export interface ErrorFields extends Keyed {
  code: ErrorCode;
  message: string;
  /** Set when `message` is already rendered; the English to keep. */
  english?: string;
  hint?: (Omit<Hint, 'key' | 'params'> & Keyed & { english?: string }) | null;
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
 * report, in English whatever the app's language. `app` adds the version and
 * platform when known.
 */
export function errorReport(error: AppError, app?: AppInfo): string {
  const lines = [
    app ? `${app.name} ${app.version} on ${app.os} ${app.arch}` : 'Homeport',
    `When: ${error.context.at.toISOString()}`,
    ...(error.context.command ? [`Command: ${error.context.command}`] : []),
    `Code: ${error.code}`,
    '',
    error.english,
  ];
  if (error.hint) {
    lines.push('', `What to do: ${error.hint.english}`);
    lines.push(...commandLines(error.hint));
  }
  if (error.detail) lines.push('', 'Detail:', error.detail);
  return lines.join('\n');
}
