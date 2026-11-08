import { age } from '@/core/cluster';
import { ForwardButton } from '@/features/port-forwards';
import type { PodContainer } from '@/shared/api';
import { Badge, DataTable, StatusDot, type ColumnDef } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

const STATES: Partial<Record<string, MessageKey>> = {
  running: 'pods.state.running',
  waiting: 'pods.state.waiting',
  terminated: 'pods.state.terminated',
};

const columns: ColumnDef<PodContainer, unknown>[] = [
  {
    id: 'name',
    header: t('pods.container.name'),
    accessorFn: (c) => c.name,
    cell: ({ row }) => (
      <span className="flex items-center gap-2 font-medium">
        {row.original.name}
        {row.original.init && <Badge variant="outline">{t('pods.container.init')}</Badge>}
      </span>
    ),
  },
  {
    id: 'image',
    header: t('pods.container.image'),
    accessorFn: (c) => c.image,
    cell: ({ row }) => <span className="font-mono text-xs break-all">{row.original.image}</span>,
  },
  {
    id: 'state',
    header: t('pods.container.state'),
    accessorFn: (c) => c.state,
    cell: ({ row }) => {
      const c = row.original;
      const tone =
        c.state === 'running'
          ? c.ready
            ? 'success'
            : 'warning'
          : c.state === 'waiting'
            ? 'info'
            : c.exitCode === 0
              ? 'muted'
              : 'danger';
      return (
        <span className="grid gap-0.5">
          <span className="inline-flex items-center gap-1.5">
            <StatusDot tone={tone} />
            {c.reason ?? t(STATES[c.state] ?? 'pods.state.waiting')}
            {c.exitCode !== null && c.state === 'terminated' && ` (${c.exitCode})`}
          </span>
          {c.message && <span className="text-muted-foreground text-xs">{c.message}</span>}
        </span>
      );
    },
  },
  {
    id: 'ready',
    header: t('pods.container.ready'),
    accessorFn: (c) => (c.ready ? t('pods.container.isReady') : t('pods.container.notReady')),
  },
  {
    id: 'restarts',
    header: t('pods.container.restarts'),
    accessorFn: (c) => c.restarts,
    cell: ({ row }) =>
      row.original.lastReason
        ? t('pods.container.restartsWhy', {
            count: row.original.restarts,
            reason: row.original.lastReason,
          })
        : row.original.restarts,
  },
  {
    id: 'started',
    header: t('pods.container.started'),
    accessorFn: (c) => c.started ?? '',
    cell: ({ row }) => age(row.original.started),
  },
];

/** The ports column: each port a container declares, TCP ones with Forward. */
function portsColumn(
  clusterId: string,
  namespace: string,
  pod: string,
): ColumnDef<PodContainer, unknown> {
  return {
    id: 'ports',
    header: t('pods.container.ports'),
    accessorFn: (c) => c.ports.map((p) => p.port).join(', '),
    cell: ({ row }) => (
      <span className="grid gap-1">
        {row.original.ports.map((p) => (
          <span key={`${p.port}/${p.protocol}`} className="flex items-center gap-1">
            <span className="font-mono text-xs">
              {p.name ? `${p.name} ` : ''}
              {p.port}/{p.protocol}
            </span>
            {p.protocol === 'TCP' && !row.original.init && (
              <ForwardButton
                clusterId={clusterId}
                target={{ kind: 'pod', namespace, name: pod }}
                port={p.port}
              />
            )}
          </span>
        ))}
      </span>
    ),
  };
}

/** A Pod's containers, init containers first, with their state, restarts and ports. */
export function PodContainers({
  clusterId,
  namespace,
  pod,
  containers,
}: {
  clusterId: string;
  namespace: string;
  pod: string;
  containers: PodContainer[];
}) {
  const withPorts = containers.some((c) => c.ports.length > 0);
  return (
    <DataTable
      columns={withPorts ? [...columns, portsColumn(clusterId, namespace, pod)] : columns}
      data={containers}
      getRowId={(c) => `${c.init}:${c.name}`}
    />
  );
}
