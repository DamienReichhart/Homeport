import { useQuery } from '@tanstack/react-query';
import { Link } from 'react-router-dom';
import type { ReleaseResource } from '@/shared/api';
import { DataTable } from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { resourcePath } from '../../model/paths';
import { t } from '@/shared/i18n';

/** The objects a revision's manifest creates, each opening its page. */
export function ReleaseResources({
  clusterId,
  namespace,
  resources,
}: {
  clusterId: string;
  namespace: string;
  resources: ReleaseResource[];
}) {
  const types = useQuery(helmQueries.apiTypes(clusterId));
  return (
    <DataTable<ReleaseResource>
      columns={[
        { id: 'kind', header: t('helm.resource.kind'), accessorFn: (r) => r.kind },
        {
          id: 'name',
          header: t('helm.resource.name'),
          accessorFn: (r) => r.name,
          cell: ({ row }) => {
            const path = resourcePath(row.original, namespace, types.data);
            return path ? (
              <Link to={path} className="font-medium underline-offset-4 hover:underline">
                {row.original.name}
              </Link>
            ) : (
              <span className="font-medium">{row.original.name}</span>
            );
          },
        },
        {
          id: 'namespace',
          header: t('helm.resource.namespace'),
          accessorFn: (r) => r.namespace ?? '',
          cell: ({ row }) => row.original.namespace ?? '—',
        },
        {
          id: 'apiVersion',
          header: t('helm.resource.apiVersion'),
          accessorFn: (r) => r.apiVersion,
          cell: ({ row }) => <span className="font-mono text-xs">{row.original.apiVersion}</span>,
        },
      ]}
      data={resources}
      getRowId={(r) => `${r.apiVersion}/${r.kind}/${r.namespace ?? ''}/${r.name}`}
      empty={t('helm.resource.empty')}
    />
  );
}
