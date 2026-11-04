import { toAppError, type AppError, type PullEvent } from '@/shared/api';
import { t } from '@/shared/i18n';

/**
 * A push as the dialog shows it. The Engine API client drops the layer each
 * push event is about, so progress is counted per status, not per layer.
 */
export interface PushState {
  /** Image-level lines: the repository pushed to, the digest at the end. */
  messages: string[];
  /** Layers uploaded, and layers the registry already had. */
  pushed: number;
  existing: number;
  /** The upload in progress, from the latest `Pushing` event. */
  current: number | null;
  total: number | null;
  done: boolean;
  error: AppError | null;
}

export const initialPush: PushState = {
  messages: [],
  pushed: 0,
  existing: 0,
  current: null,
  total: null,
  done: false,
  error: null,
};

/** Per-layer statuses the dialog counts or shows as the upload bar, not as lines. */
const LAYER_STATUSES = new Set([
  'Preparing',
  'Waiting',
  'Pushing',
  'Pushed',
  'Layer already exists',
]);

/** Folds one push event into the progress shown to the user. */
export function reducePush(state: PushState, event: PullEvent): PushState {
  switch (event.type) {
    case 'done':
      return { ...state, done: true, current: null, total: null };
    case 'error':
      return { ...state, error: toAppError(event.error) };
    case 'progress':
      switch (event.status) {
        case 'Pushing':
          return { ...state, current: event.current, total: event.total };
        case 'Pushed':
          return { ...state, pushed: state.pushed + 1, current: null, total: null };
        case 'Layer already exists':
          return { ...state, existing: state.existing + 1 };
        default:
          if (LAYER_STATUSES.has(event.status) || !event.status) return state;
          return { ...state, messages: [...state.messages, event.status] };
      }
  }
}

/** "3 layers pushed, 2 already in the registry". */
export function describePush(state: PushState): string {
  return state.existing > 0
    ? t('images.push.pushedAndExisting', { pushed: state.pushed, existing: state.existing })
    : t('images.push.pushed', { pushed: state.pushed });
}
