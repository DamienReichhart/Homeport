import { cn } from '@/shared/lib/utils';

export type Tone = 'success' | 'warning' | 'danger' | 'muted' | 'info';

const TONES: Record<Tone, string> = {
  success: 'bg-emerald-500',
  warning: 'bg-amber-500',
  danger: 'bg-red-500',
  muted: 'bg-muted-foreground/50',
  info: 'bg-sky-500',
};

export function StatusDot({ tone, pulse }: { tone: Tone; pulse?: boolean }) {
  return (
    <span
      aria-hidden
      className={cn(
        'inline-block size-2 shrink-0 rounded-full',
        TONES[tone],
        pulse && 'animate-pulse',
      )}
    />
  );
}

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
