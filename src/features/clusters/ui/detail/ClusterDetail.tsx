import { EllipsisVerticalIcon, PencilIcon, RefreshCwIcon } from 'lucide-react';
import { useState } from 'react';
import {
  CLUSTER_KIND_LABELS,
  clusterStatusError,
  ClusterStatusBadge,
  describeClusterStatus,
} from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import {
  Badge,
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  ErrorMessage,
  Label,
  PageHeader,
  Switch,
} from '@/shared/ui';
import { useClusterConnection } from '../../api/mutations';
import { NomadConnectionCard } from '../nomad/NomadConnectionCard';
import { NomadSettingsCard } from '../nomad/NomadSettingsCard';
import { ConnectionCard } from './ConnectionCard';
import { RefreshCard } from './RefreshCard';
import { RemoveClusterCard } from './RemoveClusterCard';
import { RenameDialog } from './RenameDialog';
import { t } from '@/shared/i18n';

/**
 * A Cluster's page: its status and what fixes a failing connection first,
 * then how it is reached, and removing it at the foot. Renaming a
 * Kubernetes Cluster is in the "More actions" menu; a Nomad Cluster's name
 * is among its settings.
 */
export function ClusterDetail({ view }: { view: ClusterView }) {
  const { reconnect, enable } = useClusterConnection(view.cluster.id);
  const { detail } = describeClusterStatus(view.status, view.cluster.kind);
  const error = clusterStatusError(view.status);
  const nomad = view.cluster.kind === 'nomad';
  const [renaming, setRenaming] = useState(false);
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
            {!nomad && (
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="outline" size="icon" aria-label={t('common.moreActions')}>
                    <EllipsisVerticalIcon />
                  </Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent align="end" className="w-auto">
                  <DropdownMenuItem onSelect={() => setRenaming(true)}>
                    <PencilIcon /> {t('clusters.rename.open')}
                  </DropdownMenuItem>
                </DropdownMenuContent>
              </DropdownMenu>
            )}
          </>
        }
      />
      {error && <ErrorMessage error={error} onRetry={() => reconnect.mutate()} />}
      {nomad ? (
        <>
          <NomadConnectionCard view={view} />
          <NomadSettingsCard view={view} />
        </>
      ) : (
        <>
          <ConnectionCard view={view} />
          <RefreshCard view={view} />
        </>
      )}
      <div className="mt-4">
        <RemoveClusterCard view={view} />
      </div>
      {renaming && <RenameDialog view={view} onClose={() => setRenaming(false)} />}
    </div>
  );
}
