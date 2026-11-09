import { PlusIcon, ShipWheelIcon } from 'lucide-react';
import { Link, useNavigate } from 'react-router-dom';
import {
  CLUSTER_KIND_LABELS,
  clusterAddress,
  ClusterStatusBadge,
  useClusters,
} from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import {
  Badge,
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import { clusterAuthLabel } from '../model/labels';
import { t } from '@/shared/i18n';

const columns: ColumnDef<ClusterView, unknown>[] = [
  {
    id: 'name',
    header: t('clusters.column.name'),
    accessorFn: (c) => c.cluster.name,
    cell: ({ row }) => <span className="font-medium">{row.original.cluster.name}</span>,
  },
  {
    id: 'status',
    header: t('clusters.column.status'),
    accessorFn: (c) => c.status.state,
    cell: ({ row }) => <ClusterStatusBadge status={row.original.status} />,
  },
  {
    id: 'kind',
    header: t('clusters.column.kind'),
    accessorFn: (c) => t(CLUSTER_KIND_LABELS[c.cluster.kind]),
    cell: ({ row }) => (
      <Badge variant="outline">{t(CLUSTER_KIND_LABELS[row.original.cluster.kind])}</Badge>
    ),
  },
  {
    id: 'server',
    header: t('clusters.column.server'),
    accessorFn: (c) => clusterAddress(c.cluster),
    cell: ({ row }) => (
      <span className="font-mono text-xs">{clusterAddress(row.original.cluster)}</span>
    ),
  },
  {
    id: 'auth',
    header: t('clusters.column.auth'),
    accessorFn: (c) => clusterAuthLabel(c.cluster),
  },
];

/** Every Cluster, with a way to add more. */
export function ClustersPage() {
  const clusters = useClusters();
  const navigate = useNavigate();
  if (clusters.isPending) return <Spinner />;
  if (clusters.error)
    return <ErrorMessage error={clusters.error} onRetry={() => void clusters.refetch()} />;
  return (
    <div>
      <PageHeader
        title={t('clusters.title')}
        description={t('clusters.summary', { count: clusters.data.length })}
        actions={
          <Button asChild>
            <Link to="/clusters/new">
              <PlusIcon /> {t('clusters.add')}
            </Link>
          </Button>
        }
      />
      {clusters.data.length === 0 ? (
        <EmptyState
          icon={ShipWheelIcon}
          title={t('clusters.empty.title')}
          description={t('clusters.empty.description')}
        />
      ) : (
        <DataTable
          columns={columns}
          data={clusters.data}
          getRowId={(c) => c.cluster.id}
          onRowClick={(c) => void navigate(`/clusters/${c.cluster.id}`)}
        />
      )}
    </div>
  );
}
