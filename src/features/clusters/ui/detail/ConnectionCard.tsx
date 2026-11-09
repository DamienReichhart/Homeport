import type { ReactNode } from 'react';
import { kubernetesConnection } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, KeyValue } from '@/shared/ui';
import { authLabel } from '../../model/labels';
import { t } from '@/shared/i18n';

/** Where a Kubernetes Cluster's API server is and how Homeport authenticates; never a credential. */
export function ConnectionCard({ view }: { view: ClusterView }) {
  const c = kubernetesConnection(view.cluster);
  const status = view.status;
  if (!c) return null;
  const items: [string, ReactNode][] = [
    [t('clusters.connection.server'), <span className="font-mono">{c.server ?? ''}</span>],
  ];
  if (status.state === 'connected') {
    items.push([t('clusters.connection.version'), status.serverVersion]);
    items.push([t('clusters.connection.platform'), status.platform]);
  }
  items.push([t('clusters.connection.namespace'), c.namespace ?? 'default']);
  items.push([t('clusters.connection.auth'), authLabel(c.auth)]);
  if (c.tlsServerName) items.push([t('clusters.connection.tlsServerName'), c.tlsServerName]);
  if (c.insecureSkipTlsVerify)
    items.push([t('clusters.connection.tls'), t('clusters.connection.insecure')]);
  if (c.proxyUrl) items.push([t('clusters.connection.proxy'), c.proxyUrl]);
  items.push([
    t('clusters.connection.source'),
    c.source
      ? t(c.source.pasted ? 'clusters.source.pasted' : 'clusters.source.machine', {
          context: c.source.context,
        })
      : null,
  ]);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('clusters.connection.title')}</CardTitle>
      </CardHeader>
      <CardContent>
        <KeyValue items={items} />
      </CardContent>
    </Card>
  );
}
