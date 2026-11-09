import type { Tone } from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';

/** A job's status as a colour: stopped jobs are muted, dead ones too, pending ones informative. */
export function jobTone(status: string, stopped: boolean): Tone {
  if (stopped) return 'muted';
  switch (status) {
    case 'running':
      return 'success';
    case 'pending':
      return 'info';
    default:
      return 'muted';
  }
}

/** An allocation's client status as a colour. */
export function allocationTone(clientStatus: string): Tone {
  switch (clientStatus) {
    case 'running':
      return 'success';
    case 'pending':
      return 'info';
    case 'complete':
      return 'muted';
    case 'failed':
    case 'lost':
      return 'danger';
    default:
      return 'warning';
  }
}

const STATUS_LABELS: Partial<Record<string, MessageKey>> = {
  running: 'nomad.status.running',
  pending: 'nomad.status.pending',
  dead: 'nomad.status.dead',
  complete: 'nomad.status.complete',
  failed: 'nomad.status.failed',
  lost: 'nomad.status.lost',
  unknown: 'nomad.status.unknown',
  stopped: 'nomad.status.stopped',
};

/** The catalogue key of a Nomad status, or `null` for one Homeport doesn't know. */
export function statusLabel(status: string): MessageKey | null {
  return STATUS_LABELS[status] ?? null;
}

/** The first eight characters of an allocation or node ID, as Nomad's UI and CLI show them. */
export const shortId = (id: string) => id.slice(0, 8);
