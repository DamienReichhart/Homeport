import { describeClusterStatus } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { cn } from '@/shared/lib/utils';
import { t } from '@/shared/i18n';

/** "Kubernetes v1.34.0" while connected, or what is wrong, with the reason in its tooltip. */
export function ClusterState({ view }: { view: ClusterView }) {
  const { label, tone, detail } = describeClusterStatus(view.status, view.cluster.kind);
  if (view.status.state === 'connected') {
    return (
      <span className="text-muted-foreground block max-w-[14rem] truncate" title={detail}>
        <span className="sr-only">{label} · </span>
        {detail}
      </span>
    );
  }
  return (
    <span
      className={cn(
        'block max-w-[16rem] truncate',
        tone === 'danger' && 'text-destructive',
        tone === 'warning' && 'text-warning',
        tone === 'muted' && 'text-muted-foreground',
      )}
      title={detail ? t('clusters.statusTitle', { label, detail }) : label}
    >
      {label}
    </span>
  );
}
