import { clusterAddress } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, KeyValue } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** The selected Nomad Cluster at a glance: its version, region and address. */
export function NomadDashboard({ cluster }: { cluster: ClusterView }) {
  const status = cluster.status;
  const connected = status.state === 'connected' ? status : null;
  return (
    <div className="grid gap-4 md:grid-cols-2 xl:grid-cols-3">
      <Card>
        <CardHeader>
          <CardTitle>{t('overview.cluster.versionTitle')}</CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              [t('overview.cluster.version'), connected?.serverVersion || null],
              [t('overview.nomad.region'), connected?.region ?? null],
              [
                t('overview.nomad.address'),
                <span className="font-mono text-xs">{clusterAddress(cluster.cluster)}</span>,
              ],
            ]}
          />
        </CardContent>
      </Card>
    </div>
  );
}
