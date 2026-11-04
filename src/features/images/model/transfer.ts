import { toAppError, type AppError, type Image, type TransferEvent } from '@/shared/api';

/** A save or load as the progress dialog shows it. */
export interface TransferState {
  bytes: number;
  total: number | null;
  /** The Engine's lines, such as "Loaded image: nginx:1.27". */
  lines: string[];
  /** The names saved or loaded, once done. */
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

/** The names `docker save` should write for a selection: tags, or IDs for untagged images. */
export const saveNames = (images: Image[]) =>
  images.flatMap((i) => (i.tags.length > 0 ? i.tags : [i.id]));

/** `nginx-1.27.tar` for one image, `images.tar` for several. */
export function archiveName(images: Image[]): string {
  const [only] = images;
  if (images.length !== 1 || !only) return 'images.tar';
  const name = only.tags[0] ?? only.id.replace(/^sha256:/, '').slice(0, 12);
  return `${name.replace(/[/:@\\]+/g, '-')}.tar`;
}

/** Archive files the Engine loads: plain and compressed tars. */
export const LOAD_FILTERS = [{ name: 'Image archives', extensions: ['tar', 'gz', 'tgz'] }];
/** What `docker save` writes. */
export const SAVE_FILTERS = [{ name: 'Image archive', extensions: ['tar'] }];
