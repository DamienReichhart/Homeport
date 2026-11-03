import type { Tone } from '@/shared/ui';

/** The tone of an Engine container state (`running`, `exited`, …). */
export function containerTone(state: string, health?: string | null): Tone {
  if (health === 'unhealthy') return 'danger';
  switch (state) {
    case 'running':
      return health === 'starting' ? 'warning' : 'success';
    case 'paused':
    case 'restarting':
      return 'warning';
    case 'dead':
      return 'danger';
    default:
      return 'muted';
  }
}
