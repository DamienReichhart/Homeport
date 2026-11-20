import {
  CircleArrowUpIcon,
  CirclePlayIcon,
  CircleStopIcon,
  CloudDownloadIcon,
  EllipsisVerticalIcon,
  HammerIcon,
  ImportIcon,
  RefreshCwIcon,
  RocketIcon,
  RotateCwIcon,
  Trash2Icon,
  type LucideIcon,
} from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useHosts } from '@/core/host';
import { useCapabilities, type Stack } from '@/shared/api';
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
  type ConfirmOptions,
} from '@/shared/ui';
import { useStackMutations } from '../api/mutations';
import type { ComposeJob } from '../model/composeJob';
import type { ServiceFreshness } from '../model/freshness';
import { imagesToUpdate, updateAction } from '../model/update';
import { SyncDialog } from './dialogs/SyncDialog';
import { t } from '@/shared/i18n';

/** What brings a Stack up to date, the one of them shown beside Stop/Start. */
type Lead = 'update' | 'sync' | 'deploy' | 'adopt';

interface Action {
  icon: LucideIcon;
  label: string;
  disabled?: boolean;
  /** Why it can't be done now, or more about what it does. */
  hint?: string;
  run: () => void;
}

/**
 * The actions of a Stack. Beside Stop/Start shows the one action that brings
 * it up to date: Update when an image moved on, Sync for a Synced Stack,
 * Deploy or Redeploy otherwise, Adopt for an External Stack Homeport has no
 * record of. Everything else is in the menu. `row` shows icons, for a table.
 */
export function StackActions({
  hostId,
  stack,
  confirm,
  onCompose,
  newer = [],
  row = false,
}: {
  hostId: string;
  stack: Stack;
  confirm: (o: ConfirmOptions) => void;
  onCompose: (job: ComposeJob) => void;
  /** Its services running an outdated image, for Update. */
  newer?: ServiceFreshness[];
  /** Icon buttons for a table row, instead of labelled ones for a page header. */
  row?: boolean;
}) {
  const navigate = useNavigate();
  const capabilities = useCapabilities();
  const [syncing, setSyncing] = useState(false);
  const { act, remove } = useStackMutations(hostId);
  // Taking down and removing name the Host they act on (DESIGN.md).
  const host = useHosts().data?.find((h) => h.host.id === hostId)?.host.name ?? hostId;
  const support = capabilities.data?.compose;
  // Managed, Linked and Synced Stacks have a definition to deploy; External ones don't.
  const source = stack.kind === 'external' ? null : stack.kind;
  const deployable = source !== null && stack.id !== null;
  const canCompose = deployable && support?.state === 'available';
  const composeHint = canCompose
    ? undefined
    : support?.state === 'unavailable'
      ? support.problem.message
      : t('stacks.action.checkingCompose');
  const deployed = stack.state !== 'notDeployed';
  const compose = (action: ComposeJob['action']) => {
    if (!source || !stack.id) return;
    // A redeploy shows what changes first; a first deploy goes straight on.
    const review = deployed && (action === 'up' || action === 'rebuild') ? { review: null } : {};
    onCompose({ kind: source, stackId: stack.id, name: stack.name, action, ...review });
  };
  const update = updateAction(stack, newer);
  const adoptable = stack.kind === 'external' && stack.id === null;

  const actions: Partial<Record<Lead, Action>> = {
    ...(update === 'update' && {
      update: {
        icon: CircleArrowUpIcon,
        label: t('stacks.update.action'),
        disabled: !canCompose,
        hint: composeHint,
        run: () =>
          source &&
          stack.id &&
          onCompose({
            kind: source,
            stackId: stack.id,
            name: stack.name,
            action: 'update',
            images: imagesToUpdate(newer),
          }),
      },
    }),
    ...(source === 'synced' &&
      stack.id && {
        sync: {
          icon: RefreshCwIcon,
          label: t('stacks.action.sync'),
          disabled: !canCompose,
          hint: composeHint,
          run: () => setSyncing(true),
        },
      }),
    ...(deployable && {
      deploy: {
        icon: RocketIcon,
        label: deployed ? t('stacks.action.redeploy') : t('stacks.action.deploy'),
        disabled: !canCompose,
        hint: composeHint,
        run: () => compose('up'),
      },
    }),
    ...(adoptable && {
      adopt: {
        icon: ImportIcon,
        label: t('stacks.action.adopt'),
        hint: update === 'adoptFirst' ? t('stacks.update.adoptFirst') : undefined,
        run: () => navigate(`/stacks/new?adopt=${encodeURIComponent(stack.name)}`),
      },
    }),
  };
  // A Synced Stack deploys what it fetched: Sync leads once it runs.
  const order: Lead[] =
    source === 'synced' && deployed
      ? ['update', 'sync', 'deploy', 'adopt']
      : ['update', 'deploy', 'sync', 'adopt'];
  // A page's first deploy is its definition's own button, which opens first.
  const lead = order.find((key) => actions[key] && (row || deployed || key !== 'deploy'));
  const rest = order.filter((key) => key !== lead && actions[key]);

  const lifecycle: Action | null = !deployed
    ? null
    : stack.running > 0
      ? {
          icon: CircleStopIcon,
          label: t('stacks.action.stop'),
          run: () => act.mutate({ name: stack.name, action: 'stop' }),
        }
      : {
          icon: CirclePlayIcon,
          label: t('stacks.action.start'),
          run: () => act.mutate({ name: stack.name, action: 'start' }),
        };

  const shown = [lead && actions[lead], lifecycle].filter((a): a is Action => !!a);

  return (
    <div className="flex items-center justify-end gap-0.5" onClick={(e) => e.stopPropagation()}>
      {row ? (
        <>
          {shown.map((a) => (
            <IconAction key={a.label} action={a} />
          ))}
          {/* Keeps each icon in its column when a row has no Stop/Start. */}
          {!lifecycle && <span aria-hidden className="size-7" />}
        </>
      ) : (
        <div className="flex items-center gap-2">
          {shown.map((a) => (
            <Button
              key={a.label}
              variant="outline"
              disabled={a.disabled}
              title={a.hint}
              onClick={a.run}
            >
              <a.icon /> {a.label}
            </Button>
          ))}
        </div>
      )}
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant={row ? 'ghost' : 'outline'}
            size={row ? 'icon-sm' : 'icon'}
            className={row ? undefined : 'ms-2'}
            aria-label={t('common.moreActions')}
          >
            <EllipsisVerticalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end" className="w-56">
          {rest.map((key) => {
            const a = actions[key];
            return (
              a && (
                <DropdownMenuItem key={key} disabled={a.disabled} title={a.hint} onSelect={a.run}>
                  <a.icon /> {a.label}
                </DropdownMenuItem>
              )
            );
          })}
          {deployed && (
            <DropdownMenuItem onSelect={() => act.mutate({ name: stack.name, action: 'restart' })}>
              <RotateCwIcon /> {t('stacks.action.restart')}
            </DropdownMenuItem>
          )}
          {(source === 'linked' || source === 'synced') && (
            <DropdownMenuItem disabled={!canCompose} onSelect={() => compose('rebuild')}>
              <HammerIcon /> {t('stacks.action.rebuild')}
            </DropdownMenuItem>
          )}
          {deployable && (
            <DropdownMenuItem disabled={!canCompose} onSelect={() => compose('pull')}>
              <CloudDownloadIcon /> {t('stacks.action.pull')}
            </DropdownMenuItem>
          )}
          {/* Taking down is for what runs; a Stack never deployed has nothing to remove. */}
          {deployed && <DropdownMenuSeparator />}
          {!deployed ? null : deployable ? (
            <>
              <DropdownMenuItem
                variant="destructive"
                disabled={!canCompose}
                onSelect={() =>
                  confirm({
                    title: t('stacks.down.title', { name: stack.name, host }),
                    description: t('stacks.down.description'),
                    confirmLabel: t('stacks.down.confirm'),
                    destructive: true,
                    onConfirm: () => compose('down'),
                  })
                }
              >
                <CircleStopIcon /> {t('stacks.down.confirm')}
              </DropdownMenuItem>
              <DropdownMenuItem
                variant="destructive"
                disabled={!canCompose}
                onSelect={() =>
                  confirm({
                    title: t('stacks.downWithVolumes.title', { name: stack.name, host }),
                    description: <strong>{t('stacks.downWithVolumes.warning')}</strong>,
                    confirmLabel: t('stacks.downWithVolumes.confirm'),
                    typeToConfirm: stack.name,
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
                  title: t('stacks.remove.title', { name: stack.name, host }),
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
      {syncing && stack.id && (
        <SyncDialog
          stackId={stack.id}
          name={stack.name}
          onDeploy={onCompose}
          onClose={() => setSyncing(false)}
        />
      )}
    </div>
  );
}

/**
 * A row's ghost icon button with its name as a tooltip; a disabled one keeps
 * its tooltip, saying why, on a wrapper that still takes the pointer.
 */
function IconAction({ action: a }: { action: Action }) {
  const button = (
    <Button
      variant="ghost"
      size="icon-sm"
      aria-label={a.label}
      disabled={a.disabled}
      onClick={a.run}
    >
      <a.icon />
    </Button>
  );
  return (
    <Tooltip>
      <TooltipTrigger asChild>
        {a.disabled ? <span className="inline-flex">{button}</span> : button}
      </TooltipTrigger>
      <TooltipContent className="max-w-64">
        {a.disabled && a.hint ? `${a.label}: ${a.hint}` : (a.hint ?? a.label)}
      </TooltipContent>
    </Tooltip>
  );
}
