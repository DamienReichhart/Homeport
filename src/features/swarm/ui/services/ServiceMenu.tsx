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
} from '@/shared/ui';
import { canScale } from '../../model/services';
import { t } from '@/shared/i18n';

export interface ServiceActions {
  onLogs: (service: ServiceView) => void;
  onScale: (service: ServiceView) => void;
  onRollback: (service: ServiceView) => void;
  onRemove: (service: ServiceView) => void;
}

/** A service's actions: logs, scale (replicated only), roll back, remove. */
export function ServiceMenu({ service, ...actions }: { service: ServiceView } & ServiceActions) {
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={t('swarm.services.actionsFor', { name: service.name })}
          onClick={(e) => e.stopPropagation()}
        >
          <EllipsisVerticalIcon />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem onSelect={() => actions.onLogs(service)}>
          <ScrollTextIcon /> {t('swarm.services.action.logs')}
        </DropdownMenuItem>
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
  );
}
