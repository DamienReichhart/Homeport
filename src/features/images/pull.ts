import type { PullEvent } from '@/shared/api';

export interface LayerProgress {
  id: string;
  status: string;
  current: number | null;
  total: number | null;
}

export interface PullState {
  layers: LayerProgress[];
  /** Status lines not tied to a layer ("Pulling from library/nginx", digests). */
  messages: string[];
  done: boolean;
  error: string | null;
}

export const initialPull: PullState = { layers: [], messages: [], done: false, error: null };

/** Folds one pull event into the progress shown to the user. */
export function reducePull(state: PullState, event: PullEvent): PullState {
  switch (event.type) {
    case 'done':
      return { ...state, done: true };
    case 'error':
      return { ...state, error: event.message };
    case 'progress': {
      // Image-level lines carry the reference as their id; layers are short hex ids.
      if (!event.id || !/^[0-9a-f]{12}$/.test(event.id)) {
        return {
          ...state,
          messages: [...state.messages, event.id ? `${event.id}: ${event.status}` : event.status],
        };
      }
      const layer = {
        id: event.id,
        status: event.status,
        current: event.current,
        total: event.total,
      };
      const exists = state.layers.some((l) => l.id === event.id);
      return {
        ...state,
        layers: exists
          ? state.layers.map((l) => (l.id === event.id ? layer : l))
          : [...state.layers, layer],
      };
    }
  }
}
