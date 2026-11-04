import { CircleArrowUpIcon } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/shared/ui';

/**
 * The "newer image" mark: a small icon whose tooltip says what was
 * published, e.g. "A newer nginx:1.27 was published".
 */
export function NewerImage({ label }: { label: string }) {
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          role="img"
          aria-label={label}
          className="text-foreground inline-flex shrink-0 items-center"
          onClick={(e) => e.stopPropagation()}
        >
          <CircleArrowUpIcon className="size-4" />
        </span>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}
