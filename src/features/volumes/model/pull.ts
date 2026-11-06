import type { PullEvent } from '@/shared/api';

/** Where a pull of the helper's image is, from its events. */
export interface PullState {
  /** Bytes downloaded and expected, by layer. */
  layers: Record<string, { current: number; total: number }>;
  done: boolean;
}

export const initialPull: PullState = { layers: {}, done: false };

export function reducePull(state: PullState, event: PullEvent): PullState {
  switch (event.type) {
    case 'progress':
      if (!event.id || event.total === null || event.total <= 0) return state;
      return {
        ...state,
        layers: {
          ...state.layers,
          [event.id]: { current: event.current ?? 0, total: event.total },
        },
      };
    case 'done':
      return { ...state, done: true };
    case 'error':
      return state;
  }
}

/** How far the pull is, from 0 to 100; `null` before any layer reports a size. */
export function pullPercent(state: PullState): number | null {
  if (state.done) return 100;
  const layers = Object.values(state.layers);
  const total = layers.reduce((sum, l) => sum + l.total, 0);
  if (total === 0) return null;
  const current = layers.reduce((sum, l) => sum + Math.min(l.current, l.total), 0);
  return Math.round((current / total) * 100);
}
