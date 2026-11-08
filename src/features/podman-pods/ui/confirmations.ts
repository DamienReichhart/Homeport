import type { PodmanPod } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { isUp, members } from '../model/pods';
import { t } from '@/shared/i18n';

/**
 * Removing a pod, saying how many containers go with it and that running
 * ones are stopped first.
 */
export function removePodConfirmation(
  pod: PodmanPod,
  action: (force: boolean) => Promise<unknown>,
): ConfirmedAction {
  const running = isUp(pod.status) || pod.status === 'paused';
  const count = members(pod).length;
  return {
    title: t('podmanpods.remove.title', { name: pod.name }),
    description: running
      ? t('podmanpods.remove.running', { count })
      : t('podmanpods.remove.stopped', { count }),
    confirmLabel: running ? t('podmanpods.remove.stopAndRemove') : t('common.remove'),
    destructive: true,
    action: () => action(running),
  };
}
