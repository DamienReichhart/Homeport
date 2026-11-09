import {
  EllipsisVerticalIcon,
  PauseIcon,
  PlayIcon,
  PowerOffIcon,
  RotateCwIcon,
  SquareIcon,
  Trash2Icon,
  type LucideIcon,
} from 'lucide-react';
import type { IncusInstance, InstanceAction } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { useInstanceAction } from '../api/mutations';
import { instanceActions } from '../model/instances';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<InstanceAction, { icon: LucideIcon; label: MessageKey; one: MessageKey }> = {
  start: { icon: PlayIcon, label: 'incus.action.start', one: 'incus.actionOne.start' },
  stop: { icon: SquareIcon, label: 'incus.action.stop', one: 'incus.actionOne.stop' },
  restart: { icon: RotateCwIcon, label: 'incus.action.restart', one: 'incus.actionOne.restart' },
  freeze: { icon: PauseIcon, label: 'incus.action.freeze', one: 'incus.actionOne.freeze' },
  unfreeze: { icon: PlayIcon, label: 'incus.action.unfreeze', one: 'incus.actionOne.unfreeze' },
};

/**
 * An Instance's actions: its main one as a button, the others, a forced
 * stop and Delete in a menu. A list row shows the main one as an icon.
 */
export function InstanceActions({
  hostId,
  project,
  instance,
  compact = false,
  onDelete,
}: {
  hostId: string;
  project: string;
  instance: IncusInstance;
  compact?: boolean;
  onDelete: () => void;
}) {
  const act = useInstanceAction(hostId, project);
  const [main, ...rest] = instanceActions(instance.status);
  const run = (action: InstanceAction, force = false) =>
    act.mutate({ name: instance.name, action, force });
  const up = instance.status === 'running' || instance.status === 'frozen';
  const MainIcon = main ? ACTIONS[main].icon : null;
  return (
    <div className="flex items-center gap-1">
      {main && MainIcon && (
        <Button
          variant={compact ? 'ghost' : 'outline'}
          size={compact ? 'icon-sm' : 'default'}
          aria-label={compact ? t(ACTIONS[main].one, { name: instance.name }) : undefined}
          disabled={act.isPending}
          onClick={() => run(main)}
        >
          <MainIcon /> {!compact && t(ACTIONS[main].label)}
        </Button>
      )}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant={compact ? 'ghost' : 'outline'}
            size={compact ? 'icon-sm' : 'icon'}
            aria-label={t('incus.instances.more', { name: instance.name })}
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {rest.map((action) => {
            const { icon: Icon, label } = ACTIONS[action];
            return (
              <DropdownMenuItem key={action} disabled={act.isPending} onSelect={() => run(action)}>
                <Icon /> {t(label)}
              </DropdownMenuItem>
            );
          })}
          {up && (
            <DropdownMenuItem disabled={act.isPending} onSelect={() => run('stop', true)}>
              <PowerOffIcon /> {t('incus.action.forceStop')}
            </DropdownMenuItem>
          )}
          {(rest.length > 0 || up) && <DropdownMenuSeparator />}
          <DropdownMenuItem variant="destructive" onSelect={onDelete}>
            <Trash2Icon /> {t('incus.delete')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
