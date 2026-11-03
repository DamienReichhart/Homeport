import { useId } from 'react';
import { cn } from '@/shared/lib/utils';

// The mark's fixed colours (assets/brand/mark.svg); they don't follow the theme.
const TILE_TOP = '#004c76';
const TILE_BOTTOM = '#091a34';
const BEACON = '#f8842d';
const TIDE = '#6eccec';

/** Homeport's mark: a container-house docked on the tide, on its harbour tile. */
export function LogoMark({ className, title }: { className?: string; title?: string }) {
  const gradient = useId();
  return (
    <svg
      viewBox="64 64 896 896"
      className={cn('size-6 shrink-0', className)}
      role={title ? 'img' : undefined}
      aria-label={title}
      aria-hidden={title ? undefined : true}
    >
      <defs>
        <linearGradient id={gradient} x1="0" y1="0" x2="0" y2="1">
          <stop offset="0" stopColor={TILE_TOP} />
          <stop offset="1" stopColor={TILE_BOTTOM} />
        </linearGradient>
      </defs>
      <rect x="64" y="64" width="896" height="896" rx="208" fill={`url(#${gradient})`} />
      <rect
        x="70"
        y="70"
        width="884"
        height="884"
        rx="202"
        fill="none"
        stroke="#fff"
        strokeOpacity=".12"
        strokeWidth="12"
      />
      <path
        d="M512 272L716 432V640H308V432Z"
        fill={BEACON}
        stroke={BEACON}
        strokeWidth="40"
        strokeLinejoin="round"
      />
      <path
        d="M416 480V592M480 480V592M544 480V592M608 480V592"
        stroke={TILE_BOTTOM}
        strokeWidth="28"
        strokeLinecap="round"
      />
      <path
        d="M300 742q53-44 106 0t106 0t106 0t106 0"
        fill="none"
        stroke={TIDE}
        strokeWidth="52"
        strokeLinecap="round"
      />
    </svg>
  );
}

/** The mark beside the name, as shown at the top of the sidebar. */
export function Logo({ className }: { className?: string }) {
  return (
    <span className={cn('inline-flex items-center gap-2.5', className)}>
      <LogoMark className="size-7" />
      <span className="text-[1.0625rem] font-semibold tracking-tight">Homeport</span>
    </span>
  );
}
