import { toAppError, type AppError, type PullEvent } from '@/shared/api';

/** An Incus operation that downloads or unpacks, as its dialog shows it. */
export interface OperationState {
  /** Incus's own words: "rootfs: 42% (12.30MB/s)". */
  status: string | null;
  percent: number | null;
  done: boolean;
  error: AppError | null;
}

export const initialProgress: OperationState = {
  status: null,
  percent: null,
  done: false,
  error: null,
};

/** Folds one progress event into what the dialog shows. */
export function reduceProgress(state: OperationState, event: PullEvent): OperationState {
  switch (event.type) {
    case 'done':
      return { ...state, done: true, percent: 100 };
    case 'error':
      return { ...state, error: toAppError(event.error) };
    case 'progress':
      return {
        ...state,
        status: event.status,
        percent:
          event.current !== null && event.total
            ? Math.round((100 * event.current) / event.total)
            : null,
      };
  }
}
