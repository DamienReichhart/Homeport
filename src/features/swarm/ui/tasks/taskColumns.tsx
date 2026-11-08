import { Link } from 'react-router-dom';
import type { HostView, SwarmTask } from '@/shared/api';
import { Button, StatusDot, type ColumnDef } from '@/shared/ui';
import { isFailed, taskName } from '../../model/tasks';
import { t } from '@/shared/i18n';

const tone = (task: SwarmTask) =>
  isFailed(task) ? 'danger' : task.state === 'running' ? 'success' : 'warning';

/** The Tasks table's columns. */
export function taskColumns({
  swarmId,
  hosts,
  onOpenContainer,
}: {
  swarmId: string;
  hosts: HostView[] | undefined;
  onOpenContainer: (task: SwarmTask) => void;
}): ColumnDef<SwarmTask, unknown>[] {
  return [
    {
      id: 'task',
      header: t('swarm.tasks.column.task'),
      accessorFn: taskName,
      cell: ({ row }) => (
        <Link
          to={`/swarms/${swarmId}/services/${row.original.serviceId}/tasks`}
          className="font-medium hover:underline"
        >
          {taskName(row.original)}
        </Link>
      ),
    },
    {
      id: 'node',
      header: t('swarm.tasks.column.node'),
      accessorFn: (task) => task.nodeHostname ?? '',
      cell: ({ row }) => {
        const host = hosts?.find((h) => h.host.id === row.original.hostId);
        return (
          <span className="flex flex-wrap items-center gap-2">
            {row.original.nodeHostname ?? '—'}
            {host && (
              <Link to={`/hosts/${host.host.id}`} className="text-primary text-xs hover:underline">
                {t('swarm.nodes.isHost', { name: host.host.name })}
              </Link>
            )}
          </span>
        );
      },
    },
    {
      id: 'state',
      header: t('swarm.tasks.column.state'),
      accessorFn: (task) => task.state,
      cell: ({ row }) => (
        <span className="grid gap-0.5">
          <span className="flex items-center gap-2">
            <StatusDot tone={tone(row.original)} />
            {row.original.state}
            {row.original.desiredState !== row.original.state && (
              <span className="text-muted-foreground text-xs">
                {t('swarm.tasks.desired', { state: row.original.desiredState })}
              </span>
            )}
          </span>
          {row.original.since && (
            <span className="text-muted-foreground text-xs">
              {new Date(row.original.since).toLocaleString()}
            </span>
          )}
        </span>
      ),
    },
    {
      id: 'error',
      header: t('swarm.tasks.column.error'),
      accessorFn: (task) => task.error ?? task.message ?? '',
      cell: ({ row }) =>
        row.original.error ? (
          <span className="text-destructive text-xs break-words">{row.original.error}</span>
        ) : (
          <span className="text-muted-foreground text-xs">{row.original.message}</span>
        ),
    },
    {
      id: 'image',
      header: t('swarm.tasks.column.image'),
      accessorFn: (task) => task.image,
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.image}</span>,
    },
    {
      id: 'container',
      header: '',
      enableSorting: false,
      cell: ({ row }) =>
        row.original.hostId &&
        row.original.containerId && (
          <Button variant="link" size="sm" onClick={() => onOpenContainer(row.original)}>
            {t('swarm.tasks.openContainer')}
          </Button>
        ),
    },
  ];
}
