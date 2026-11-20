import { CircleArrowUpIcon } from 'lucide-react';
import { newerLabel } from '@/core/host';
import type { Image } from '@/shared/api';
import { Badge, Tooltip, TooltipContent, TooltipTrigger } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * An image's state: amber when a newer version of one of its tags (`newer`)
 * was published, which is worth acting on; then a quiet count of the
 * containers using it, or an "Unused" mark, which is what a clean-up removes.
 */
export function ImageStatus({ image, newer }: { image: Image; newer: string[] }) {
  return (
    <span className="flex items-center gap-2">
      {newer.length > 0 && (
        <Tooltip>
          <TooltipTrigger asChild>
            <Badge variant="warning" tabIndex={0} onClick={(e) => e.stopPropagation()}>
              <CircleArrowUpIcon /> {t('images.newer')}
            </Badge>
          </TooltipTrigger>
          <TooltipContent>{newerLabel(newer)}</TooltipContent>
        </Tooltip>
      )}
      {image.containers > 0 ? (
        <span className="text-muted-foreground">
          {t('images.inUse', { count: image.containers })}
        </span>
      ) : (
        <Badge variant="outline">{t('images.unused')}</Badge>
      )}
    </span>
  );
}
