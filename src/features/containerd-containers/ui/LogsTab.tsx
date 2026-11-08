import { useQuery } from '@tanstack/react-query';
import { ScrollTextIcon } from 'lucide-react';
import { useMemo } from 'react';
import { LogViewer } from '@/features/containers';
import { EmptyState, ErrorMessage, Spinner } from '@/shared/ui';
import { containerdContainerQueries } from '../api/queries';
import { containerdLogs } from '../api/streams';
import { t } from '@/shared/i18n';

/**
 * A containerd container's logs, where the tool that made it wrote them,
 * or why there are none: containerd keeps no logs of its own.
 */
export function LogsTab({
  hostId,
  namespace,
  id,
}: {
  hostId: string;
  namespace: string;
  id: string;
}) {
  const source = useQuery(containerdContainerQueries.logsSource(hostId, namespace, id));
  const stream = useMemo(() => containerdLogs(hostId, namespace, id), [hostId, namespace, id]);
  if (source.isPending) return <Spinner />;
  if (source.error)
    return <ErrorMessage error={source.error} onRetry={() => void source.refetch()} />;
  switch (source.data.kind) {
    case 'none':
      return (
        <EmptyState
          icon={ScrollTextIcon}
          title={t('containerd.logs.none.title')}
          description={t('containerd.logs.none.description')}
        />
      );
    case 'driver':
      return (
        <EmptyState
          icon={ScrollTextIcon}
          title={t('containerd.logs.driver.title', { driver: source.data.driver })}
          description={t('containerd.logs.driver.description', { driver: source.data.driver })}
        />
      );
    default:
      return (
        <div className="flex h-full min-h-0 flex-col gap-1">
          <p className="text-muted-foreground font-mono text-xs">
            {t('containerd.logs.from', { path: source.data.path })}
          </p>
          <LogViewer stream={stream} streamKey={`${hostId}/${namespace}/${id}`} />
        </div>
      );
  }
}
