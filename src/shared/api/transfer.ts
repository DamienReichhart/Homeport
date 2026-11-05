import { toAppError, type AppError } from './errors';
import type { TransferEvent } from './bindings';

/** A file transfer (save, load, import, export) as its progress dialog shows it. */
export interface TransferState {
  bytes: number;
  total: number | null;
  /** The Engine's lines, such as "Loaded image: nginx:1.27". */
  lines: string[];
  /** The names saved, loaded or imported, once done. */
  images: string[];
  done: boolean;
  error: AppError | null;
}

export const initialTransfer: TransferState = {
  bytes: 0,
  total: null,
  lines: [],
  images: [],
  done: false,
  error: null,
};

export function reduceTransfer(state: TransferState, event: TransferEvent): TransferState {
  switch (event.type) {
    case 'progress':
      return { ...state, bytes: event.bytes, total: event.total ?? state.total };
    case 'line':
      return { ...state, lines: [...state.lines, event.text] };
    case 'done':
      return { ...state, done: true, images: event.images };
    case 'error':
      return { ...state, error: toAppError(event.error) };
  }
}
