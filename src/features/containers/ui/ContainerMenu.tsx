import {
  CopyIcon,
  EllipsisVerticalIcon,
  FileArchiveIcon,
  ImageDownIcon,
  PauseIcon,
  PencilIcon,
  PlayIcon,
  RotateCwIcon,
  ScrollTextIcon,
  SlidersHorizontalIcon,
  SkullIcon,
  SquareIcon,
  SquareTerminalIcon,
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
  Tooltip,
  TooltipContent,
  TooltipTrigger,
} from '@/shared/ui';
import type { ConfirmOptions } from '@/shared/ui';
import { useContainerMutations } from '../api/mutations';
import type { ContainerDialogRequest } from './dialogs/ContainerDialogs';
import { removeConfirmation } from './removeConfirmation';
import { t } from '@/shared/i18n';

/** Start/stop toggle plus the overflow menu of a container row. */
export function ContainerMenu({
  hostId,
  container,
  onOpenDialog,
  confirm,
}: {
  hostId: string;
  container: Container;
  onOpenDialog: (request: ContainerDialogRequest) => void;
  confirm: (options: ConfirmOptions) => void;
}) {
  const navigate = useNavigate();
  const { act, remove } = useContainerMutations(hostId);
  const ids = [container.id];
  const running = container.state === 'running';
  const paused = container.state === 'paused';

  return (
    <div className="flex items-center justify-end gap-0.5" onClick={(e) => e.stopPropagation()}>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={
              running || paused ? t('containers.action.stop') : t('containers.action.start')
            }
            disabled={act.isPending}
            onClick={() => act.mutate({ ids, action: running || paused ? 'stop' : 'start' })}
          >
            {running || paused ? <SquareIcon /> : <PlayIcon />}
          </Button>
        </TooltipTrigger>
        <TooltipContent>
          {running || paused ? t('containers.action.stop') : t('containers.action.start')}
        </TooltipContent>
      </Tooltip>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon-sm" aria-label={t('common.moreActions')}>
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onSelect={() => navigate(`/containers/${container.id}/logs`)}>
            <ScrollTextIcon /> {t('containers.action.logs')}
          </DropdownMenuItem>
          <DropdownMenuItem
            disabled={!running}
            onSelect={() => navigate(`/containers/${container.id}/exec`)}
          >
            <SquareTerminalIcon /> {t('containers.action.terminal')}
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            disabled={!running && !paused}
            onSelect={() => act.mutate({ ids, action: 'restart' })}
          >
            <RotateCwIcon /> {t('containers.action.restart')}
          </DropdownMenuItem>
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
                removeConfirmation([container], () => remove.mutateAsync({ ids, volumes: true })),
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
