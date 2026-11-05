import { CircleArrowUpIcon } from 'lucide-react';
import { Badge, Tooltip, TooltipContent, TooltipTrigger } from '@/shared/ui';
import { newerServicesLabel, type ServiceFreshness } from '../model/freshness';
import { t } from '@/shared/i18n';

/** "Update available", naming the services with a newer image on hover and to screen readers. */
export function UpdateBadge({ newer }: { newer: ServiceFreshness[] }) {
  const label = newerServicesLabel(newer);
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        <Badge variant="secondary" tabIndex={0} onClick={(e) => e.stopPropagation()}>
          <CircleArrowUpIcon /> {t('stacks.update.available')}
          <span className="sr-only">: {label}</span>
        </Badge>
      </TooltipTrigger>
      <TooltipContent>{label}</TooltipContent>
    </Tooltip>
  );
}
