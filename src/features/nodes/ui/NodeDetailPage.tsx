import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { useParams } from 'react-router-dom';
import {
  age,
  ClusterGate,
  ConditionList,
  Labels,
  ObjectEvents,
  objectPath,
  ResourceTabs,
  ResourceYaml,
  useResourceWatch,
} from '@/core/cluster';
import { RelatedPods } from '@/features/pods';
import type { ClusterNodeDetail, ClusterView } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, DataTable, KeyValue } from '@/shared/ui';
import { nodeQueries } from '../api/queries';
import { readableQuantity } from '../model/quantity';
import { NodeStatus } from './NodeStatus';
import { t } from '@/shared/i18n';

function Overview({ node }: { node: ClusterNodeDetail }) {
  const v = node.view;
  const items: [string, ReactNode][] = [
    [t('nodes.detail.roles'), v.roles.join(', ') || null],
    [t('nodes.detail.version'), v.version],
    [t('nodes.detail.os'), node.osImage],
    [t('nodes.detail.kernel'), node.kernel],
    [t('nodes.detail.runtime'), node.containerRuntime],
    [t('nodes.detail.addresses'), node.addresses.join(', ') || null],
    [
      t('nodes.detail.taints'),
      node.taints.length
        ? node.taints.map((x) => `${x.key}${x.value ? `=${x.value}` : ''}:${x.effect}`).join(', ')
        : t('nodes.detail.noTaints'),
    ],
    [t('nodes.detail.age'), age(v.created)],
    [t('nodes.detail.labels'), <Labels values={node.labels} />],
  ];
  const resources = Object.keys({ ...node.capacity, ...node.allocatable }).sort();
  return (
    <div className="grid gap-4">
      <Card>
        <CardContent>
          <KeyValue items={items} />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('nodes.detail.resources')}</CardTitle>
        </CardHeader>
        <CardContent>
          <DataTable
            columns={[
              { id: 'name', header: t('nodes.resource.name'), accessorFn: (r) => r },
              {
                id: 'capacity',
                header: t('nodes.resource.capacity'),
                accessorFn: (r) => readableQuantity(r, node.capacity[r]),
              },
              {
                id: 'allocatable',
                header: t('nodes.resource.allocatable'),
                accessorFn: (r) => readableQuantity(r, node.allocatable[r]),
              },
            ]}
            data={resources}
            getRowId={(r) => r}
          />
        </CardContent>
      </Card>
    </div>
  );
}

function NodeDetailView({ cluster }: { cluster: ClusterView }) {
  const { name = '' } = useParams();
  const id = cluster.cluster.id;
  const detail = useQuery(nodeQueries.detail(id, name));
  useResourceWatch(id, 'node', null);
  return (
    <ResourceTabs
      detail={detail}
      base={objectPath('node', null, name) ?? '/nodes'}
      title={name}
      description={(n) => <NodeStatus node={n.view} />}
      tabs={(n) => [
        { value: 'overview', label: t('nodes.tab.overview'), content: <Overview node={n} /> },
        {
          value: 'pods',
          label: t('nodes.tab.pods'),
          content: <RelatedPods clusterId={id} namespace={null} node={name} />,
        },
        {
          value: 'conditions',
          label: t('nodes.tab.conditions'),
          content: <ConditionList conditions={n.conditions} />,
        },
        {
          value: 'events',
          label: t('nodes.tab.events'),
          content: <ObjectEvents clusterId={id} kind="node" namespace={null} name={name} />,
        },
        {
          value: 'source',
          label: t('nodes.tab.source'),
          content: <ResourceYaml clusterId={id} kind="node" namespace={null} name={name} />,
        },
      ]}
    />
  );
}

export function NodeDetailPage() {
  return <ClusterGate>{(cluster) => <NodeDetailView cluster={cluster} />}</ClusterGate>;
}
