import { cn } from '@/shared/lib/utils';

export type Tone = 'success' | 'warning' | 'danger' | 'muted' | 'info';

const TONES: Record<Tone, string> = {
  success: 'bg-success',
  warning: 'bg-warning',
  danger: 'bg-destructive',
  muted: 'bg-muted-foreground/50',
  info: 'bg-info',
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
