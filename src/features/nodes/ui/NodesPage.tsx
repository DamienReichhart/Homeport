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

const columns: ColumnDef<ClusterNodeView, unknown>[] = [
  {
    id: 'name',
    header: t('nodes.column.name'),
    accessorFn: (n) => n.name,
    cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
  },
  {
    id: 'status',
    header: t('nodes.column.status'),
    accessorFn: (n) => (n.ready ? 1 : 0),
    cell: ({ row }) => <NodeStatus node={row.original} />,
  },
  { id: 'roles', header: t('nodes.column.roles'), accessorFn: (n) => n.roles.join(', ') || '—' },
  { id: 'version', header: t('nodes.column.version'), accessorFn: (n) => n.version },
  {
    id: 'ip',
    header: t('nodes.column.ip'),
    accessorFn: (n) => n.internalIp ?? '',
    cell: ({ row }) => <span className="font-mono text-xs">{row.original.internalIp}</span>,
  },
  { id: 'platform', header: t('nodes.column.platform'), accessorFn: (n) => `${n.os}/${n.arch}` },
  {
    id: 'cpu',
    header: t('nodes.column.cpu'),
    accessorFn: (n) => coresOf(n.cpu ?? '') ?? 0,
    cell: ({ row }) => readableQuantity('cpu', row.original.cpu),
  },
  {
    id: 'memory',
    header: t('nodes.column.memory'),
    accessorFn: (n) => bytesOf(n.memory ?? '') ?? 0,
    cell: ({ row }) => readableQuantity('memory', row.original.memory),
  },
  {
    id: 'age',
    header: t('nodes.column.age'),
    accessorFn: (n) => n.created ?? '',
    cell: ({ row }) => age(row.original.created),
  },
];

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
      columns={
        usage ? [...columns.slice(0, -1), ...usageColumns(usage), ...columns.slice(-1)] : columns
      }
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
