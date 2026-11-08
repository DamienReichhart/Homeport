import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import type { ImagePuller } from '@/features/images';
import { commands, subscribe, type PullEvent } from '@/shared/api';
import { t } from '@/shared/i18n';

/** Pulls into a containerd namespace through containerd's Transfer service. */
export function useContainerdPull(hostId: string, namespace: string): ImagePuller {
  const client = useQueryClient();
  return useCallback<ImagePuller>(
    (reference, onEvent, onError) =>
      subscribe<PullEvent>(
        (ch) => commands.containerdImagesPullSubscribe(hostId, namespace, reference, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'done') {
            toast.success(t('containerd.images.pulled', { reference, namespace }));
            void client.invalidateQueries({ queryKey: hostKeys.containerdImages(hostId) });
            void client.invalidateQueries({ queryKey: hostKeys.containerdNamespaces(hostId) });
          }
        },
        onError,
      ),
    [client, hostId, namespace],
  );
}
