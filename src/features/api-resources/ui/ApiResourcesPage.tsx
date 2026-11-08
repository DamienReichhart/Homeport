import { useQuery } from '@tanstack/react-query';
import { RefreshCwIcon } from 'lucide-react';
import { useParams } from 'react-router-dom';
import { ClusterGate, useNamespace } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { Button, ErrorMessage, PageHeader, Spinner } from '@/shared/ui';
import { apiQueries, CRDS } from '../api/queries';
import { useApiWatch } from '../api/useApiWatch';
import { findType } from '../model/types';
import { ObjectList } from './ObjectList';
import { TypePicker } from './TypePicker';
import { t } from '@/shared/i18n';

function ApiResources({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const { group = '', version = '', plural = '' } = useParams();
  const [namespace, setNamespace] = useNamespace(cluster);
  const types = useQuery(apiQueries.types(id, namespace));
  // A CRD added or removed changes the types.
  useApiWatch(id, CRDS, null, apiQueries.types(id, namespace).queryKey.slice(0, 3));
  const current = types.data && plural ? findType(types.data, group, version, plural) : undefined;
  return (
    <div className="grid gap-3">
      <PageHeader
        title={t('apiresources.title')}
        description={types.data && t('apiresources.summary', { count: types.data.length })}
        actions={
          <Button
            variant="outline"
            size="sm"
            onClick={() => void types.refetch()}
            disabled={types.isFetching}
          >
            <RefreshCwIcon /> {t('apiresources.refresh')}
          </Button>
        }
      />
      {types.isPending ? (
        <Spinner />
      ) : types.error ? (
        <ErrorMessage error={types.error} onRetry={() => void types.refetch()} />
      ) : (
        <div className="grid gap-4 md:grid-cols-[16rem_1fr]">
          <TypePicker types={types.data} current={current} />
          <div className="min-w-0">
            {current ? (
              <ObjectList
                cluster={cluster}
                info={current}
                namespace={namespace}
                onNamespace={setNamespace}
              />
            ) : plural ? (
              <p className="text-muted-foreground text-sm">{t('apiresources.typeGone')}</p>
            ) : (
              <p className="text-muted-foreground text-sm">{t('apiresources.pick')}</p>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

export function ApiResourcesPage() {
  return (
    <ClusterGate>
      {(cluster) => <ApiResources key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
