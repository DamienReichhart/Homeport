import { useQuery } from '@tanstack/react-query';
import { BoxIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usePageQuery } from '@/core/feature';
import { ContainerdGate } from '@/core/host';
import type { ContainerdContainer } from '@/shared/api';
import {
  DataTable,
  EmptyState,
  ErrorMessage,
  Label,
  PageHeader,
  SearchInput,
  Spinner,
  Switch,
} from '@/shared/ui';
import { containerdContainerQueries } from '../api/queries';
import { visibleContainers } from '../model/containers';
import { containerColumns } from './containerColumns';
import { RemoveContainerDialog } from './RemoveContainerDialog';
import { t } from '@/shared/i18n';

function ContainersList({ hostId, namespace }: { hostId: string; namespace: string }) {
  const containers = useQuery(containerdContainerQueries.list(hostId, namespace));
  const navigate = useNavigate();
  const [query, setQuery] = usePageQuery();
  const [sandboxes, setSandboxes] = useState(false);
  const [removing, setRemoving] = useState<ContainerdContainer | null>(null);

  if (containers.isPending) return <Spinner />;
  if (containers.error)
    return <ErrorMessage error={containers.error} onRetry={() => void containers.refetch()} />;
  const shown = visibleContainers(containers.data, sandboxes);
  const hasSandboxes = containers.data.some((c) => c.sandbox);
  return (
    <div>
      <PageHeader
        title={t('containerd.containers.title')}
        description={t('containerd.containers.summary', {
          count: containers.data.length,
          running: containers.data.filter((c) => c.state === 'running').length,
          namespace,
        })}
      />
      <div className="mb-3 flex flex-wrap items-center gap-4">
        <SearchInput
          value={query}
          onChange={setQuery}
          placeholder={t('containerd.containers.search')}
        />
        {hasSandboxes && (
          <div className="flex items-center gap-2">
            <Switch id="show-sandboxes" checked={sandboxes} onCheckedChange={setSandboxes} />
            <Label htmlFor="show-sandboxes">{t('containerd.containers.showSandboxes')}</Label>
          </div>
        )}
      </div>
      {shown.length === 0 ? (
        <EmptyState
          icon={BoxIcon}
          title={t('containerd.containers.empty', { namespace })}
          description={t('containerd.containers.emptyDescription')}
        />
      ) : (
        <DataTable
          columns={containerColumns({ hostId, namespace, onRemove: setRemoving })}
          data={shown}
          getRowId={(c) => c.id}
          filter={query}
          onRowClick={(c) => void navigate(`/containerd/containers/${encodeURIComponent(c.id)}`)}
        />
      )}
      <RemoveContainerDialog
        hostId={hostId}
        namespace={namespace}
        container={removing}
        onClose={() => setRemoving(null)}
      />
    </div>
  );
}

export function ContainersPage() {
  return (
    <ContainerdGate>
      {(hostId, namespace) => (
        <ContainersList key={`${hostId}/${namespace}`} hostId={hostId} namespace={namespace} />
      )}
    </ContainerdGate>
  );
}
