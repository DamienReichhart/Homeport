import {
  CirclePlayIcon,
  CircleStopIcon,
  EllipsisVerticalIcon,
  PauseIcon,
  RotateCwIcon,
  Trash2Icon,
} from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import type { PodmanPod, PodmanPodAction } from '@/shared/api';
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
  type ConfirmedAction,
} from '@/shared/ui';
import { usePodAction, useRemovePod } from '../api/mutations';
import { podActions } from '../model/pods';
import { removePodConfirmation } from './confirmations';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<PodmanPodAction, { icon: LucideIcon; label: MessageKey; one: MessageKey }> = {
  start: {
    icon: CirclePlayIcon,
    label: 'podmanpods.action.start',
    one: 'podmanpods.actionOne.start',
  },
  stop: { icon: CircleStopIcon, label: 'podmanpods.action.stop', one: 'podmanpods.actionOne.stop' },
  restart: {
    icon: RotateCwIcon,
    label: 'podmanpods.action.restart',
    one: 'podmanpods.actionOne.restart',
  },
  pause: { icon: PauseIcon, label: 'podmanpods.action.pause', one: 'podmanpods.actionOne.pause' },
  unpause: {
    icon: CirclePlayIcon,
    label: 'podmanpods.action.unpause',
    one: 'podmanpods.actionOne.unpause',
  },
};

/**
 * A pod's lifecycle actions and Remove. The main one (Stop, Start or
 * Resume) is at hand, with Restart beside it on the pod's page; the rest
 * is in the ⋯ menu. A list row shows the main one as an icon.
 */
export function PodActions({
  hostId,
  pod,
  compact = false,
  confirm,
  onRemoved,
}: {
  hostId: string;
  pod: PodmanPod;
  compact?: boolean;
  confirm: (request: ConfirmedAction) => void;
  onRemoved?: () => void;
}) {
  const act = usePodAction(hostId);
  const remove = useRemovePod(hostId);
  const all = podActions(pod.status);
  const shown = compact ? all.slice(0, 1) : all.filter((a, i) => i === 0 || a === 'restart');
  const rest = all.filter((a) => !shown.includes(a));
  return (
    <div
      className={compact ? 'flex items-center justify-end gap-0.5' : 'flex items-center gap-2'}
      onClick={(e) => e.stopPropagation()}
    >
      {shown.map((action) => {
        const { icon: Icon, label, one } = ACTIONS[action];
        return compact ? (
          <Tooltip key={action}>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="icon-sm"
                aria-label={t(one, { name: pod.name })}
                disabled={act.isPending}
                onClick={() => act.mutate({ pod, action })}
              >
                <Icon />
              </Button>
            </TooltipTrigger>
            <TooltipContent>{t(label)}</TooltipContent>
          </Tooltip>
        ) : (
          <Button
            key={action}
            variant="outline"
            disabled={act.isPending}
            onClick={() => act.mutate({ pod, action })}
          >
            <Icon /> {t(label)}
          </Button>
        );
      })}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant={compact ? 'ghost' : 'outline'}
            size={compact ? 'icon-sm' : 'icon'}
            aria-label={
              compact ? t('podmanpods.actionsFor', { name: pod.name }) : t('common.moreActions')
            }
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {rest.map((action) => {
            const { icon: Icon, label } = ACTIONS[action];
            return (
              <DropdownMenuItem
                key={action}
                disabled={act.isPending}
                onSelect={() => act.mutate({ pod, action })}
              >
                <Icon /> {t(label)}
              </DropdownMenuItem>
            );
          })}
          {rest.length > 0 && <DropdownMenuSeparator />}
          <DropdownMenuItem
            variant="destructive"
            onSelect={() =>
              confirm(
                removePodConfirmation(pod, async (force) => {
                  await remove.mutateAsync({ pod, force });
                  onRemoved?.();
                }),
              )
            }
          >
            <Trash2Icon /> {t('common.remove')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
