import {
  CopyIcon,
  EllipsisVerticalIcon,
  FileArchiveIcon,
  ImageDownIcon,
  PauseIcon,
  PencilIcon,
  PlayIcon,
  RotateCwIcon,
  SlidersHorizontalIcon,
  SquareIcon,
  Trash2Icon,
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { Container } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  type ConfirmOptions,
} from '@/shared/ui';
import { useContainerMutations } from '../../api/mutations';
import type { ContainerDialogRequest } from '../dialogs/ContainerDialogs';
import { removeConfirmation } from '../removeConfirmation';
import { t } from '@/shared/i18n';

/** The lifecycle buttons in a container page's header. */
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
  const { act, remove, refresh } = useContainerMutations(hostId);
  const running = container.state === 'running';
  const paused = container.state === 'paused';
  const ids = [container.id];

  return (
    <>
      {running || paused ? (
        <Button variant="outline" onClick={() => act.mutate({ ids, action: 'stop' })}>
          <SquareIcon /> {t('containers.action.stop')}
        </Button>
      ) : (
        <Button variant="outline" onClick={() => act.mutate({ ids, action: 'start' })}>
          <PlayIcon /> {t('containers.action.start')}
        </Button>
      )}
      <Button
        variant="outline"
        disabled={!running && !paused}
        onClick={() => act.mutate({ ids, action: 'restart' })}
      >
        <RotateCwIcon /> {t('containers.action.restart')}
      </Button>
      <Button
        variant="outline"
        disabled={!running && !paused}
        onClick={() => act.mutate({ ids, action: paused ? 'unpause' : 'pause' })}
      >
        {paused ? <PlayIcon /> : <PauseIcon />}{' '}
        {paused ? t('containers.action.resume') : t('containers.action.pause')}
      </Button>
      <Button
        variant="destructive"
        onClick={() =>
          confirm(
            removeConfirmation([container], async () => {
              await remove.mutateAsync({ ids, volumes: true });
              await refresh();
              navigate('/containers');
            }),
          )
        }
      >
        <Trash2Icon /> {t('common.remove')}
      </Button>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="icon" aria-label={t('common.moreActions')}>
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
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
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  );
}
