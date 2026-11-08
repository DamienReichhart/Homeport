import {
  MoreHorizontalIcon,
  PauseIcon,
  PlayIcon,
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
  useConfirmedAction,
} from '@/shared/ui';
import { useRestart, useSuspend, useTrigger } from '../../api/mutations';
import { workloadActions } from '../../model/actions';
import { ScaleDialog } from './ScaleDialog';
import { t } from '@/shared/i18n';

/**
 * Everything that can be done to a workload, as a menu: in its row, or
 * (`header`) as buttons on its page. `onDeleted` runs once it's gone.
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
  const [confirmDialog, confirm] = useConfirmedAction();
  const [deleteDialog, remove] = useDeleteObjects(cluster);
  const restart = useRestart(id);
  const suspend = useSuspend(id);
  const trigger = useTrigger(id);

  const askRestart = () =>
    confirm({
      title: t('workloads.restart.title', { name: workload.name }),
      description: t('workloads.restart.description', {
        namespace: workload.namespace,
        cluster: cluster.cluster.name,
      }),
      confirmLabel: t('workloads.restart.confirm'),
      action: () => restart.mutateAsync(ref),
    });
  const askSuspend = () =>
    confirm({
      title: t('workloads.suspend.title', { name: workload.name }),
      description: t('workloads.suspend.description', {
        namespace: workload.namespace,
        cluster: cluster.cluster.name,
      }),
      confirmLabel: t('workloads.suspend.confirm'),
      action: () => suspend.mutateAsync({ ref, suspended: true }),
    });
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
    },
    can.restart && {
      key: 'restart',
      icon: <RotateCwIcon />,
      label: t('workloads.action.restart'),
      run: askRestart,
    },
    can.cron && {
      key: 'trigger',
      icon: <ZapIcon />,
      label: t('workloads.action.trigger'),
      run: () => trigger.mutate(ref),
    },
    can.cron &&
      (suspended
        ? {
            key: 'resume',
            icon: <PlayIcon />,
            label: t('workloads.action.resume'),
            run: () => suspend.mutate({ ref, suspended: false }),
          }
        : {
            key: 'suspend',
            icon: <PauseIcon />,
            label: t('workloads.action.suspend'),
            run: askSuspend,
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
      {confirmDialog}
      {deleteDialog}
    </>
  );

  if (header)
    return (
      <div className="flex flex-wrap items-center gap-2">
        {items.map((item) => (
          <Button key={item.key} variant="outline" size="sm" onClick={item.run}>
            {item.icon} {item.label}
          </Button>
        ))}
        <Button variant="outline" size="sm" className="text-destructive" onClick={askDelete}>
          <TrashIcon /> {t('core.delete.action')}
        </Button>
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
