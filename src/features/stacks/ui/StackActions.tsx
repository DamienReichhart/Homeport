import {
  CloudDownloadIcon,
  EllipsisVerticalIcon,
  PlayIcon,
  RocketIcon,
  RotateCwIcon,
  SquareIcon,
  Trash2Icon,
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { useCapabilities, type Stack } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  type ConfirmOptions,
} from '@/shared/ui';
import { useStackMutations } from '../api/mutations';
import type { ComposeJob } from '../model/composeJob';

/**
 * The actions of a Stack. Lifecycle works for every Stack; deploying needs
 * a Managed Stack and the compose CLI.
 */
export function StackActions({
  hostId,
  stack,
  confirm,
  onCompose,
  compact = false,
}: {
  hostId: string;
  stack: Stack;
  confirm: (o: ConfirmOptions) => void;
  onCompose: (job: ComposeJob) => void;
  compact?: boolean;
}) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const { act, remove } = useStackMutations(hostId);
  const support = capabilities.data?.compose;
  const canCompose = stack.managedId !== null && support?.state === 'available';
  const compose = (action: ComposeJob['action']) => {
    if (stack.managedId) onCompose({ stackId: stack.managedId, name: stack.name, action });
  };
  const deployed = stack.state !== 'notDeployed';
  const size = compact ? 'sm' : 'default';

  return (
    <div className="flex items-center justify-end gap-1.5" onClick={(e) => e.stopPropagation()}>
      {stack.managedId ? (
        <Button
          size={size}
          disabled={!canCompose}
          title={
            canCompose
              ? undefined
              : support?.state === 'unavailable'
                ? support.problem.message
                : 'Checking for the docker compose CLI…'
          }
          onClick={() => compose('up')}
        >
          <RocketIcon /> {deployed ? 'Redeploy' : 'Deploy'}
        </Button>
      ) : (
        <Button
          size={size}
          variant="outline"
          onClick={() => navigate(`/stacks/new?adopt=${encodeURIComponent(stack.name)}`)}
        >
          Adopt
        </Button>
      )}
      {deployed &&
        (stack.running > 0 ? (
          <Button
            size={size}
            variant="outline"
            onClick={() => act.mutate({ name: stack.name, action: 'stop' })}
          >
            <SquareIcon /> Stop
          </Button>
        ) : (
          <Button
            size={size}
            variant="outline"
            onClick={() => act.mutate({ name: stack.name, action: 'start' })}
          >
            <PlayIcon /> Start
          </Button>
        ))}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            size={compact ? 'icon-sm' : 'icon'}
            aria-label="More stack actions"
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem
            disabled={!deployed}
            onSelect={() => act.mutate({ name: stack.name, action: 'restart' })}
          >
            <RotateCwIcon /> Restart
          </DropdownMenuItem>
          {stack.managedId && (
            <DropdownMenuItem disabled={!canCompose} onSelect={() => compose('pull')}>
              <CloudDownloadIcon /> Pull images
            </DropdownMenuItem>
          )}
          <DropdownMenuSeparator />
          {stack.managedId ? (
            <>
              <DropdownMenuItem
                variant="destructive"
                disabled={!canCompose || !deployed}
                onSelect={() =>
                  confirm({
                    title: `Take down ${stack.name}?`,
                    description:
                      'Its containers and networks are removed. Volumes and the stored definition are kept, so you can deploy again.',
                    confirmLabel: 'Take down',
                    destructive: true,
                    onConfirm: () => compose('down'),
                  })
                }
              >
                <SquareIcon /> Take down
              </DropdownMenuItem>
              <DropdownMenuItem
                variant="destructive"
                disabled={!canCompose || !deployed}
                onSelect={() =>
                  confirm({
                    title: `Take down ${stack.name} and delete its volumes?`,
                    description: (
                      <strong>
                        Containers, networks and every volume of this Stack are deleted, with all
                        their data.
                      </strong>
                    ),
                    confirmLabel: 'Delete with volumes',
                    destructive: true,
                    onConfirm: () => compose('downWithVolumes'),
                  })
                }
              >
                <Trash2Icon /> Take down with volumes
              </DropdownMenuItem>
            </>
          ) : (
            <DropdownMenuItem
              variant="destructive"
              onSelect={() =>
                confirm({
                  title: `Remove ${stack.name}?`,
                  description: `Its ${stack.total} container(s) and networks are removed. Volumes are kept. Homeport has no compose file for this Stack, so it can't be redeployed from here.`,
                  confirmLabel: 'Remove',
                  destructive: true,
                  onConfirm: () => remove.mutateAsync(stack.name),
                })
              }
            >
              <Trash2Icon /> Remove containers
            </DropdownMenuItem>
          )}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
