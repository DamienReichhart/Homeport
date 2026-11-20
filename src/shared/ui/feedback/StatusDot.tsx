import { usePrefersReducedMotion } from '@/shared/hooks';
import { cn } from '@/shared/lib/utils';

export type Tone = 'success' | 'warning' | 'danger' | 'muted' | 'info';

/*
 * A status is a shape as well as a colour, so it reads in greyscale and for
 * colour-blind eyes (docs/BRAND.md). Running is a filled disc in a muted
 * green, stopped a hollow grey ring: the two read apart at a glance down a
 * long list. Only what needs you is red.
 *
 *   success  ● muted green disc  danger   ● Signal red disc
 *   muted    ○ grey ring         warning  ◆ amber diamond
 *   info     ○ ink ring
 */
const TONES: Record<Tone, string> = {
  success: 'size-2 rounded-full bg-success',
  warning: 'size-[7px] rotate-45 rounded-[1px] bg-warning',
  danger: 'size-2 rounded-full bg-destructive',
  muted: 'size-2 rounded-full border-[1.5px] border-muted-foreground',
  info: 'size-2 rounded-full border-[1.5px] border-foreground',
};

/** `pulse` animates "connecting", unless the OS asks for reduced motion: the shape says it. */
export function StatusDot({ tone, pulse }: { tone: Tone; pulse?: boolean }) {
  const still = usePrefersReducedMotion();
  return (
    <span
      aria-hidden
      data-tone={tone}
      className={cn('inline-block shrink-0', TONES[tone], pulse && !still && 'animate-pulse')}
    />
  );
}
