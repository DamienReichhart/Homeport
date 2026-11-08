import { PlusIcon, ShipWheelIcon } from 'lucide-react';
import { Link, useNavigate } from 'react-router-dom';
import { ClusterStatusBadge, useClusters } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import { authLabel } from '../model/labels';
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
    id: 'server',
    header: t('clusters.column.server'),
    accessorFn: (c) => c.cluster.connection.server,
    cell: ({ row }) => (
      <span className="font-mono text-xs">{row.original.cluster.connection.server}</span>
    ),
  },
  {
    id: 'auth',
    header: t('clusters.column.auth'),
    accessorFn: (c) => authLabel(c.cluster.connection.auth),
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
