import { useQuery } from '@tanstack/react-query';
import { age, ClusterGate, objectPath, ResourceList, useNamespace } from '@/core/cluster';
import type { ClusterView, ConfigMapView } from '@/shared/api';
import { type ColumnDef } from '@/shared/ui';
import { configQueries } from '../api/queries';
import { sections } from './sections';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<ConfigMapView, unknown>[] {
  return [
    {
      id: 'name',
      header: t('config.column.name'),
      accessorFn: (c) => c.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('config.column.namespace'),
            accessorFn: (c: ConfigMapView) => c.namespace,
          },
        ]
      : []),
    { id: 'keys', header: t('config.column.keys'), accessorFn: (c) => c.keys.length },
    {
      id: 'age',
      header: t('config.column.age'),
      accessorFn: (c) => c.created ?? '',
      cell: ({ row }) => age(row.original.created),
    },
  ];
}

function ConfigMaps({ cluster }: { cluster: ClusterView }) {
  const [namespace, setNamespace] = useNamespace(cluster);
  const list = useQuery(configQueries.configMaps(cluster.cluster.id, namespace));
  return (
    <ResourceList
      cluster={cluster}
      kind="configMap"
      title={t('config.title')}
      summary={(count) => t('config.configMapsSummary', { count })}
      list={list}
      namespace={namespace}
      onNamespace={setNamespace}
      columns={columns(namespace === null)}
      getRowId={(c) => `${c.namespace}/${c.name}`}
      rowPath={(c) => objectPath('configMap', c.namespace, c.name) ?? '/configmaps'}
      search={t('config.searchConfigMaps')}
      sections={sections()}
      current="/configmaps"
    />
  );
}

export function ConfigMapsPage() {
  return (
    <ClusterGate>
      {(cluster) => <ConfigMaps key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
