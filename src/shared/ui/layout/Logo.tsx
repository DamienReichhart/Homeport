import { cn } from '@/shared/lib/utils';

// The mark's fixed colours (assets/brand/mark.svg); they don't follow the theme.
const INK = '#141414';
const PAPER = '#f7f7f4';
const SIGNAL = '#e1001a';

/**
 * Homeport's mark, on its ink tile: a square (this machine), a line (the
 * connection) and a hollow ring (a Host, with nothing installed on it).
 */
export function LogoMark({ className, title }: { className?: string; title?: string }) {
  return (
    <svg
      viewBox="64 64 896 896"
      className={cn('size-6 shrink-0', className)}
      role={title ? 'img' : undefined}
      aria-label={title}
      aria-hidden={title ? undefined : true}
    >
      <rect x="64" y="64" width="896" height="896" rx="192" fill={INK} />
      <rect x="162" y="412" width="200" height="200" fill={PAPER} />
      <rect x="356" y="476" width="240" height="72" fill={PAPER} />
      <circle cx="726" cy="512" r="100" fill="none" stroke={SIGNAL} strokeWidth="72" />
    </svg>
  );
}

/**
 * The tile-less glyph beside the name, as shown at the top of the sidebar: the
 * square and the line take the text colour, so it follows the theme.
 */
export function Logo({ className }: { className?: string }) {
  return (
    <span className={cn('inline-flex items-center gap-2.5', className)}>
      <svg viewBox="162 376 700 272" className="h-4 w-auto shrink-0" aria-hidden>
        <rect x="162" y="412" width="200" height="200" fill="currentColor" />
        <rect x="356" y="476" width="240" height="72" fill="currentColor" />
        <circle cx="726" cy="512" r="100" fill="none" stroke={SIGNAL} strokeWidth="72" />
      </svg>
      <span className="text-[1.0625rem] font-semibold tracking-[-0.01em]">Homeport</span>
    </span>
  );
}
