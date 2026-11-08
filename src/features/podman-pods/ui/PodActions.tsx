import { PauseIcon, PlayIcon, RotateCwIcon, SquareIcon, Trash2Icon } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import type { PodmanPod, PodmanPodAction } from '@/shared/api';
import { Button, type ConfirmedAction } from '@/shared/ui';
import { usePodAction, useRemovePod } from '../api/mutations';
import { podActions } from '../model/pods';
import { removePodConfirmation } from './confirmations';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<PodmanPodAction, { icon: LucideIcon; label: MessageKey; one: MessageKey }> = {
  start: { icon: PlayIcon, label: 'podmanpods.action.start', one: 'podmanpods.actionOne.start' },
  stop: { icon: SquareIcon, label: 'podmanpods.action.stop', one: 'podmanpods.actionOne.stop' },
  restart: {
    icon: RotateCwIcon,
    label: 'podmanpods.action.restart',
    one: 'podmanpods.actionOne.restart',
  },
  pause: { icon: PauseIcon, label: 'podmanpods.action.pause', one: 'podmanpods.actionOne.pause' },
  unpause: {
    icon: PlayIcon,
    label: 'podmanpods.action.unpause',
    one: 'podmanpods.actionOne.unpause',
  },
};

/**
 * A pod's lifecycle buttons and Remove: labelled in its page's header,
 * icons only in a list row.
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
  const actions = compact ? podActions(pod.status).slice(0, 1) : podActions(pod.status);
  return (
    <>
      {actions.map((action) => {
        const { icon: Icon, label, one } = ACTIONS[action];
        return (
          <Button
            key={action}
            variant={compact ? 'ghost' : 'outline'}
            size={compact ? 'icon-sm' : 'default'}
            aria-label={compact ? t(one, { name: pod.name }) : undefined}
            disabled={act.isPending}
            onClick={() => act.mutate({ pod, action })}
          >
            <Icon /> {!compact && t(label)}
          </Button>
        );
      })}
      <Button
        variant={compact ? 'ghost' : 'destructive'}
        size={compact ? 'icon-sm' : 'default'}
        aria-label={compact ? t('podmanpods.removeOne', { name: pod.name }) : undefined}
        onClick={() =>
          confirm(
            removePodConfirmation(pod, async (force) => {
              await remove.mutateAsync({ pod, force });
              onRemoved?.();
            }),
          )
        }
      >
        <Trash2Icon /> {!compact && t('common.remove')}
      </Button>
    </>
  );
}
