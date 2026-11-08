import { useQuery } from '@tanstack/react-query';
import type { ResourceKind } from '@/shared/api';
import { Badge, DataTable, ErrorMessage, Spinner, type ColumnDef } from '@/shared/ui';
import type { EventView } from '@/shared/api';
import { clusterResourceQueries } from '../api/resources';
import { useResourceWatch } from '../live/useResourceWatch';
import { age } from '../model/health';
import { t } from '@/shared/i18n';

const columns: ColumnDef<EventView, unknown>[] = [
  {
    id: 'type',
    header: t('core.events.type'),
    accessorFn: (e) => e.type,
    cell: ({ row }) => (
      <Badge variant={row.original.type === 'Warning' ? 'destructive' : 'secondary'}>
        {row.original.type}
      </Badge>
    ),
  },
  { id: 'reason', header: t('core.events.reason'), accessorFn: (e) => e.reason },
  {
    id: 'message',
    header: t('core.events.message'),
    accessorFn: (e) => e.message,
    cell: ({ row }) => <span className="text-sm break-words">{row.original.message}</span>,
  },
  { id: 'count', header: t('core.events.count'), accessorFn: (e) => e.count },
  {
    id: 'last',
    header: t('core.events.lastSeen'),
    accessorFn: (e) => e.lastSeen ?? '',
    cell: ({ row }) => age(row.original.lastSeen),
  },
];

/** The Events about one object, warnings first, kept fresh by a watch. */
export function ObjectEvents({
  clusterId,
  kind,
  namespace,
  name,
}: {
  clusterId: string;
  /** One of Homeport's kinds, or any Kubernetes kind by name (`Certificate`). */
  kind: ResourceKind | { name: string };
  namespace: string | null;
  name: string;
}) {
  const known = typeof kind === 'string';
  const ofKind = useQuery({
    ...clusterResourceQueries.objectEvents(clusterId, known ? kind : 'event', namespace, name),
    enabled: known,
  });
  const ofName = useQuery({
    ...clusterResourceQueries.eventsOfKind(clusterId, known ? '' : kind.name, namespace, name),
    enabled: !known,
  });
  const events = known ? ofKind : ofName;
  useResourceWatch(clusterId, 'event', namespace);
  if (events.isPending) return <Spinner />;
  if (events.error)
    return <ErrorMessage error={events.error} onRetry={() => void events.refetch()} />;
  return (
    <DataTable
      columns={columns}
      data={events.data}
      getRowId={(e) => e.name}
      empty={t('core.events.none')}
    />
  );
}
