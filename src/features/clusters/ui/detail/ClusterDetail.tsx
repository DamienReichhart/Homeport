import { RefreshCwIcon } from 'lucide-react';
import {
  CLUSTER_KIND_LABELS,
  clusterStatusError,
  ClusterStatusBadge,
  describeClusterStatus,
} from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { Badge, Button, ErrorMessage, Label, PageHeader, Switch } from '@/shared/ui';
import { useClusterConnection } from '../../api/mutations';
import { ConnectionCard } from './ConnectionCard';
import { RefreshCard } from './RefreshCard';
import { RemoveClusterCard } from './RemoveClusterCard';
import { RenameCard } from './RenameCard';
import { t } from '@/shared/i18n';

/** A Cluster's status, how it is reached, and renaming, refreshing or removing it. */
export function ClusterDetail({ view }: { view: ClusterView }) {
  const { reconnect, enable } = useClusterConnection(view.cluster.id);
  const { detail } = describeClusterStatus(view.status);
  const error = clusterStatusError(view.status);
  return (
    <div className="grid max-w-3xl gap-4">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            {view.cluster.name}{' '}
            <Badge variant="outline">{t(CLUSTER_KIND_LABELS[view.cluster.kind])}</Badge>
          </span>
        }
        description={
          <span className="flex items-center gap-2">
            <ClusterStatusBadge status={view.status} />
            {detail && !error && <span className="text-muted-foreground">· {detail}</span>}
          </span>
        }
        actions={
          <>
            <div className="flex items-center gap-2">
              <Switch
                id="enabled"
                checked={view.cluster.enabled}
                onCheckedChange={(v) => enable.mutate(v)}
              />
              <Label htmlFor="enabled">{t('clusters.detail.enabled')}</Label>
            </div>
            <Button
              variant="outline"
              disabled={reconnect.isPending}
              onClick={() => reconnect.mutate()}
            >
              <RefreshCwIcon /> {t('clusters.detail.reconnect')}
            </Button>
          </>
        }
      />
      {error && <ErrorMessage error={error} onRetry={() => reconnect.mutate()} />}
      <ConnectionCard view={view} />
      <RefreshCard view={view} />
      <RenameCard view={view} />
      <RemoveClusterCard view={view} />
    </div>
  );
}
