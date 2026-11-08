import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { Link, useParams } from 'react-router-dom';
import {
  age,
  ClusterGate,
  Labels,
  ObjectEvents,
  objectPath,
  ResourceTabs,
  ResourceYaml,
  useResourceWatch,
} from '@/core/cluster';
import type { ClusterView, IngressDetail } from '@/shared/api';
import { Badge, Card, CardContent, DataTable, KeyValue } from '@/shared/ui';
import { networkQueries } from '../api/queries';
import { t } from '@/shared/i18n';

function Overview({ ingress }: { ingress: IngressDetail }) {
  const v = ingress.view;
  const items: [string, ReactNode][] = [
    [t('network.detail.namespace'), v.namespace],
    [t('network.detail.class'), v.class],
    [t('network.detail.addresses'), v.addresses.join(', ') || null],
    [
      t('network.detail.tls'),
      ingress.tls.map((x) => `${x.hosts.join(', ') || '*'} → ${x.secret ?? '—'}`).join('; ') ||
        null,
    ],
    [t('network.detail.age'), age(v.created)],
    [t('network.detail.labels'), <Labels values={ingress.labels} />],
    [t('network.detail.annotations'), <Labels values={ingress.annotations} />],
  ];
  return (
    <Card>
      <CardContent>
        <KeyValue items={items} />
      </CardContent>
    </Card>
  );
}

function Rules({ ingress }: { ingress: IngressDetail }) {
  const ns = ingress.view.namespace;
  return (
    <DataTable
      columns={[
        { id: 'host', header: t('network.rule.host'), accessorFn: (p) => p.host },
        {
          id: 'path',
          header: t('network.rule.path'),
          accessorFn: (p) => p.path,
          cell: ({ row }) => <code className="text-xs">{row.original.path}</code>,
        },
        { id: 'type', header: t('network.rule.type'), accessorFn: (p) => p.pathType },
        {
          id: 'backend',
          header: t('network.rule.backend'),
          accessorFn: (p) => p.service ?? p.resource ?? '',
          cell: ({ row }) => {
            const p = row.original;
            if (!p.service) return p.resource;
            const to = objectPath('service', ns, p.service);
            const label = p.port ? `${p.service}:${p.port}` : p.service;
            return to ? <Link to={to}>{label}</Link> : label;
          },
        },
      ]}
      data={ingress.paths}
      getRowId={(p) => `${p.host} ${p.path} ${p.service ?? p.resource ?? ''}`}
    />
  );
}

function IngressDetailView({ cluster }: { cluster: ClusterView }) {
  const { namespace = '', name = '' } = useParams();
  const id = cluster.cluster.id;
  const detail = useQuery(networkQueries.ingress(id, namespace, name));
  useResourceWatch(id, 'ingress', namespace);
  return (
    <ResourceTabs
      detail={detail}
      base={objectPath('ingress', namespace, name) ?? '/ingresses'}
      title={
        <span className="flex items-center gap-2">
          {name} <Badge variant="outline">{t('network.ingress')}</Badge>
        </span>
      }
      tabs={(i) => [
        { value: 'overview', label: t('network.tab.overview'), content: <Overview ingress={i} /> },
        { value: 'rules', label: t('network.tab.rules'), content: <Rules ingress={i} /> },
        {
          value: 'events',
          label: t('network.tab.events'),
          content: <ObjectEvents clusterId={id} kind="ingress" namespace={namespace} name={name} />,
        },
        {
          value: 'source',
          label: t('network.tab.source'),
          content: <ResourceYaml clusterId={id} kind="ingress" namespace={namespace} name={name} />,
        },
      ]}
    />
  );
}

export function IngressDetailPage() {
  return <ClusterGate>{(cluster) => <IngressDetailView cluster={cluster} />}</ClusterGate>;
}
