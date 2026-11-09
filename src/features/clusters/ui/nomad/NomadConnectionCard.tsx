import type { ReactNode } from 'react';
import type { ClusterView } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, KeyValue } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Where a Nomad Cluster's agent is and what Homeport sends it; never a credential. */
export function NomadConnectionCard({ view }: { view: ClusterView }) {
  const c = view.cluster.connection;
  if (c.kind !== 'nomad') return null;
  const status = view.status;
  const items: [string, ReactNode][] = [
    [t('clusters.connection.address'), <span className="font-mono">{c.address ?? ''}</span>],
  ];
  if (status.state === 'connected') {
    items.push([t('clusters.connection.version'), status.serverVersion || null]);
    items.push([t('clusters.connection.region'), status.region]);
  } else if (c.region) {
    items.push([t('clusters.connection.region'), c.region]);
  }
  items.push([
    t('clusters.connection.auth'),
    c.aclToken ? t('clusters.auth.aclToken') : t('clusters.auth.none'),
  ]);
  const tls = [
    c.customCa && t('clusters.connection.customCa'),
    c.clientCertificate && t('clusters.connection.clientCertificate'),
    c.tlsServerName && t('clusters.connection.checkedAs', { name: c.tlsServerName }),
  ].filter(Boolean);
  if (tls.length > 0) items.push([t('clusters.connection.tls'), tls.join(' · ')]);
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
