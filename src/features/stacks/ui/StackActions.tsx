import {
  CircleArrowUpIcon,
  HammerIcon,
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
import type { ServiceFreshness } from '../model/freshness';
import { imagesToUpdate, updateAction } from '../model/update';
import { t } from '@/shared/i18n';

/**
 * The actions of a Stack. Lifecycle works for every Stack; deploying needs
 * a Managed or Linked Stack and the compose CLI.
 */
export function StackActions({
  hostId,
  stack,
  confirm,
  onCompose,
  newer = [],
  compact = false,
}: {
  hostId: string;
  stack: Stack;
  confirm: (o: ConfirmOptions) => void;
  onCompose: (job: ComposeJob) => void;
  /** Its services running an outdated image, for Update. */
  newer?: ServiceFreshness[];
  compact?: boolean;
}) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const { act, remove } = useStackMutations(hostId);
  const support = capabilities.data?.compose;
  // Managed and Linked Stacks have a definition to deploy; External ones don't.
  const source = stack.kind === 'external' ? null : stack.kind;
  const deployable = source !== null && stack.id !== null;
  const canCompose = deployable && support?.state === 'available';
  const deployed = stack.state !== 'notDeployed';
  const compose = (action: ComposeJob['action']) => {
    if (!source || !stack.id) return;
    // A redeploy shows what changes first; a first deploy goes straight on.
    const review = deployed && (action === 'up' || action === 'rebuild') ? { review: null } : {};
    onCompose({ kind: source, stackId: stack.id, name: stack.name, action, ...review });
  };
  const size = compact ? 'sm' : 'default';
  const update = updateAction(stack, newer);

  return (
    <div className="flex items-center justify-end gap-1.5" onClick={(e) => e.stopPropagation()}>
      {update !== 'none' && (
        <Button
          size={size}
          variant="outline"
          disabled={update === 'adoptFirst' || !canCompose}
          title={update === 'adoptFirst' ? t('stacks.update.adoptFirst') : undefined}
          onClick={() =>
            source &&
            stack.id &&
            onCompose({
              kind: source,
              stackId: stack.id,
              name: stack.name,
              action: 'update',
              images: imagesToUpdate(newer),
            })
          }
        >
          <CircleArrowUpIcon /> {t('stacks.update.action')}
        </Button>
      )}
      {deployable ? (
        <Button
          size={size}
          disabled={!canCompose}
          title={
            canCompose
              ? undefined
              : support?.state === 'unavailable'
                ? support.problem.message
                : t('stacks.action.checkingCompose')
          }
          onClick={() => compose('up')}
        >
          <RocketIcon /> {deployed ? t('stacks.action.redeploy') : t('stacks.action.deploy')}
        </Button>
      ) : stack.kind === 'external' && stack.id === null ? (
        <Button
          size={size}
          variant="outline"
          onClick={() => navigate(`/stacks/new?adopt=${encodeURIComponent(stack.name)}`)}
        >
          {t('stacks.action.adopt')}
        </Button>
      ) : null}
      {deployed &&
        (stack.running > 0 ? (
          <Button
            size={size}
            variant="outline"
            onClick={() => act.mutate({ name: stack.name, action: 'stop' })}
          >
            <SquareIcon /> {t('stacks.action.stop')}
          </Button>
        ) : (
          <Button
            size={size}
            variant="outline"
            onClick={() => act.mutate({ name: stack.name, action: 'start' })}
          >
            <PlayIcon /> {t('stacks.action.start')}
          </Button>
        ))}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            size={compact ? 'icon-sm' : 'icon'}
            aria-label={t('stacks.action.more')}
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem
            disabled={!deployed}
            onSelect={() => act.mutate({ name: stack.name, action: 'restart' })}
          >
            <RotateCwIcon /> {t('stacks.action.restart')}
          </DropdownMenuItem>
          {source === 'linked' && (
            <DropdownMenuItem disabled={!canCompose} onSelect={() => compose('rebuild')}>
              <HammerIcon /> {t('stacks.action.rebuild')}
            </DropdownMenuItem>
          )}
          {deployable && (
            <DropdownMenuItem disabled={!canCompose} onSelect={() => compose('pull')}>
              <CloudDownloadIcon /> {t('stacks.action.pull')}
            </DropdownMenuItem>
          )}
          <DropdownMenuSeparator />
          {deployable ? (
            <>
              <DropdownMenuItem
                variant="destructive"
                disabled={!canCompose || !deployed}
                onSelect={() =>
                  confirm({
                    title: t('stacks.down.title', { name: stack.name }),
                    description: t('stacks.down.description'),
                    confirmLabel: t('stacks.down.confirm'),
                    destructive: true,
                    onConfirm: () => compose('down'),
                  })
                }
              >
                <SquareIcon /> {t('stacks.down.confirm')}
              </DropdownMenuItem>
              <DropdownMenuItem
                variant="destructive"
                disabled={!canCompose || !deployed}
                onSelect={() =>
                  confirm({
                    title: t('stacks.downWithVolumes.title', { name: stack.name }),
                    description: <strong>{t('stacks.downWithVolumes.warning')}</strong>,
                    confirmLabel: t('stacks.downWithVolumes.confirm'),
                    destructive: true,
                    onConfirm: () => compose('downWithVolumes'),
                  })
                }
              >
                <Trash2Icon /> {t('stacks.downWithVolumes.action')}
              </DropdownMenuItem>
            </>
          ) : (
            <DropdownMenuItem
              variant="destructive"
              onSelect={() =>
                confirm({
                  title: t('stacks.remove.title', { name: stack.name }),
                  description: t('stacks.remove.description', { count: stack.total }),
                  confirmLabel: t('common.remove'),
                  destructive: true,
                  onConfirm: () => remove.mutateAsync(stack.name),
                })
              }
            >
              <Trash2Icon /> {t('stacks.remove.action')}
            </DropdownMenuItem>
          )}
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}
