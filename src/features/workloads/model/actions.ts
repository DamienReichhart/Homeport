import type { WorkloadKind } from '@/shared/api';

/** What can be done to a workload of `kind`, besides deleting it. */
export function workloadActions(kind: WorkloadKind) {
  return {
    scale: kind === 'deployment' || kind === 'statefulSet',
    restart: kind === 'deployment' || kind === 'statefulSet' || kind === 'daemonSet',
    cron: kind === 'cronJob',
  };
}

/** The replicas typed in the Scale dialog, or `null` while it isn't a whole number from 0. */
export function parseReplicas(text: string): number | null {
  const trimmed = text.trim();
  if (!/^\d+$/.test(trimmed)) return null;
  const value = Number(trimmed);
  return Number.isSafeInteger(value) && value <= 2_147_483_647 ? value : null;
}
