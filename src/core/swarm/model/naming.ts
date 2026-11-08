import type { HostView, SwarmRole, SwarmView } from '@/shared/api';
import { t } from '@/shared/i18n';

/** The first characters of a swarm ID, as Docker shows IDs. */
export const shortSwarmId = (id: string) => id.slice(0, 12);

/**
 * A Swarm's name. Swarms have none of their own (`Spec.Name` is always
 * `default`), so it's named after its first manager Host.
 */
export function swarmLabel(swarm: SwarmView, hosts: HostView[] | undefined): string {
  const first = swarm.managers
    .map((id) => hosts?.find((h) => h.host.id === id)?.host.name)
    .find((name) => name !== undefined);
  return first
    ? t('core.swarm.label', { host: first })
    : t('core.swarm.labelUnknown', { id: shortSwarmId(swarm.id) });
}

/** The Swarm a Host belongs to among `swarms`, and its role there. */
export function swarmOfHost(
  swarms: SwarmView[] | undefined,
  hostId: string,
): { swarm: SwarmView; role: SwarmRole } | null {
  for (const swarm of swarms ?? []) {
    if (swarm.managers.includes(hostId)) return { swarm, role: 'manager' };
    if (swarm.workers.includes(hostId)) return { swarm, role: 'worker' };
  }
  return null;
}

/** The Swarm whose manager `hostId` is, from the Host's own status. */
export function managedSwarmId(hosts: HostView[] | undefined, hostId: string): string | null {
  const status = hosts?.find((h) => h.host.id === hostId)?.status;
  if (status?.state !== 'connected' || status.swarm?.role !== 'manager') return null;
  return status.swarm.swarmId;
}
