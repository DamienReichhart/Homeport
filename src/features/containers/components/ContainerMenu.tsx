import {
  EllipsisVerticalIcon,
  PauseIcon,
  PencilIcon,
  PlayIcon,
  RotateCwIcon,
  ScrollTextIcon,
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

/** Confirmation for removing containers, naming what goes with them. */
export function removeConfirmation(
  containers: Container[],
  onConfirm: () => Promise<unknown>,
): ConfirmOptions {
  const running = containers.filter((c) => c.state === 'running').length;
  const what = containers.length === 1 ? containers[0]?.name : `${containers.length} containers`;
  return {
    title: `Remove ${what}?`,
    description: (
      <>
        The container{containers.length === 1 ? '' : 's'} and{' '}
        {containers.length === 1 ? 'its' : 'their'} writable layer and anonymous volumes are
        deleted. {running > 0 && <strong>{running} running will be stopped first. </strong>}
        Named volumes are kept.
      </>
    ),
    confirmLabel: 'Remove',
    destructive: true,
    onConfirm,
  };
}

/** Start/stop toggle plus the overflow menu of a container row. */
export function ContainerMenu({
  hostId,
  container,
  onRename,
  confirm,
}: {
  hostId: string;
  container: Container;
  onRename: (c: Container) => void;
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
            aria-label={running || paused ? 'Stop' : 'Start'}
            disabled={act.isPending}
            onClick={() => act.mutate({ ids, action: running || paused ? 'stop' : 'start' })}
          >
            {running || paused ? <SquareIcon /> : <PlayIcon />}
          </Button>
        </TooltipTrigger>
        <TooltipContent>{running || paused ? 'Stop' : 'Start'}</TooltipContent>
      </Tooltip>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="ghost" size="icon-sm" aria-label="More actions">
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem onSelect={() => navigate(`/containers/${container.id}/logs`)}>
            <ScrollTextIcon /> Logs
          </DropdownMenuItem>
          <DropdownMenuItem
            disabled={!running}
            onSelect={() => navigate(`/containers/${container.id}/exec`)}
          >
            <SquareTerminalIcon /> Open terminal
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            disabled={!running && !paused}
            onSelect={() => act.mutate({ ids, action: 'restart' })}
          >
            <RotateCwIcon /> Restart
          </DropdownMenuItem>
          {paused ? (
            <DropdownMenuItem onSelect={() => act.mutate({ ids, action: 'unpause' })}>
              <PlayIcon /> Resume
            </DropdownMenuItem>
          ) : (
            <DropdownMenuItem
              disabled={!running}
              onSelect={() => act.mutate({ ids, action: 'pause' })}
            >
              <PauseIcon /> Pause
            </DropdownMenuItem>
          )}
          <DropdownMenuItem
            disabled={!running}
            onSelect={() =>
              confirm({
                title: `Kill ${container.name}?`,
                description:
                  'The main process gets SIGKILL immediately, without a chance to shut down cleanly.',
                confirmLabel: 'Kill',
                destructive: true,
                onConfirm: () => act.mutateAsync({ ids, action: 'kill' }),
              })
            }
          >
            <SkullIcon /> Kill
          </DropdownMenuItem>
          <DropdownMenuItem onSelect={() => onRename(container)}>
            <PencilIcon /> Rename
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
            <Trash2Icon /> Remove
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
