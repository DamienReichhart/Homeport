import type { ClusterNodeView } from '@/shared/api';
import { StatusDot } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Ready or not, and cordoned, as `kubectl get nodes` shows it. */
export function NodeStatus({ node }: { node: ClusterNodeView }) {
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={node.ready ? (node.unschedulable ? 'warning' : 'success') : 'danger'} />
      {node.ready ? t('nodes.ready') : t('nodes.notReady')}
      {node.unschedulable && <span className="text-muted-foreground">· {t('nodes.cordoned')}</span>}
    </span>
  );
}
