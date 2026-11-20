import {
  CirclePauseIcon,
  CirclePlayIcon,
  MoreHorizontalIcon,
  RotateCwIcon,
  ScalingIcon,
  TrashIcon,
  ZapIcon,
} from 'lucide-react';
import { useState } from 'react';
import { useDeleteObjects } from '@/core/cluster';
import type { ClusterView, WorkloadView } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { useRestart, useSuspend, useTrigger } from '../../api/mutations';
import { workloadActions } from '../../model/actions';
import { ScaleDialog } from './ScaleDialog';
import { t } from '@/shared/i18n';

/**
 * Everything that can be done to a workload: a menu in its row, or (`header`)
 * its everyday actions as buttons on its page with Delete in a menu beside
 * them. Restarting and suspending ask nothing, as stopping doesn't; deleting
 * asks, naming it. `onDeleted` runs once it's gone.
 */
export function WorkloadActions({
  cluster,
  workload,
  header = false,
  onDeleted,
}: {
  cluster: ClusterView;
  workload: WorkloadView;
  header?: boolean;
  onDeleted?: () => void;
}) {
  const id = cluster.cluster.id;
  const ref = { kind: workload.kind, namespace: workload.namespace, name: workload.name };
  const can = workloadActions(workload.kind);
  const suspended = workload.health === 'paused';
  const [scaling, setScaling] = useState(false);
  const [deleteDialog, remove] = useDeleteObjects(cluster);
  const restart = useRestart(id);
  const suspend = useSuspend(id);
  const trigger = useTrigger(id);

  const askDelete = () =>
    remove(
      [{ type: { kind: workload.kind }, namespace: workload.namespace, name: workload.name }],
      onDeleted,
    );

  const items = [
    can.scale && {
      key: 'scale',
      icon: <ScalingIcon />,
      label: t('workloads.action.scale'),
      run: () => setScaling(true),
      busy: false,
    },
    can.restart && {
      key: 'restart',
      icon: <RotateCwIcon />,
      label: t('workloads.action.restart'),
      run: () => restart.mutate(ref),
      busy: restart.isPending,
    },
    can.cron && {
      key: 'trigger',
      icon: <ZapIcon />,
      label: t('workloads.action.trigger'),
      run: () => trigger.mutate(ref),
      busy: trigger.isPending,
    },
    can.cron &&
      (suspended
        ? {
            key: 'resume',
            icon: <CirclePlayIcon />,
            label: t('workloads.action.resume'),
            run: () => suspend.mutate({ ref, suspended: false }),
            busy: suspend.isPending,
          }
        : {
            key: 'suspend',
            icon: <CirclePauseIcon />,
            label: t('workloads.action.suspend'),
            run: () => suspend.mutate({ ref, suspended: true }),
            busy: suspend.isPending,
          }),
  ].filter((x) => x !== false);

  const dialogs = (
    <>
      {scaling && (
        <ScaleDialog
          clusterId={id}
          clusterName={cluster.cluster.name}
          workload={ref}
          current={workload.desired}
          onClose={() => setScaling(false)}
        />
      )}
      {deleteDialog}
    </>
  );

  if (header)
    return (
      <div className="flex flex-wrap items-center gap-2">
        {items.map((item) => (
          <Button key={item.key} variant="outline" disabled={item.busy} onClick={item.run}>
            {item.icon} {item.label}
          </Button>
        ))}
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="outline" size="icon" aria-label={t('common.moreActions')}>
              <MoreHorizontalIcon />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem variant="destructive" onSelect={askDelete}>
              <TrashIcon /> {t('core.delete.action')}
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
        {dialogs}
      </div>
    );

  return (
    // Clicks in the menu don't open the row.
    <div onClick={(e) => e.stopPropagation()} onKeyDown={(e) => e.stopPropagation()}>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label={t('workloads.action.menu', { name: workload.name })}
          >
            <MoreHorizontalIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          {items.map((item) => (
            <DropdownMenuItem key={item.key} onSelect={item.run}>
              {item.icon} {item.label}
            </DropdownMenuItem>
          ))}
          {items.length > 0 && <DropdownMenuSeparator />}
          <DropdownMenuItem variant="destructive" onSelect={askDelete}>
            <TrashIcon /> {t('core.delete.action')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      {dialogs}
    </div>
  );
}
