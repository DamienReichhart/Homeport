import { objectPath } from '@/core/cluster';
import type { WorkloadKind } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** Each workload kind, in the order the page lists them, with its URL segment. */
export const WORKLOAD_KINDS: { kind: WorkloadKind; path: string; label: MessageKey }[] = [
  { kind: 'deployment', path: 'deployments', label: 'workloads.kind.deployment' },
  { kind: 'statefulSet', path: 'statefulsets', label: 'workloads.kind.statefulSet' },
  { kind: 'daemonSet', path: 'daemonsets', label: 'workloads.kind.daemonSet' },
  { kind: 'job', path: 'jobs', label: 'workloads.kind.job' },
  { kind: 'cronJob', path: 'cronjobs', label: 'workloads.kind.cronJob' },
];

/** The kind a URL segment names, Deployments by default. */
export function kindOf(path: string | undefined): WorkloadKind {
  return WORKLOAD_KINDS.find((k) => k.path === path)?.kind ?? 'deployment';
}

export function pathOf(kind: WorkloadKind): string {
  return WORKLOAD_KINDS.find((k) => k.kind === kind)?.path ?? 'deployments';
}

/** The page of one workload. */
export const workloadPath = (kind: WorkloadKind, namespace: string, name: string) =>
  objectPath(kind, namespace, name) ?? `/workloads/${pathOf(kind)}`;

/** `3/3` for replicas; a CronJob, which wants none, shows its active Jobs. */
export function readyText(ready: number, desired: number, kind: WorkloadKind): string {
  return kind === 'cronJob' ? String(ready) : `${ready}/${desired}`;
}
