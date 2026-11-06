import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import {
  commands,
  subscribe,
  type AppError,
  type RestoreSpec,
  type TransferEvent,
} from '@/shared/api';
import { t } from '@/shared/i18n';

type OnEvent = (event: TransferEvent) => void;
type OnError = (error: AppError) => void;

/** Backs a volume up to a `.tar` on this machine; returns the stream's canceller. */
export function useBackupVolume(hostId: string) {
  return useCallback(
    (
      volume: string,
      destination: string,
      pulledImage: boolean,
      onEvent: OnEvent,
      onError: OnError,
    ) =>
      subscribe<TransferEvent>(
        (ch) => commands.volumesBackupSubscribe(hostId, volume, destination, pulledImage, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done')
            toast.success(t('volumes.backup.done', { volume, path: destination }));
        },
        onError,
      ),
    [hostId],
  );
}

/** Restores a volume from a `.tar` on this machine; refreshes the list when done. */
export function useRestoreVolume(hostId: string) {
  const client = useQueryClient();
  return useCallback(
    (source: string, spec: RestoreSpec, pulledImage: boolean, onEvent: OnEvent, onError: OnError) =>
      subscribe<TransferEvent>(
        (ch) => commands.volumesRestoreSubscribe(hostId, source, spec, pulledImage, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done')
            toast.success(t('volumes.restore.done', { volume: spec.volume }));
          if (event.type === 'done' || event.type === 'error') {
            void client.invalidateQueries({ queryKey: hostKeys.volumes(hostId) });
            void client.invalidateQueries({ queryKey: hostKeys.volumeSizes(hostId) });
          }
        },
        onError,
      ),
    [client, hostId],
  );
}
