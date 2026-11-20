import { CLUSTER_KIND_LABELS, clusterAddress, describeClusterStatus } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { StatusDot, type ColumnDef } from '@/shared/ui';
import { clusterAuthLabel } from '../model/labels';
import { ClusterState } from './ClusterState';
import { t } from '@/shared/i18n';

/**
 * The Clusters table's columns, as the Hosts table's: the status is a shape
 * beside the name, and the Status column names it only when something is
 * wrong, otherwise showing what the Cluster runs.
 */
export const clusterColumns = (): ColumnDef<ClusterView, unknown>[] => [
  {
    id: 'name',
    header: t('clusters.column.name'),
    accessorFn: (c) => c.cluster.name,
    cell: ({ row }) => {
      const { label, tone } = describeClusterStatus(row.original.status, row.original.cluster.kind);
      return (
        <span className="flex min-w-0 items-center gap-2">
          <span title={label} className="inline-flex">
            <StatusDot tone={tone} pulse={row.original.status.state === 'connecting'} />
          </span>
          <span className="truncate font-medium">{row.original.cluster.name}</span>
        </span>
      );
    },
  },
  {
    id: 'kind',
    header: t('clusters.column.kind'),
    accessorFn: (c) => t(CLUSTER_KIND_LABELS[c.cluster.kind]),
    cell: ({ getValue }) => (
      <span className="text-muted-foreground whitespace-nowrap">{String(getValue())}</span>
    ),
  },
  {
    id: 'server',
    header: t('clusters.column.server'),
    accessorFn: (c) => clusterAddress(c.cluster),
    cell: ({ getValue }) => (
      <span className="block max-w-[22rem] truncate font-mono text-xs" title={String(getValue())}>
        {String(getValue())}
      </span>
    ),
  },
  {
    id: 'auth',
    header: t('clusters.column.auth'),
    accessorFn: (c) => clusterAuthLabel(c.cluster),
    cell: ({ getValue }) => (
      <span
        className="text-muted-foreground block max-w-[12rem] truncate"
        title={String(getValue())}
      >
        {String(getValue())}
      </span>
    ),
  },
  {
    id: 'status',
    header: t('clusters.column.status'),
    accessorFn: (c) => c.status.state,
    cell: ({ row }) => <ClusterState view={row.original} />,
  },
];
