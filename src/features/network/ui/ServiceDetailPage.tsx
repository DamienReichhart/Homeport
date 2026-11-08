import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { useParams } from 'react-router-dom';
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
import { RelatedPods } from '@/features/pods';
import type { ClusterServiceDetail, ClusterView } from '@/shared/api';
import { Badge, Card, CardContent, DataTable, KeyValue } from '@/shared/ui';
import { networkQueries } from '../api/queries';
import { t } from '@/shared/i18n';

function Overview({ service }: { service: ClusterServiceDetail }) {
  const v = service.view;
  const items: [string, ReactNode][] = [
    [t('network.detail.namespace'), v.namespace],
    [t('network.detail.type'), v.type],
    [t('network.detail.clusterIp'), v.clusterIp],
    [t('network.detail.external'), v.external.join(', ') || null],
    [
      t('network.detail.selector'),
      v.selector ? <code className="text-xs">{v.selector}</code> : t('network.detail.noSelector'),
    ],
    [t('network.detail.affinity'), service.sessionAffinity],
    [t('network.detail.age'), age(v.created)],
    [t('network.detail.labels'), <Labels values={service.labels} />],
  ];
  return (
    <div className="grid gap-4">
      <Card>
        <CardContent>
          <KeyValue items={items} />
        </CardContent>
      </Card>
      <DataTable
        columns={[
          { id: 'name', header: t('network.port.name'), accessorFn: (p) => p.name ?? '' },
          {
            id: 'port',
            header: t('network.port.port'),
            accessorFn: (p) => `${p.port}/${p.protocol}`,
          },
          { id: 'target', header: t('network.port.target'), accessorFn: (p) => p.targetPort },
          { id: 'node', header: t('network.port.node'), accessorFn: (p) => p.nodePort ?? '' },
        ]}
        data={service.ports}
        getRowId={(p) => `${p.port}/${p.protocol}`}
      />
    </div>
  );
}

function ServiceDetailView({ cluster }: { cluster: ClusterView }) {
  const { namespace = '', name = '' } = useParams();
  const id = cluster.cluster.id;
  const detail = useQuery(networkQueries.service(id, namespace, name));
  useResourceWatch(id, 'service', namespace);
  return (
    <ResourceTabs
      detail={detail}
      deletable={{
        cluster,
        target: { type: { kind: 'service' }, namespace: namespace, name },
        listPath: '/services',
      }}
      base={objectPath('service', namespace, name) ?? '/services'}
      title={
        <span className="flex items-center gap-2">
          {name} <Badge variant="outline">{t('network.service')}</Badge>
        </span>
      }
      description={(s) => s.view.type}
      tabs={(s) => [
        { value: 'overview', label: t('network.tab.overview'), content: <Overview service={s} /> },
        {
          value: 'pods',
          label: t('network.tab.pods'),
          content: s.view.selector ? (
            <RelatedPods clusterId={id} namespace={namespace} selector={s.view.selector} />
          ) : (
            <p className="text-muted-foreground text-sm">{t('network.detail.noSelector')}</p>
          ),
        },
        {
          value: 'events',
          label: t('network.tab.events'),
          content: <ObjectEvents clusterId={id} kind="service" namespace={namespace} name={name} />,
        },
        {
          value: 'source',
          label: t('network.tab.source'),
          content: <ResourceYaml clusterId={id} kind="service" namespace={namespace} name={name} />,
        },
      ]}
    />
  );
}

export function ServiceDetailPage() {
  return <ClusterGate>{(cluster) => <ServiceDetailView cluster={cluster} />}</ClusterGate>;
}
