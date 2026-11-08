import { useQuery } from '@tanstack/react-query';
import { age, ClusterGate, objectPath, ResourceList, useNamespace } from '@/core/cluster';
import type { ClusterView, SecretView } from '@/shared/api';
import { Badge, type ColumnDef } from '@/shared/ui';
import { configQueries } from '../api/queries';
import { sections } from './sections';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<SecretView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('config.column.name'),
      accessorFn: (s) => s.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('config.column.namespace'),
            accessorFn: (s: SecretView) => s.namespace,
          },
        ]
      : []),
    {
      id: 'type',
      header: t('config.column.type'),
      accessorFn: (s) => s.type,
      cell: ({ row }) => <Badge variant="outline">{row.original.type}</Badge>,
    },
    { id: 'keys', header: t('config.column.keys'), accessorFn: (s) => s.keys.length },
    {
      id: 'age',
      header: t('config.column.age'),
      accessorFn: (s) => s.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];
}

function Secrets({ cluster }: { cluster: ClusterView }) {
  const [namespace, setNamespace] = useNamespace(cluster);
  const list = useQuery(configQueries.secrets(cluster.cluster.id, namespace));
  return (
    <ResourceList
      cluster={cluster}
      kind="secret"
      title={t('config.title')}
      summary={(count) => t('config.secretsSummary', { count })}
      list={list}
      namespace={namespace}
      onNamespace={setNamespace}
      columns={columns(namespace === null)}
      deletable={(s) => ({ type: { kind: 'secret' }, namespace: s.namespace, name: s.name })}
      getRowId={(s) => `${s.namespace}/${s.name}`}
      rowPath={(s) => objectPath('secret', s.namespace, s.name) ?? '/secrets'}
      search={t('config.searchSecrets')}
      sections={sections()}
      current="/secrets"
    />
  );
}

export function SecretsPage() {
  return (
    <ClusterGate>{(cluster) => <Secrets key={cluster.cluster.id} cluster={cluster} />}</ClusterGate>
  );
}
