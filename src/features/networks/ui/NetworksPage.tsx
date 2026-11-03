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
  useConfirm,
} from '@/shared/ui';
import { usePruneNetworks, useRemoveNetwork } from '../api/mutations';
import { CreateNetworkDialog } from './dialogs/CreateNetworkDialog';
import { NetworkDialog } from './dialogs/NetworkDialog';
import { networkColumns } from './networkColumns';

function NetworksList({ hostId }: { hostId: string }) {
  const networks = useQuery(resourceQueries.networks(hostId));
  const removeNetwork = useRemoveNetwork(hostId);
  const prune = usePruneNetworks(hostId);
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [creating, setCreating] = useState(false);
  const [open, setOpen] = useState<NetworkView | null>(null);

  const remove = (n: NetworkView) =>
    confirm({
      title: `Remove network ${n.name}?`,
      description: n.containers.length
        ? `It still has ${n.containers.length} attached container(s); disconnect them first.`
        : 'No container uses it. Containers that reference it by name will fail to start.',
      confirmLabel: 'Remove',
      destructive: true,
      onConfirm: () => removeNetwork.mutateAsync(n).catch(() => {}),
    });

  if (networks.isPending) return <Spinner />;
  if (networks.error)
    return <ErrorMessage error={networks.error} onRetry={() => void networks.refetch()} />;
  return (
    <div>
      <PageHeader
        title="Networks"
        description={`${networks.data.length} networks`}
        actions={
          <>
            <Button
              variant="outline"
              onClick={() =>
                confirm({
                  title: 'Remove unused networks?',
                  description: 'Every custom network no container is attached to is removed.',
                  confirmLabel: 'Clean up',
                  destructive: true,
                  onConfirm: () => prune.mutateAsync(),
                })
              }
            >
              <BrushCleaningIcon /> Clean up
            </Button>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> Create network
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search networks" />
      </div>
      {networks.data.length === 0 ? (
        <EmptyState icon={NetworkIcon} title="No networks" />
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
