import type { NomadNodeView } from '@/shared/api';
import type { Tone } from '@/shared/ui';

/** A node's status as a colour: ready and taking work, ready but not, or down. */
export function nodeTone(node: NomadNodeView): Tone {
  switch (node.status) {
    case 'ready':
      return node.draining || !node.eligible ? 'warning' : 'success';
    case 'initializing':
      return 'info';
    case 'down':
    case 'disconnected':
      return 'danger';
    default:
      return 'muted';
  }
}

/**
 * Seconds in a duration as Nomad's CLI takes it: `1h`, `30m`, `90s`,
 * `1h30m`… `null` unless a positive whole duration of at most a year.
 */
export function parseDuration(text: string): number | null {
  const match = /^(?:(\d+)h)?(?:(\d+)m)?(?:(\d+)s)?$/.exec(text.trim());
  if (!match || !text.trim()) return null;
  const [, h = '0', m = '0', s = '0'] = match;
  const secs = Number(h) * 3600 + Number(m) * 60 + Number(s);
  return secs > 0 && secs <= 366 * 24 * 3600 ? secs : null;
}
