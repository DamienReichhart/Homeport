import type { ReactElement } from 'react';
import { formatKeys, isMac } from '@/shared/hooks';
import { cn } from '@/shared/lib';
import { Tooltip, TooltipContent, TooltipTrigger } from '../primitives/tooltip';

/** A shortcut's keys as `<kbd>`s, with ⌘ on macOS and Ctrl elsewhere. */
export function Keys({ keys, className }: { keys: string; className?: string }) {
  return (
    <span className={cn('inline-flex items-center gap-0.5', className)}>
      {formatKeys(keys, isMac()).map((key, i) => (
        <kbd
          key={i}
          className="bg-muted text-muted-foreground border-border min-w-5 rounded border px-1 text-center font-mono text-[11px] leading-4"
        >
          {key}
        </kbd>
      ))}
    </span>
  );
}

/** Shows what `children` does and its shortcut on hover and focus. */
export function WithShortcut({
  keys,
  label,
  side,
  children,
}: {
  keys: string;
  label: string;
  side?: 'top' | 'right' | 'bottom' | 'left';
  children: ReactElement;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>{children}</TooltipTrigger>
      <TooltipContent side={side} className="flex items-center gap-2">
        {label}
        <Keys keys={keys} />
      </TooltipContent>
    </Tooltip>
  );
}
