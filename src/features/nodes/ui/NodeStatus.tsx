import type { ClusterNodeView } from '@/shared/api';
import { StatusDot } from '@/shared/ui';
import { cn } from '@/shared/lib';
import { t } from '@/shared/i18n';

/**
 * Ready or not, and cordoned, as `kubectl get nodes` shows it; a Node that
 * isn't ready is in red. Cut short in a narrow list, with the whole on hover.
 */
export function NodeStatus({ node }: { node: ClusterNodeView }) {
  const state = node.ready ? t('nodes.ready') : t('nodes.notReady');
  return (
    <span
      className={cn(
        'inline-flex max-w-full items-center gap-1.5 text-sm',
        !node.ready && 'text-destructive font-medium',
      )}
      title={node.unschedulable ? `${state} · ${t('nodes.cordoned')}` : undefined}
    >
      <StatusDot tone={node.ready ? (node.unschedulable ? 'warning' : 'success') : 'danger'} />
      <span className="shrink-0">{state}</span>
      {node.unschedulable && (
        <span className="text-muted-foreground truncate font-normal">· {t('nodes.cordoned')}</span>
      )}
    </span>
  );
}
