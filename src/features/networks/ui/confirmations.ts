import type { NetworkView } from '@/shared/api';
import type { ConfirmedAction } from '@/shared/ui';

/** Removing a network, warning about the containers still attached. */
export function removeNetworkConfirmation(
  network: NetworkView,
  action: () => Promise<unknown>,
): ConfirmedAction {
  return {
    title: `Remove network ${network.name}?`,
    description: network.containers.length
      ? `It still has ${network.containers.length} attached container(s); disconnect them first.`
      : 'No container uses it. Containers that reference it by name will fail to start.',
    confirmLabel: 'Remove',
    destructive: true,
    action,
  };
}

/** Removing every custom network no container is attached to. */
export function pruneNetworksConfirmation(action: () => Promise<unknown>): ConfirmedAction {
  return {
    title: 'Remove unused networks?',
    description: 'Every custom network no container is attached to is removed.',
    confirmLabel: 'Clean up',
    destructive: true,
    action,
  };
}
