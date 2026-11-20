import { PlusIcon, ShipWheelIcon } from 'lucide-react';
import { Link, useNavigate } from 'react-router-dom';
import { useClusters } from '@/core/cluster';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  WithShortcut,
} from '@/shared/ui';
import { clusterColumns } from './clusterColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

/** Every Cluster, opened beside the list; adding one is the page's action. */
export function ClustersPage() {
  const clusters = useClusters();
  const navigate = useNavigate();
  usePageAction(t('clusters.add'), () => void navigate('/clusters/new'));
  return (
    <div>
      <PageHeader
        title={t('clusters.title')}
        description={t('clusters.description')}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('clusters.add')}>
            <Button asChild>
              <Link to="/clusters/new">
                <PlusIcon /> {t('clusters.add')}
              </Link>
            </Button>
          </WithShortcut>
        }
      />
      {clusters.error ? (
        <ErrorMessage error={clusters.error} onRetry={() => void clusters.refetch()} />
      ) : clusters.isPending ? (
        <Spinner />
      ) : clusters.data.length === 0 ? (
        <EmptyState
          icon={ShipWheelIcon}
          title={t('clusters.empty.title')}
          description={t('clusters.empty.description')}
          action={
            <Button variant="outline" asChild>
              <Link to="/clusters/new">
                <PlusIcon /> {t('clusters.add')}
              </Link>
            </Button>
          }
        />
      ) : (
        <DataTable
          columns={clusterColumns()}
          data={clusters.data}
          getRowId={(c) => c.cluster.id}
          onRowClick={(c) => void navigate(`/clusters/${c.cluster.id}`)}
        />
      )}
    </div>
  );
}
