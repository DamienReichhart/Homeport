import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import {
  commands,
  subscribe,
  type AppError,
  type BuildEvent,
  type BuildSpec,
  type PullEvent,
  type TransferEvent,
} from '@/shared/api';

/**
 * Pulls images onto a Host and streams their progress. A finished pull is
 * announced and refreshes the image list. The returned function starts a
 * pull and returns its canceller.
 */
export function usePullImage(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (reference: string, onEvent: (event: PullEvent) => void, onError: (error: AppError) => void) =>
      subscribe<PullEvent>(
        (ch) => commands.imagesPullSubscribe(hostId, reference, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') {
            toast.success(`Pulled ${reference}`);
            void client.invalidateQueries({ queryKey: hostKeys.images(hostId) });
          }
        },
        onError,
      ),
    [client, hostId],
  );
}

/**
 * Pushes an image name to its registry and streams the progress. A finished
 * push is announced. The returned function starts a push and returns its canceller.
 */
export function usePushImage(hostId: string) {
  return useCallback(
    (reference: string, onEvent: (event: PullEvent) => void, onError: (error: AppError) => void) =>
      subscribe<PullEvent>(
        (ch) => commands.imagesPushSubscribe(hostId, reference, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') toast.success(`Pushed ${reference}`);
        },
        onError,
      ),
    [hostId],
  );
}

/**
 * Builds an image on a Host from a folder on this machine and streams the
 * output. A finished build is announced and refreshes the image list.
 */
export function useBuildImage(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (spec: BuildSpec, onEvent: (event: BuildEvent) => void, onError: (error: AppError) => void) =>
      subscribe<BuildEvent>(
        (ch) => commands.imagesBuildSubscribe(hostId, spec, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') {
            toast.success(event.tags.length ? `Built ${event.tags[0]}` : 'Built the image');
            void client.invalidateQueries({ queryKey: hostKeys.images(hostId) });
          }
        },
        onError,
      ),
    [client, hostId],
  );
}

/** Saves images into one tar on this machine; returns the stream's canceller. */
export function useSaveImages(hostId: string) {
  return useCallback(
    (
      names: string[],
      destination: string,
      onEvent: (event: TransferEvent) => void,
      onError: (error: AppError) => void,
    ) =>
      subscribe<TransferEvent>(
        (ch) => commands.imagesSaveSubscribe(hostId, names, destination, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') toast.success(`Saved ${destination}`);
        },
        onError,
      ),
    [hostId],
  );
}

/** Loads the images in a tar on this machine into a Host; refreshes the list when done. */
export function useLoadImages(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (source: string, onEvent: (event: TransferEvent) => void, onError: (error: AppError) => void) =>
      subscribe<TransferEvent>(
        (ch) => commands.imagesLoadSubscribe(hostId, source, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') {
            toast.success(
              event.images.length === 1
                ? `Loaded ${event.images[0]}`
                : `Loaded ${event.images.length} images`,
            );
            void client.invalidateQueries({ queryKey: hostKeys.images(hostId) });
          }
        },
        onError,
      ),
    [client, hostId],
  );
}
