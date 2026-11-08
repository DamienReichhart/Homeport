import { useQuery } from '@tanstack/react-query';
import { age, ClusterGate, objectPath, ResourceList, useNamespace } from '@/core/cluster';
import type { ClusterServiceView, ClusterView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';
import { networkQueries } from '../api/queries';
import { sections } from './sections';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<ClusterServiceView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('network.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('network.column.namespace'),
            accessorFn: (s: ClusterServiceView) => s.namespace,
          },
        ]
      : []),
    {
      id: 'type',
      header: t('network.column.type'),
      accessorFn: (s) => s.type,
      cell: ({ row }) => <Badge variant="outline">{row.original.type}</Badge>,
    },
    {
      id: 'clusterIp',
      header: t('network.column.clusterIp'),
      accessorFn: (s) => s.clusterIp ?? '',
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.clusterIp ?? '—'}</span>,
    },
    {
      id: 'external',
      header: t('network.column.external'),
      accessorFn: (s) => s.external.join(', '),
      cell: ({ row }) => (
        <span className="font-mono text-xs">{row.original.external.join(', ') || '—'}</span>
      ),
    },
    {
      id: 'ports',
      header: t('network.column.ports'),
      accessorFn: (s) => s.ports.join(', '),
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.ports.join(', ')}</span>,
    },
    {
      id: 'age',
      header: t('network.column.age'),
      accessorFn: (s) => s.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];
}

function Services({ cluster }: { cluster: ClusterView }) {
  const [namespace, setNamespace] = useNamespace(cluster);
  const list = useQuery(networkQueries.services(cluster.cluster.id, namespace));
  return (
    <ResourceList
      cluster={cluster}
      kind="service"
      title={t('network.title')}
      summary={(count) => t('network.servicesSummary', { count })}
      list={list}
      namespace={namespace}
      onNamespace={setNamespace}
      columns={columns(namespace === null)}
      deletable={(s) => ({ type: { kind: 'service' }, namespace: s.namespace, name: s.name })}
      getRowId={(s) => `${s.namespace}/${s.name}`}
      rowPath={(s) => objectPath('service', s.namespace, s.name) ?? '/services'}
      search={t('network.searchServices')}
      sections={sections()}
      current="/services"
    />
  );
}

export function ServicesPage() {
  return (
    <ClusterGate>
      {(cluster) => <Services key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
