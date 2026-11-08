import { useQuery } from '@tanstack/react-query';
import { useParams } from 'react-router-dom';
import {
  ClusterGate,
  Labels,
  ObjectEvents,
  objectPath,
  ResourceTabs,
  ResourceYaml,
  useResourceWatch,
} from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { Badge, Card, CardContent, KeyValue } from '@/shared/ui';
import { configQueries } from '../api/queries';
import { SecretValues } from './SecretValues';
import { t } from '@/shared/i18n';

function ConfigMapView({ cluster }: { cluster: ClusterView }) {
  const { namespace = '', name = '' } = useParams();
  const id = cluster.cluster.id;
  const detail = useQuery(configQueries.configMap(id, namespace, name));
  useResourceWatch(id, 'configMap', namespace);
  return (
    <ResourceTabs
      detail={detail}
      deletable={{
        cluster,
        target: { type: { kind: 'configMap' }, namespace, name },
        listPath: '/configmaps',
      }}
      base={objectPath('configMap', namespace, name) ?? '/configmaps'}
      title={
        <span className="flex items-center gap-2">
          {name} <Badge variant="outline">{t('config.configMap')}</Badge>
        </span>
      }
      tabs={(c) => [
        {
          value: 'data',
          label: t('config.tab.data'),
          content: (
            <div className="grid gap-3">
              {c.values.length === 0 && (
                <p className="text-muted-foreground text-sm">{t('config.noKeys')}</p>
              )}
              {c.values.map((v) => (
                <div key={v.key} className="grid gap-1">
                  <span className="flex items-center gap-2">
                    <span className="font-mono text-sm font-medium">{v.key}</span>
                    <span className="text-muted-foreground text-xs">{formatBytes(v.size)}</span>
                  </span>
                  <pre className="bg-muted max-h-80 overflow-auto rounded p-2 font-mono text-xs whitespace-pre-wrap">
                    {v.value ?? t('config.binary')}
                  </pre>
                </div>
              ))}
            </div>
          ),
        },
        {
          value: 'overview',
          label: t('config.tab.overview'),
          content: (
            <Card>
              <CardContent>
                <KeyValue
                  items={[
                    [t('config.detail.namespace'), c.view.namespace],
                    [t('config.detail.labels'), <Labels values={c.labels} />],
                    [t('config.detail.annotations'), <Labels values={c.annotations} />],
                  ]}
                />
              </CardContent>
            </Card>
          ),
        },
        {
          value: 'events',
          label: t('config.tab.events'),
          content: (
            <ObjectEvents clusterId={id} kind="configMap" namespace={namespace} name={name} />
          ),
        },
        {
          value: 'source',
          label: t('config.tab.source'),
          content: (
            <ResourceYaml clusterId={id} kind="configMap" namespace={namespace} name={name} />
          ),
        },
      ]}
    />
  );
}

function SecretView({ cluster }: { cluster: ClusterView }) {
  const { namespace = '', name = '' } = useParams();
  const id = cluster.cluster.id;
  const detail = useQuery(configQueries.secret(id, namespace, name));
  useResourceWatch(id, 'secret', namespace);
  return (
    <ResourceTabs
      detail={detail}
      deletable={{
        cluster,
        target: { type: { kind: 'secret' }, namespace, name },
        listPath: '/secrets',
      }}
      base={objectPath('secret', namespace, name) ?? '/secrets'}
      title={
        <span className="flex items-center gap-2">
          {name} <Badge variant="outline">{t('config.secret')}</Badge>
        </span>
      }
      description={(s) => s.view.type}
      tabs={(s) => [
        {
          value: 'data',
          label: t('config.tab.data'),
          content: (
            <SecretValues clusterId={id} namespace={namespace} name={name} keys={s.view.keys} />
          ),
        },
        {
          value: 'overview',
          label: t('config.tab.overview'),
          content: (
            <Card>
              <CardContent>
                <KeyValue
                  items={[
                    [t('config.detail.namespace'), s.view.namespace],
                    [t('config.detail.type'), s.view.type],
                    [t('config.detail.labels'), <Labels values={s.labels} />],
                    [t('config.detail.annotations'), <Labels values={s.annotations} />],
                  ]}
                />
              </CardContent>
            </Card>
          ),
        },
        {
          value: 'events',
          label: t('config.tab.events'),
          content: <ObjectEvents clusterId={id} kind="secret" namespace={namespace} name={name} />,
        },
        {
          value: 'source',
          label: t('config.tab.source'),
          content: <ResourceYaml clusterId={id} kind="secret" namespace={namespace} name={name} />,
        },
      ]}
    />
  );
}

export function ConfigMapDetailPage() {
  return <ClusterGate>{(cluster) => <ConfigMapView cluster={cluster} />}</ClusterGate>;
}

export function SecretDetailPage() {
  return <ClusterGate>{(cluster) => <SecretView cluster={cluster} />}</ClusterGate>;
}
