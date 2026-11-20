import { BoxIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import type { HostView, SwarmTask } from '@/shared/api';
import {
  Button,
  StatusDot,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  type ColumnDef,
} from '@/shared/ui';
import { isFailed, taskName } from '../../model/tasks';
import { t } from '@/shared/i18n';
import { formatDateTime, formatRelative } from '@/shared/lib';

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
          className="block max-w-72 truncate font-medium hover:underline"
          title={taskName(row.original)}
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
        // A node that is one of the Hosts links to it.
        return host ? (
          <Link
            to={`/hosts/${host.host.id}`}
            className="whitespace-nowrap hover:underline"
            title={t('swarm.nodes.isHost', { name: host.host.name })}
          >
            {row.original.nodeHostname ?? host.host.name}
          </Link>
        ) : (
          <span className="whitespace-nowrap">{row.original.nodeHostname ?? '—'}</span>
        );
      },
    },
    {
      id: 'state',
      header: t('swarm.tasks.column.state'),
      accessorFn: (task) => task.state,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 whitespace-nowrap">
          <StatusDot tone={tone(row.original)} />
          <span
            title={
              row.original.desiredState !== row.original.state
                ? t('swarm.tasks.desired', { state: row.original.desiredState })
                : undefined
            }
          >
            {row.original.state}
          </span>
          {row.original.since && (
            <span
              className="text-muted-foreground text-xs"
              title={formatDateTime(row.original.since)}
            >
              · {formatRelative(Date.parse(row.original.since) / 1000)}
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
          <span
            className="text-destructive block max-w-64 truncate text-xs"
            title={row.original.error}
          >
            {row.original.error}
          </span>
        ) : row.original.state === 'running' ? null : (
          <span
            className="text-muted-foreground block max-w-48 truncate text-xs"
            title={row.original.message ?? undefined}
          >
            {row.original.message}
          </span>
        ),
    },
    {
      id: 'image',
      header: t('swarm.tasks.column.image'),
      accessorFn: (task) => task.image,
      cell: ({ row }) => (
        <span className="block max-w-48 truncate font-mono text-xs" title={row.original.image}>
          {row.original.image}
        </span>
      ),
    },
    {
      id: 'container',
      header: '',
      enableSorting: false,
      cell: ({ row }) =>
        row.original.hostId &&
        row.original.containerId && (
          <div className="flex justify-end">
            <Tooltip>
              <TooltipTrigger asChild>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('swarm.tasks.openContainer')}
                  onClick={() => onOpenContainer(row.original)}
                >
                  <BoxIcon />
                </Button>
              </TooltipTrigger>
              <TooltipContent>{t('swarm.tasks.openContainer')}</TooltipContent>
            </Tooltip>
          </div>
        ),
    },
  ];
}
