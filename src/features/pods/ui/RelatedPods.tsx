import { useQuery } from '@tanstack/react-query';
import { useResourceWatch } from '@/core/cluster';
import { ErrorMessage, Spinner } from '@/shared/ui';
import { podQueries } from '../api/queries';
import { PodsTable } from './PodsTable';

/**
 * The Pods a workload or a Service selects (`selector`), or that run on a
 * `node`, kept fresh by a watch.
 */
export function RelatedPods({
  clusterId,
  namespace,
  selector,
  node,
}: {
  clusterId: string;
  namespace: string | null;
  selector?: string;
  node?: string;
}) {
  const pods = useQuery(podQueries.list(clusterId, namespace, { selector, node }));
  useResourceWatch(clusterId, 'pod', namespace);
  if (pods.isPending) return <Spinner />;
  if (pods.error) return <ErrorMessage error={pods.error} onRetry={() => void pods.refetch()} />;
  return <PodsTable pods={pods.data} showNamespace={namespace === null} />;
}
