import { openUrl } from '@tauri-apps/plugin-opener';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { commands, type ForwardTarget } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { forwardKeys } from './queries';
import { t } from '@/shared/i18n';

/** Starts forwarding a local port to a Pod's or Service's port. */
export function useStartForward(clusterId: string, onStarted?: () => void) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (request: {
      target: ForwardTarget;
      remotePort: number;
      localPort: number | null;
    }) =>
      commands.clustersForwardStart(
        clusterId,
        request.target,
        request.remotePort,
        request.localPort,
      ),
    onSuccess: async (forward) => {
      const url = `http://127.0.0.1:${forward.localPort}`;
      toast.success(
        t('portforwards.started', {
          address: `127.0.0.1:${forward.localPort}`,
          target: forward.target.name,
          port: forward.remotePort,
        }),
        { action: { label: t('portforwards.open'), onClick: () => void openUrl(url) } },
      );
      await client.invalidateQueries({ queryKey: forwardKeys.all });
      onStarted?.();
    },
  });
}

export function useStopForward() {
  const client = useQueryClient();
  return useMutation({
    mutationFn: (id: number) => commands.clustersForwardStop(id),
    onSuccess: () => client.invalidateQueries({ queryKey: forwardKeys.all }),
    onError: (e) => toastError(e, t('portforwards.stopFailed')),
  });
}
