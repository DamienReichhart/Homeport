import { cn } from '@/shared/lib/utils';

export type Tone = 'success' | 'warning' | 'danger' | 'muted' | 'info';

/*
 * A status is a shape as well as a colour, so it reads in greyscale and for
 * colour-blind eyes (docs/BRAND.md). Healthy is a plain ink disc: only what
 * needs attention is coloured, and only what needs you is red.
 *
 *   success  ● ink disc          danger   ● Signal red disc
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

export function StatusDot({ tone, pulse }: { tone: Tone; pulse?: boolean }) {
  return (
    <span
      aria-hidden
      data-tone={tone}
      className={cn('inline-block shrink-0', TONES[tone], pulse && 'animate-pulse')}
    />
  );
}
