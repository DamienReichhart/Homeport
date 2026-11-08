import { useQuery } from '@tanstack/react-query';
import { GroupIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useActionRequest, usePageQuery } from '@/core/feature';
import { HostGate, resourceQueries } from '@/core/host';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirmedAction,
  WithShortcut,
} from '@/shared/ui';
import { NewPodDialog } from './dialogs/NewPodDialog';
import { podColumns } from './podColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

function PodsList({ hostId }: { hostId: string }) {
  const pods = useQuery(resourceQueries.pods(hostId));
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = usePageQuery();
  const [creating, setCreating] = useState(false);
  usePageAction(t('podmanpods.new'), () => setCreating(true));
  useActionRequest('new', () => setCreating(true));

  if (pods.isPending) return <Spinner />;
  if (pods.error) return <ErrorMessage error={pods.error} onRetry={() => void pods.refetch()} />;
  return (
    <div>
      <PageHeader
        title={t('podmanpods.title')}
        description={t('podmanpods.summary', { count: pods.data.length })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('podmanpods.new')}>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> {t('podmanpods.new')}
            </Button>
          </WithShortcut>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('podmanpods.search')} />
      </div>
      {pods.data.length === 0 ? (
        <EmptyState
          icon={GroupIcon}
          title={t('podmanpods.empty')}
          description={t('podmanpods.emptyDescription')}
        />
      ) : (
        <DataTable
          columns={podColumns({ hostId, confirm })}
          data={pods.data}
          getRowId={(p) => p.id}
          filter={query}
          onRowClick={(p) => void navigate(`/podman-pods/${p.id}`)}
        />
      )}
      <NewPodDialog
        hostId={hostId}
        open={creating}
        onOpenChange={setCreating}
        onCreated={(id) => void navigate(`/podman-pods/${id}`)}
      />
      {dialog}
    </div>
  );
}

export function PodsPage() {
  return (
    <HostGate requires="podman">{(hostId) => <PodsList key={hostId} hostId={hostId} />}</HostGate>
  );
}
