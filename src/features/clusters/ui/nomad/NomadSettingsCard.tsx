import type { ClusterView } from '@/shared/api';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui';
import { useUpdateNomadCluster } from '../../api/mutations';
import { NomadClusterForm } from './NomadClusterForm';
import { t } from '@/shared/i18n';

/** Changes a Nomad Cluster's name, address, token and TLS material. */
export function NomadSettingsCard({ view }: { view: ClusterView }) {
  const update = useUpdateNomadCluster(view.cluster.id);
  const connection = view.cluster.connection;
  if (connection.kind !== 'nomad') return null;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('clusters.nomad.settings')}</CardTitle>
        <CardDescription>{t('clusters.nomad.settingsDescription')}</CardDescription>
      </CardHeader>
      <CardContent>
        <NomadClusterForm
          clusterId={view.cluster.id}
          name={view.cluster.name}
          connection={connection}
          submitLabel={t('clusters.nomad.save')}
          pending={update.isPending}
          error={update.error}
          onSubmit={(form) => update.mutate(form)}
        />
      </CardContent>
    </Card>
  );
}
