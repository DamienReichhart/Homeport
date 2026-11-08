import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { Link } from 'react-router-dom';
import { age, ClusterGate, kindPath, ResourceList, useNamespace } from '@/core/cluster';
import type { ClusterView, EventView } from '@/shared/api';
import { Badge, ToggleGroup, ToggleGroupItem, type ColumnDef } from '@/shared/ui';
import { clusterEventQueries } from '../api/queries';
import { byType, type EventType } from '../model/filter';
import { t } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<EventView, unknown>[] {
  return [
    {
      id: 'type',
      header: t('clusterevents.column.type'),
      accessorFn: (e) => e.type,
      cell: ({ row }) => (
        <Badge variant={row.original.type === 'Warning' ? 'destructive' : 'secondary'}>
          {row.original.type}
        </Badge>
      ),
    },
    { id: 'reason', header: t('clusterevents.column.reason'), accessorFn: (e) => e.reason },
    {
      id: 'object',
      header: t('clusterevents.column.object'),
      accessorFn: (e) => `${e.object.kind} ${e.object.name}`,
      cell: ({ row }) => {
        const o = row.original.object;
        const to = kindPath(o.kind, o.namespace, o.name);
        const label = `${o.kind}/${o.name}`;
        return to ? <Link to={to}>{label}</Link> : label;
      },
    },
    ...(showNamespace
      ? [
          {
            id: 'ns',
            header: t('clusterevents.column.namespace'),
            accessorFn: (e: EventView) => e.namespace,
          },
        ]
      : []),
    {
      id: 'message',
      header: t('clusterevents.column.message'),
      accessorFn: (e) => e.message,
      cell: ({ row }) => <span className="text-sm break-words">{row.original.message}</span>,
    },
    { id: 'count', header: t('clusterevents.column.count'), accessorFn: (e) => e.count },
    {
      id: 'last',
      header: t('clusterevents.column.lastSeen'),
      accessorFn: (e) => e.lastSeen ?? '',
      cell: ({ row }) => age(row.original.lastSeen),
    },
  ];
}

function ClusterEvents({ cluster }: { cluster: ClusterView }) {
  const [namespace, setNamespace] = useNamespace(cluster);
  const [type, setType] = useState<EventType>('all');
  const query = useQuery(clusterEventQueries.list(cluster.cluster.id, namespace));
  const list = { ...query, data: query.data && byType(query.data, type) } as typeof query;
  return (
    <ResourceList
      cluster={cluster}
      kind="event"
      title={t('clusterevents.title')}
      summary={(count) => t('clusterevents.summary', { count })}
      list={list}
      namespace={namespace}
      onNamespace={setNamespace}
      columns={columns(namespace === null)}
      getRowId={(e) => `${e.namespace}/${e.name}`}
      search={t('clusterevents.search')}
      empty={t('clusterevents.empty')}
      toolbar={
        <ToggleGroup
          type="single"
          variant="outline"
          value={type}
          onValueChange={(v) => v && setType(v as EventType)}
          aria-label={t('clusterevents.filter')}
        >
          <ToggleGroupItem value="all">{t('clusterevents.all')}</ToggleGroupItem>
          <ToggleGroupItem value="Warning">{t('clusterevents.warnings')}</ToggleGroupItem>
          <ToggleGroupItem value="Normal">{t('clusterevents.normal')}</ToggleGroupItem>
        </ToggleGroup>
      }
    />
  );
}

export function ClusterEventsPage() {
  return (
    <ClusterGate>
      {(cluster) => <ClusterEvents key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
