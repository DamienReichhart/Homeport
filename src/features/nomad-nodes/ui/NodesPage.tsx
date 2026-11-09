import { useQuery } from '@tanstack/react-query';
import { ServerIcon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { ClusterGate } from '@/core/cluster';
import { nomadKeys, useNomadWatch } from '@/core/nomad';
import type { ClusterView, NomadNodeView } from '@/shared/api';
import {
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  StatusDot,
  type ColumnDef,
} from '@/shared/ui';
import { nomadNodeQueries } from '../api/queries';
import { nodeTone } from '../model/nodes';
import { t, type MessageKey } from '@/shared/i18n';

const STATUS_LABELS: Partial<Record<string, MessageKey>> = {
  ready: 'nomad.nodes.status.ready',
  down: 'nomad.nodes.status.down',
  initializing: 'nomad.nodes.status.initializing',
  disconnected: 'nomad.nodes.status.disconnected',
};

/** A node's status, with whether it takes new work. */
export function NodeStatus({ node }: { node: NomadNodeView }) {
  const known = STATUS_LABELS[node.status];
  const label = node.draining
    ? t('nomad.nodes.draining')
    : node.status === 'ready' && !node.eligible
      ? t('nomad.nodes.ineligible')
      : known
        ? t(known)
        : node.status;
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={nodeTone(node)} />
      {label}
    </span>
  );
}

const columns: ColumnDef<NomadNodeView, unknown>[] = [
  {
    id: 'name',
    header: t('nomad.nodes.column.name'),
    accessorFn: (n) => n.name,
    cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
  },
  {
    id: 'status',
    header: t('nomad.nodes.column.status'),
    accessorFn: (n) => n.status,
    cell: ({ row }) => <NodeStatus node={row.original} />,
  },
  {
    id: 'eligibility',
    header: t('nomad.nodes.column.eligibility'),
    accessorFn: (n) => (n.eligible ? t('nomad.nodes.eligible') : t('nomad.nodes.ineligible')),
  },
  { id: 'datacenter', header: t('nomad.nodes.column.datacenter'), accessorFn: (n) => n.datacenter },
  {
    id: 'pool',
    header: t('nomad.nodes.column.pool'),
    accessorFn: (n) => n.nodePool ?? '',
  },
  {
    id: 'running',
    header: t('nomad.nodes.column.running'),
    accessorFn: (n) => n.running ?? -1,
    cell: ({ row }) => row.original.running ?? '—',
  },
  { id: 'address', header: t('nomad.nodes.column.address'), accessorFn: (n) => n.address ?? '' },
  { id: 'version', header: t('nomad.nodes.column.version'), accessorFn: (n) => n.version ?? '' },
];

function NodesList({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const navigate = useNavigate();
  const nodes = useQuery(nomadNodeQueries.list(id));
  useNomadWatch(id, { topic: 'nodes' }, nomadKeys.nodes(id));
  return (
    <div>
      <PageHeader
        title={t('nomad.nodes.title')}
        description={nodes.data && t('nomad.nodes.summary', { count: nodes.data.length })}
      />
      {nodes.isPending ? (
        <Spinner />
      ) : nodes.error ? (
        <ErrorMessage error={nodes.error} onRetry={() => void nodes.refetch()} />
      ) : nodes.data.length === 0 ? (
        <EmptyState icon={ServerIcon} title={t('nomad.nodes.empty')} />
      ) : (
        <DataTable
          columns={columns}
          data={nodes.data}
          getRowId={(n) => n.id}
          onRowClick={(n) => void navigate(`/nomad/nodes/${n.id}`)}
        />
      )}
    </div>
  );
}

/** The selected Nomad Cluster's client nodes, kept current while the page is open. */
export function NodesPage() {
  return (
    <ClusterGate>
      {(cluster) => <NodesList key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
