import { CircleSlashIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { Link } from 'react-router-dom';
import type { ClusterKind } from '@/shared/api';
import { Button, EmptyState } from '@/shared/ui';
import { CLUSTER_KIND_LABELS } from '../model/kind';
import { useClusterSelection } from '../selection/useClusterSelection';
import { t } from '@/shared/i18n';

/**
 * Shows the pages of a feature made for one `kind` of Cluster only while a
 * Cluster of that kind is selected, and says why otherwise, so a link or a
 * remembered page never opens one it can't show.
 */
export function ClusterKindGuard({ kind, children }: { kind: ClusterKind; children: ReactNode }) {
  const { cluster } = useClusterSelection();
  if (!cluster || cluster.cluster.kind === kind) return <>{children}</>;
  const name = cluster.cluster.name;
  return (
    <EmptyState
      icon={CircleSlashIcon}
      title={t('core.cluster.unsupported.title', { name })}
      description={t('core.cluster.unsupported.description', {
        name,
        kind: t(CLUSTER_KIND_LABELS[kind]),
        current: t(CLUSTER_KIND_LABELS[cluster.cluster.kind]),
      })}
      action={
        <div className="flex gap-2">
          <Button asChild variant="outline">
            <Link to="/">{t('core.unsupported.overview')}</Link>
          </Button>
          <Button asChild>
            <Link to={`/clusters/${cluster.cluster.id}`}>
              {t('core.cluster.gate.openSettings')}
            </Link>
          </Button>
        </div>
      }
    />
  );
}
