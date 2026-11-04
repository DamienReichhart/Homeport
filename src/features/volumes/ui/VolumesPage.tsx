import { useQuery } from '@tanstack/react-query';
import { BrushCleaningIcon, HardDriveIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { HostGate, resourceQueries } from '@/core/host';
import type { VolumeView } from '@/shared/api';
import {
  Button,
  DataTable,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  useConfirm,
} from '@/shared/ui';
import { usePruneVolumes, useRemoveVolume } from '../api/mutations';
import { CreateVolumeDialog } from './dialogs/CreateVolumeDialog';
import { InspectVolumeDialog } from './dialogs/InspectVolumeDialog';
import { volumeColumns } from './volumeColumns';

function VolumesList({ hostId }: { hostId: string }) {
  const volumes = useQuery(resourceQueries.volumes(hostId));
  const removeVolume = useRemoveVolume(hostId);
  const pruneVolumes = usePruneVolumes(hostId);
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [creating, setCreating] = useState(false);
  const [inspecting, setInspecting] = useState<VolumeView | null>(null);

  const remove = (v: VolumeView) =>
    confirm({
      title: `Delete volume ${v.name}?`,
      description: (
        <>
          <strong>All data in this volume is permanently deleted.</strong>
          {v.usedBy.length > 0 &&
            ` It is mounted by ${v.usedBy.join(', ')}; remove those containers first.`}
        </>
      ),
      confirmLabel: 'Delete volume and data',
      destructive: true,
      onConfirm: () => removeVolume.mutateAsync(v).catch(() => {}),
    });

  const prune = (all: boolean) =>
    confirm({
      title: all ? 'Delete every unused volume?' : 'Delete unused anonymous volumes?',
      description: (
        <strong>
          {all
            ? 'The data in every volume no container uses is permanently deleted, named volumes included.'
            : 'The data in unnamed volumes no container uses is permanently deleted.'}
        </strong>
      ),
      confirmLabel: 'Delete volumes and data',
      destructive: true,
      onConfirm: () => pruneVolumes.mutateAsync(all),
    });

  if (volumes.isPending) return <Spinner />;
  if (volumes.error)
    return <ErrorMessage error={volumes.error} onRetry={() => void volumes.refetch()} />;
  return (
    <div>
      <PageHeader
        title="Volumes"
        description={`${volumes.data.length} volumes · ${volumes.data.filter((v) => v.usedBy.length === 0).length} unused`}
        actions={
          <>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">
                  <BrushCleaningIcon /> Clean up
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem variant="destructive" onSelect={() => prune(false)}>
                  Unused anonymous volumes
                </DropdownMenuItem>
                <DropdownMenuItem variant="destructive" onSelect={() => prune(true)}>
                  All unused volumes
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> Create volume
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search volumes" />
      </div>
      {volumes.data.length === 0 ? (
        <EmptyState
          icon={HardDriveIcon}
          title="No volumes"
          description="Volumes keep container data across restarts and re-creations."
        />
      ) : (
        <DataTable
          columns={volumeColumns({ onInspect: setInspecting, onRemove: remove })}
          data={volumes.data}
          getRowId={(v) => v.name}
          filter={query}
          onRowClick={setInspecting}
        />
      )}
      <CreateVolumeDialog hostId={hostId} open={creating} onOpenChange={setCreating} />
      {inspecting && (
        <InspectVolumeDialog
          hostId={hostId}
          volume={inspecting}
          onClose={() => setInspecting(null)}
        />
      )}
      {dialog}
    </div>
  );
}

export function VolumesPage() {
  return <HostGate>{(hostId) => <VolumesList key={hostId} hostId={hostId} />}</HostGate>;
}
