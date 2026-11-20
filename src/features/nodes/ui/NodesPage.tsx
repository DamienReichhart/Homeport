import { useQuery } from '@tanstack/react-query';
import {
  age,
  ClusterGate,
  formatCpu,
  objectPath,
  ResourceList,
  UsageCell,
  usageFormat,
  useMetrics,
} from '@/core/cluster';
import type { ClusterNodeView, ClusterView, NodeUsage } from '@/shared/api';
import { type ColumnDef } from '@/shared/ui';
import { nodeQueries } from '../api/queries';
import { bytesOf, coresOf, readableQuantity } from '../model/quantity';
import { NodeStatus } from './NodeStatus';
import { t } from '@/shared/i18n';

/** What each Node uses of CPU and memory against its allocatable, sorted by use. */
function usageColumns(usage: Map<string, NodeUsage>): ColumnDef<ClusterNodeView, unknown>[] {
  const of = (n: ClusterNodeView) => usage.get(n.name)?.usage;
  const cpuOf = (u: { cpu: number | null } | undefined) => (u ? (u.cpu ?? 0) : undefined);
  const allocatable = (value: number | null) =>
    value ? { kind: 'allocatable' as const, value } : null;
  return [
    {
      id: 'cpuUsed',
      header: t('nodes.column.cpuUsed'),
      accessorFn: (n) => of(n)?.cpu ?? -1,
      cell: ({ row }) => (
        <UsageCell
          used={cpuOf(of(row.original))}
          measure={allocatable(row.original.cpuMillicores)}
          format={formatCpu}
        />
      ),
    },
    {
      id: 'memoryUsed',
      header: t('nodes.column.memoryUsed'),
      accessorFn: (n) => of(n)?.memory ?? -1,
      cell: ({ row }) => (
        <UsageCell
          used={of(row.original)?.memory}
          measure={allocatable(row.original.memoryBytes)}
          format={usageFormat('memory')}
        />
      ),
    },
  ];
}

/**
 * The Nodes table's columns. Roles shows only when some Node has one (a
 * managed Cluster's Nodes have none), and with use from metrics-server the
 * capacity columns give way to use against the allocatable.
 */
function columns(
  nodes: ClusterNodeView[],
  usage: Map<string, NodeUsage> | null,
): ColumnDef<ClusterNodeView, unknown>[] {
  const withRoles = nodes.some((n) => n.roles.length > 0);
  return [
    ...head,
    ...(withRoles ? [roles] : []),
    ...facts,
    ...(usage ? usageColumns(usage) : capacity),
    ageColumn,
  ];
}

const head: ColumnDef<ClusterNodeView, unknown>[] = [
  {
    id: 'name',
    header: t('nodes.column.name'),
    accessorFn: (n) => n.name,
    cell: ({ row }) => (
      <span
        className="inline-block max-w-64 truncate align-bottom font-medium"
        title={row.original.name}
      >
        {row.original.name}
      </span>
    ),
  },
  {
    id: 'status',
    header: t('nodes.column.status'),
    accessorFn: (n) => (n.ready ? 1 : 0),
    cell: ({ row }) => (
      <span className="block max-w-40">
        <NodeStatus node={row.original} />
      </span>
    ),
    meta: { compact: true },
  },
];

const roles: ColumnDef<ClusterNodeView, unknown> = {
  id: 'roles',
  header: t('nodes.column.roles'),
  accessorFn: (n) => n.roles.join(', ') || '—',
};

const facts: ColumnDef<ClusterNodeView, unknown>[] = [
  {
    id: 'version',
    header: t('nodes.column.version'),
    accessorFn: (n) => n.version,
    cell: ({ row }) => (
      <span className="inline-block max-w-48 truncate align-bottom" title={row.original.version}>
        {row.original.version}
      </span>
    ),
  },
  {
    id: 'ip',
    header: t('nodes.column.ip'),
    accessorFn: (n) => n.internalIp ?? '',
    cell: ({ row }) => <span className="font-mono text-xs">{row.original.internalIp}</span>,
  },
  { id: 'platform', header: t('nodes.column.platform'), accessorFn: (n) => `${n.os}/${n.arch}` },
];

const capacity: ColumnDef<ClusterNodeView, unknown>[] = [
  {
    id: 'cpu',
    header: t('nodes.column.cpu'),
    accessorFn: (n) => coresOf(n.cpu ?? '') ?? 0,
    cell: ({ row }) => (
      <span className="tabular-nums">{readableQuantity('cpu', row.original.cpu)}</span>
    ),
  },
  {
    id: 'memory',
    header: t('nodes.column.memory'),
    accessorFn: (n) => bytesOf(n.memory ?? '') ?? 0,
    cell: ({ row }) => (
      <span className="tabular-nums">{readableQuantity('memory', row.original.memory)}</span>
    ),
  },
];

const ageColumn: ColumnDef<ClusterNodeView, unknown> = {
  id: 'age',
  header: t('nodes.column.age'),
  accessorFn: (n) => n.created ?? '',
  cell: ({ row }) => age(row.original.created),
};

function Nodes({ cluster }: { cluster: ClusterView }) {
  const list = useQuery(nodeQueries.list(cluster.cluster.id));
  // Without metrics-server there are no columns of use, and nothing to say.
  const metrics = useMetrics(cluster.cluster.id, { type: 'nodes' });
  const usage = metrics.nodes;
  return (
    <ResourceList
      cluster={cluster}
      kind="node"
      title={t('nodes.title')}
      summary={(count) => t('nodes.summary', { count })}
      list={list}
      columns={columns(list.data ?? [], usage)}
      deletable={(n) => ({ type: { kind: 'node' }, namespace: null, name: n.name })}
      getRowId={(n) => n.name}
      rowPath={(n) => objectPath('node', null, n.name) ?? '/nodes'}
      search={t('nodes.search')}
    />
  );
}

export function NodesPage() {
  return (
    <ClusterGate>{(cluster) => <Nodes key={cluster.cluster.id} cluster={cluster} />}</ClusterGate>
  );
}
