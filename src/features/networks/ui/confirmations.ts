import type { NetworkView } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Removing a network, warning about the containers still attached. */
export function removeNetworkConfirmation(
  network: NetworkView,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: t('networks.remove.title', { name: network.name }),
    description: network.containers.length
      ? t('networks.remove.attached', { count: network.containers.length })
      : t('networks.remove.unused'),
    confirmLabel: t('common.remove'),
    destructive: true,
    action,
  };
}

/** Removing every custom network no container is attached to. */
export function pruneNetworksConfirmation(action: () => Promise<unknown>): ConfirmedAction {
  return {
    title: t('networks.prune.title'),
    description: t('networks.prune.description'),
    confirmLabel: t('networks.cleanUp'),
    destructive: true,
    action,
  };
}
