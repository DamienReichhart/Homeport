import { PauseIcon, PlayIcon, SquareIcon, Trash2Icon, ZapIcon } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import type { ContainerdContainer, TaskAction } from '@/shared/api';
import { Button } from '@/shared/ui';
import { useTaskAction } from '../api/mutations';
import { taskActions } from '../model/containers';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<TaskAction, { icon: LucideIcon; label: MessageKey; one: MessageKey }> = {
  start: {
    icon: PlayIcon,
    label: 'containerd.containers.action.start',
    one: 'containerd.containers.actionOne.start',
  },
  stop: {
    icon: SquareIcon,
    label: 'containerd.containers.action.stop',
    one: 'containerd.containers.actionOne.stop',
  },
  pause: {
    icon: PauseIcon,
    label: 'containerd.containers.action.pause',
    one: 'containerd.containers.actionOne.pause',
  },
  resume: {
    icon: PlayIcon,
    label: 'containerd.containers.action.resume',
    one: 'containerd.containers.actionOne.resume',
  },
};

/**
 * A container's task buttons, Kill and Remove: labelled on its page, the
 * main action alone as an icon in a list row.
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
  const actions = taskActions(container.state);
  return (
    <>
      {(compact ? actions.slice(0, 1) : actions).map((action) => {
        const { icon: Icon, label, one } = ACTIONS[action];
        return (
          <Button
            key={action}
            variant={compact ? 'ghost' : 'outline'}
            size={compact ? 'icon-sm' : 'default'}
            aria-label={compact ? t(one, { name: container.name }) : undefined}
            disabled={act.isPending}
            onClick={() => act.mutate({ container, action })}
          >
            <Icon /> {!compact && t(label)}
          </Button>
        );
      })}
      {!compact && onKill && container.state === 'running' && (
        <Button variant="outline" onClick={onKill}>
          <ZapIcon /> {t('containerd.containers.action.kill')}
        </Button>
      )}
      <Button
        variant={compact ? 'ghost' : 'outline'}
        size={compact ? 'icon-sm' : 'default'}
        aria-label={
          compact ? t('containerd.containers.removeOne', { name: container.name }) : undefined
        }
        onClick={onRemove}
      >
        <Trash2Icon /> {!compact && t('common.remove')}
      </Button>
    </>
  );
}
