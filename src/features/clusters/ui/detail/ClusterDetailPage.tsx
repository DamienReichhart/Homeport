import { useParams } from 'react-router-dom';
import { useClusters } from '@/core/cluster';
import { ErrorMessage, Spinner } from '@/shared/ui';
import { ClusterDetail } from './ClusterDetail';
import { t } from '@/shared/i18n';

export function ClusterDetailPage() {
  const { clusterId = '' } = useParams();
  const clusters = useClusters();
  const view = clusters.data?.find((c) => c.cluster.id === clusterId);
  if (clusters.isPending) return <Spinner />;
  if (!view) return <ErrorMessage error={{ code: 'not_found', message: t('clusters.notFound') }} />;
  return <ClusterDetail key={clusterId} view={view} />;
}
