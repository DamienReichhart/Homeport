import { commands, subscribe, type AppError, type ImageCopy, type PullEvent } from '@/shared/api';

/** Copies an image from a remote server; returns what cancels following it. */
export function startImageCopy(
  hostId: string,
  project: string,
  copy: ImageCopy,
  onEvent: (event: PullEvent) => void,
  onError: (error: AppError) => void,
): () => void {
  return subscribe<PullEvent>(
    (ch) => commands.incusImageCopySubscribe(hostId, project, copy, ch),
    onEvent,
    onError,
  );
}
