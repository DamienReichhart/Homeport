import type { ReactNode } from 'react';
import { Button, Tooltip, TooltipContent, TooltipTrigger } from '@/shared/ui';

/** A row's ghost icon button, named by `label` for screen readers and as a tooltip. */
export function RowButton({
  label,
  onClick,
  children,
}: {
  label: string;
  onClick: () => void;
  children: ReactNode;
}) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Button variant="ghost" size="icon-sm" aria-label={label} onClick={onClick}>
          {children}
        </Button>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}
