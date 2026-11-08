import type { PodHealth, WorkloadHealth } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

const WORKLOAD: Record<WorkloadHealth, { tone: Tone; label: MessageKey }> = {
  ready: { tone: 'success', label: 'core.health.ready' },
  progressing: { tone: 'info', label: 'core.health.progressing' },
  degraded: { tone: 'danger', label: 'core.health.degraded' },
  failed: { tone: 'danger', label: 'core.health.failed' },
  paused: { tone: 'muted', label: 'core.health.paused' },
};

const POD: Record<PodHealth, Tone> = {
  running: 'success',
  succeeded: 'muted',
  pending: 'info',
  failed: 'danger',
  crashing: 'danger',
  terminating: 'warning',
  unknown: 'muted',
};

export function workloadHealth(health: WorkloadHealth): { tone: Tone; label: string } {
  const { tone, label } = WORKLOAD[health];
  return { tone, label: t(label) };
}

export const podTone = (health: PodHealth): Tone => POD[health];

/** How long ago an RFC 3339 time was, as `kubectl` shows ages: 5s, 3m, 2h, 4d. */
export function age(rfc3339: string | null, now = Date.now()): string {
  if (!rfc3339) return '—';
  const secs = Math.max(0, Math.floor((now - Date.parse(rfc3339)) / 1000));
  if (secs < 120) return `${secs}s`;
  const mins = Math.floor(secs / 60);
  if (mins < 120) return `${mins}m`;
  const hours = Math.floor(mins / 60);
  if (hours < 48) return `${hours}h`;
  return `${Math.floor(hours / 24)}d`;
}
