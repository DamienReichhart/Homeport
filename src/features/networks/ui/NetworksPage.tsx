import { useQuery } from '@tanstack/react-query';
import { BrushCleaningIcon, NetworkIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { HostGate, resourceQueries } from '@/core/host';
import type { NetworkView } from '@/shared/api';
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
import { usePruneNetworks, useRemoveNetwork } from '../api/mutations';
import { pruneNetworksConfirmation, removeNetworkConfirmation } from './confirmations';
import { CreateNetworkDialog } from './dialogs/CreateNetworkDialog';
import { NetworkDialog } from './dialogs/NetworkDialog';
import { networkColumns } from './networkColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

function NetworksList({ hostId }: { hostId: string }) {
  const networks = useQuery(resourceQueries.networks(hostId));
  const removeNetwork = useRemoveNetwork(hostId);
  const prune = usePruneNetworks(hostId);
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = useState('');
  const [creating, setCreating] = useState(false);
  const [open, setOpen] = useState<NetworkView | null>(null);
  usePageAction(t('networks.create'), () => setCreating(true));

  const remove = (n: NetworkView) =>
    confirm(removeNetworkConfirmation(n, () => removeNetwork.mutateAsync(n)));

  if (networks.isPending) return <Spinner />;
  if (networks.error)
    return <ErrorMessage error={networks.error} onRetry={() => void networks.refetch()} />;
  return (
    <div>
      <PageHeader
        title={t('networks.title')}
        description={t('networks.summary', { count: networks.data.length })}
        actions={
          <>
            <Button
              variant="outline"
              onClick={() => confirm(pruneNetworksConfirmation(() => prune.mutateAsync()))}
            >
              <BrushCleaningIcon /> {t('networks.cleanUp')}
            </Button>
            <WithShortcut keys={PAGE_ACTION_KEYS} label={t('networks.create')}>
              <Button onClick={() => setCreating(true)}>
                <PlusIcon /> {t('networks.create')}
              </Button>
            </WithShortcut>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('networks.search')} />
      </div>
      {networks.data.length === 0 ? (
        <EmptyState icon={NetworkIcon} title={t('networks.empty')} />
      ) : (
        <DataTable
          columns={networkColumns({ onInspect: setOpen, onRemove: remove })}
          data={networks.data}
          getRowId={(n) => n.id}
          filter={query}
          onRowClick={setOpen}
        />
      )}
      <CreateNetworkDialog hostId={hostId} open={creating} onOpenChange={setCreating} />
      {open && <NetworkDialog hostId={hostId} network={open} onClose={() => setOpen(null)} />}
      {dialog}
    </div>
  );
}

export function NetworksPage() {
  return <HostGate>{(hostId) => <NetworksList key={hostId} hostId={hostId} />}</HostGate>;
}
