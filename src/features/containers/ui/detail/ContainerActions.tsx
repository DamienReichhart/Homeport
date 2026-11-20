import {
  CirclePlayIcon,
  CircleStopIcon,
  CopyIcon,
  EllipsisVerticalIcon,
  FileArchiveIcon,
  ImageDownIcon,
  PauseIcon,
  PencilIcon,
  PlayIcon,
  RotateCwIcon,
  SkullIcon,
  SlidersHorizontalIcon,
  Trash2Icon,
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { Container } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  type ConfirmOptions,
} from '@/shared/ui';
import { useContainerMutations } from '../../api/mutations';
import type { ContainerDialogRequest } from '../dialogs/ContainerDialogs';
import { removeConfirmation } from '../removeConfirmation';
import { useHostName } from '../useHostName';
import { t } from '@/shared/i18n';

/**
 * A container page's header actions: Restart and Stop/Start, the everyday
 * ones, as buttons; pausing, killing, the dialogs and removing in the menu.
 */
export function ContainerActions({
  hostId,
  container,
  confirm,
  onOpenDialog,
}: {
  hostId: string;
  container: Container;
  confirm: (options: ConfirmOptions) => void;
  onOpenDialog: (request: ContainerDialogRequest) => void;
}) {
  const navigate = useNavigate();
  const host = useHostName(hostId);
  const { act, remove, refresh } = useContainerMutations(hostId);
  const running = container.state === 'running';
  const paused = container.state === 'paused';
  const ids = [container.id];

  return (
    <>
      {(running || paused) && (
        <Button
          variant="outline"
          disabled={act.isPending}
          onClick={() => act.mutate({ ids, action: 'restart' })}
        >
          <RotateCwIcon /> {t('containers.action.restart')}
        </Button>
      )}
      {running || paused ? (
        <Button
          variant="outline"
          disabled={act.isPending}
          onClick={() => act.mutate({ ids, action: 'stop' })}
        >
          <CircleStopIcon /> {t('containers.action.stop')}
        </Button>
      ) : (
        <Button
          variant="outline"
          disabled={act.isPending}
          onClick={() => act.mutate({ ids, action: 'start' })}
        >
          <CirclePlayIcon /> {t('containers.action.start')}
        </Button>
      )}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="icon" aria-label={t('common.moreActions')}>
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-52">
          {paused ? (
            <DropdownMenuItem onSelect={() => act.mutate({ ids, action: 'unpause' })}>
              <PlayIcon /> {t('containers.action.resume')}
            </DropdownMenuItem>
          ) : (
            <DropdownMenuItem
              disabled={!running}
              onSelect={() => act.mutate({ ids, action: 'pause' })}
            >
              <PauseIcon /> {t('containers.action.pause')}
            </DropdownMenuItem>
          )}
          <DropdownMenuItem
            disabled={!running}
            onSelect={() =>
              confirm({
                title: t('containers.kill.title', { name: container.name }),
                description: t('containers.kill.description'),
                confirmLabel: t('containers.action.kill'),
                destructive: true,
                onConfirm: () => act.mutateAsync({ ids, action: 'kill' }),
              })
            }
          >
            <SkullIcon /> {t('containers.action.kill')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem onSelect={() => onOpenDialog({ kind: 'rename', container })}>
            <PencilIcon /> {t('containers.action.rename')}
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => onOpenDialog({ kind: 'edit', container })}>
            <SlidersHorizontalIcon /> {t('containers.action.edit')}
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => onOpenDialog({ kind: 'duplicate', container })}>
            <CopyIcon /> {t('containers.action.duplicate')}
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => onOpenDialog({ kind: 'commit', container })}>
            <ImageDownIcon /> {t('containers.action.commit')}
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => onOpenDialog({ kind: 'export', container })}>
            <FileArchiveIcon /> {t('containers.action.export')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            variant="destructive"
            onSelect={() =>
              confirm(
                removeConfirmation([container], host, async () => {
                  await remove.mutateAsync({ ids, volumes: true });
                  await refresh();
                  navigate('/containers');
                }),
              )
            }
          >
            <Trash2Icon /> {t('common.remove')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  );
}
