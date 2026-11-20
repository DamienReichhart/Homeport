import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { Link } from 'react-router-dom';
import { age, ClusterGate, kindPath, ResourceList, useNamespace } from '@/core/cluster';
import type { ClusterView, EventView } from '@/shared/api';
import { Badge, Tabs, TabsList, TabsTrigger, type ColumnDef } from '@/shared/ui';
import { clusterEventQueries } from '../api/queries';
import { byType, type EventType } from '../model/filter';
import { t, type MessageKey } from '@/shared/i18n';

function columns(showNamespace: boolean): ColumnDef<EventView, unknown>[] {
  return [
    {
      id: 'type',
      header: t('clusterevents.column.type'),
      accessorFn: (e) => e.type,
      // A warning is worth a look; a normal Event is the Cluster doing its job.
      cell: ({ row }) =>
        row.original.type === 'Warning' ? (
          <Badge variant="warning">{row.original.type}</Badge>
        ) : (
          <span className="text-muted-foreground text-xs">{row.original.type}</span>
        ),
    },
    {
      id: 'reason',
      header: t('clusterevents.column.reason'),
      accessorFn: (e) => e.reason,
      cell: ({ row }) => (
        <span className="inline-block max-w-48 truncate align-bottom" title={row.original.reason}>
          {row.original.reason}
        </span>
      ),
    },
    {
      id: 'object',
      header: t('clusterevents.column.object'),
      accessorFn: (e) => `${e.object.kind} ${e.object.name}`,
      cell: ({ row }) => {
        const o = row.original.object;
        const to = kindPath(o.kind, o.namespace, o.name);
        const label = `${o.kind}/${o.name}`;
        return (
          <span className="inline-block max-w-52 truncate align-bottom" title={label}>
            {to ? <Link to={to}>{label}</Link> : label}
          </span>
        );
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
      cell: ({ row }) => (
        <span className="inline-block max-w-sm truncate align-bottom" title={row.original.message}>
          {row.original.message}
        </span>
      ),
    },
    {
      id: 'count',
      header: t('clusterevents.column.count'),
      accessorFn: (e) => e.count,
      cell: ({ row }) => <span className="tabular-nums">{row.original.count}</span>,
    },
    {
      id: 'last',
      header: t('clusterevents.column.lastSeen'),
      accessorFn: (e) => e.lastSeen ?? '',
      cell: ({ row }) => age(row.original.lastSeen),
    },
  ];
}

const LABELS: Record<EventType, MessageKey> = {
  all: 'clusterevents.all',
  Warning: 'clusterevents.warnings',
  Normal: 'clusterevents.normal',
};

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
        <Tabs value={type} onValueChange={(v) => setType(v as EventType)}>
          <TabsList variant="default" aria-label={t('clusterevents.filter')}>
            {(['all', 'Warning', 'Normal'] as const).map((value) => (
              <TabsTrigger
                key={value}
                value={value}
                // A tab that switches what one view shows, without a panel of its own.
                aria-controls={undefined}
              >
                {t(LABELS[value])}
              </TabsTrigger>
            ))}
          </TabsList>
        </Tabs>
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
