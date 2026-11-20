import {
  EllipsisVerticalIcon,
  ScaleIcon,
  ScrollTextIcon,
  Trash2Icon,
  Undo2Icon,
} from 'lucide-react';
import type { ServiceView } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/shared/ui';
import { canScale } from '../../model/services';
import { t } from '@/shared/i18n';

export interface ServiceActions {
  onLogs: (service: ServiceView) => void;
  onScale: (service: ServiceView) => void;
  onRollback: (service: ServiceView) => void;
  onRemove: (service: ServiceView) => void;
}

/**
 * A service row's actions: its logs at hand, then scale (replicated only),
 * roll back and remove in its menu.
 */
export function ServiceMenu({ service, ...actions }: { service: ServiceView } & ServiceActions) {
  return (
    <div className="flex items-center justify-end gap-0.5" onClick={(e) => e.stopPropagation()}>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('swarm.services.logsOf', { name: service.name })}
            onClick={() => actions.onLogs(service)}
          >
            <ScrollTextIcon />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t('swarm.services.action.logs')}</TooltipContent>
      </Tooltip>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('swarm.services.actionsFor', { name: service.name })}
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {canScale(service) && (
            <DropdownMenuItem onSelect={() => actions.onScale(service)}>
              <ScaleIcon /> {t('swarm.services.action.scale')}
            </DropdownMenuItem>
          )}
          <DropdownMenuItem onSelect={() => actions.onRollback(service)}>
            <Undo2Icon /> {t('swarm.services.action.rollback')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem variant="destructive" onSelect={() => actions.onRemove(service)}>
            <Trash2Icon /> {t('common.remove')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
