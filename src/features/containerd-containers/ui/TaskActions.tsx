import {
  CirclePlayIcon,
  CircleStopIcon,
  EllipsisVerticalIcon,
  PauseIcon,
  Trash2Icon,
  ZapIcon,
} from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import type { ContainerdContainer, TaskAction } from '@/shared/api';
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
import { useTaskAction } from '../api/mutations';
import { taskActions } from '../model/containers';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<TaskAction, { icon: LucideIcon; label: MessageKey; one: MessageKey }> = {
  start: {
    icon: CirclePlayIcon,
    label: 'containerd.containers.action.start',
    one: 'containerd.containers.actionOne.start',
  },
  stop: {
    icon: CircleStopIcon,
    label: 'containerd.containers.action.stop',
    one: 'containerd.containers.actionOne.stop',
  },
  pause: {
    icon: PauseIcon,
    label: 'containerd.containers.action.pause',
    one: 'containerd.containers.actionOne.pause',
  },
  resume: {
    icon: CirclePlayIcon,
    label: 'containerd.containers.action.resume',
    one: 'containerd.containers.actionOne.resume',
  },
};

/**
 * A container's task actions. The main one (stop, start or resume) is at
 * hand, an outline button on its page or an icon in a list row; the other
 * task action, Kill and Remove are in its menu.
 */
export function TaskActions({
  hostId,
  namespace,
  container,
  compact = false,
  onKill,
  onRemove,
}: {
  hostId: string;
  namespace: string;
  container: ContainerdContainer;
  compact?: boolean;
  onKill?: () => void;
  onRemove: () => void;
}) {
  const act = useTaskAction(hostId, namespace);
  const [main, ...others] = taskActions(container.state);
  const mainAction = main ? ACTIONS[main] : null;
  return (
    <div className="flex items-center justify-end gap-2" onClick={(e) => e.stopPropagation()}>
      {main &&
        mainAction &&
        (compact ? (
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t(mainAction.one, { name: container.name })}
                disabled={act.isPending}
                onClick={() => act.mutate({ container, action: main })}
              >
                <mainAction.icon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t(mainAction.label)}</TooltipContent>
          </Tooltip>
        ) : (
          <Button
            variant="outline"
            disabled={act.isPending}
            onClick={() => act.mutate({ container, action: main })}
          >
            <mainAction.icon /> {t(mainAction.label)}
          </Button>
        ))}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant={compact ? 'ghost' : 'outline'}
            size={compact ? 'icon-sm' : 'icon'}
            aria-label={
              compact
                ? t('containerd.containers.actionsFor', { name: container.name })
                : t('common.moreActions')
            }
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {others.map((action) => {
            const { icon: Icon, label } = ACTIONS[action];
            return (
              <DropdownMenuItem
                key={action}
                disabled={act.isPending}
                onSelect={() => act.mutate({ container, action })}
              >
                <Icon /> {t(label)}
              </DropdownMenuItem>
            );
          })}
          {onKill && container.state === 'running' && (
            <DropdownMenuItem onSelect={onKill}>
              <ZapIcon /> {t('containerd.containers.action.kill')}
            </DropdownMenuItem>
          )}
          {(others.length > 0 || (onKill && container.state === 'running')) && (
            <DropdownMenuSeparator />
          )}
          <DropdownMenuItem variant="destructive" onSelect={onRemove}>
            <Trash2Icon /> {t('common.remove')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
