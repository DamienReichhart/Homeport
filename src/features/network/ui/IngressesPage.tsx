import { useQuery } from '@tanstack/react-query';
import { age, ClusterGate, objectPath, ResourceList, useNamespace } from '@/core/cluster';
import type { ClusterView, IngressView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';
import { networkQueries } from '../api/queries';
import { sections } from './sections';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<IngressView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('network.column.name'),
      accessorFn: (i) => i.name,
      cell: ({ row }) => (
        <span className="flex items-center gap-2 font-medium">
          {row.original.name}
          {row.original.tls && <Badge variant="secondary">TLS</Badge>}
        </span>
      ),
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('network.column.namespace'),
            accessorFn: (i: IngressView) => i.namespace,
          },
        ]
      : []),
    { id: 'class', header: t('network.column.class'), accessorFn: (i) => i.class ?? '' },
    {
      id: 'hosts',
      header: t('network.column.hosts'),
      accessorFn: (i) => i.hosts.join(', ') || '*',
    },
    {
      id: 'addresses',
      header: t('network.column.addresses'),
      accessorFn: (i) => i.addresses.join(', '),
      cell: ({ row }) => (
        <span className="font-mono text-xs">{row.original.addresses.join(', ') || '—'}</span>
      ),
    },
    {
      id: 'age',
      header: t('network.column.age'),
      accessorFn: (i) => i.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];
}

function Ingresses({ cluster }: { cluster: ClusterView }) {
  const [namespace, setNamespace] = useNamespace(cluster);
  const list = useQuery(networkQueries.ingresses(cluster.cluster.id, namespace));
  return (
    <ResourceList
      cluster={cluster}
      kind="ingress"
      title={t('network.title')}
      summary={(count) => t('network.ingressesSummary', { count })}
      list={list}
      namespace={namespace}
      onNamespace={setNamespace}
      columns={columns(namespace === null)}
      deletable={(i) => ({ type: { kind: 'ingress' }, namespace: i.namespace, name: i.name })}
      getRowId={(i) => `${i.namespace}/${i.name}`}
      rowPath={(i) => objectPath('ingress', i.namespace, i.name) ?? '/ingresses'}
      search={t('network.searchIngresses')}
      sections={sections()}
      current="/ingresses"
    />
  );
}

export function IngressesPage() {
  return (
    <ClusterGate>
      {(cluster) => <Ingresses key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
