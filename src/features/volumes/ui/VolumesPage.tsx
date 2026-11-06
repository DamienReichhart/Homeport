import { useQuery } from '@tanstack/react-query';
import { BrushCleaningIcon, HardDriveIcon, PlusIcon, RefreshCwIcon } from 'lucide-react';
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
  useConfirmedAction,
} from '@/shared/ui';
import { usePruneVolumes, useRemoveVolume } from '../api/mutations';
import { pruneVolumesConfirmation, removeVolumeConfirmation } from './confirmations';
import { BrowseVolumeDialog } from './dialogs/BrowseVolumeDialog';
import { CreateVolumeDialog } from './dialogs/CreateVolumeDialog';
import { InspectVolumeDialog } from './dialogs/InspectVolumeDialog';
import { volumeColumns } from './volumeColumns';
import { t } from '@/shared/i18n';

function VolumesList({ hostId }: { hostId: string }) {
  const volumes = useQuery(resourceQueries.volumes(hostId));
  const sizes = useQuery(resourceQueries.volumeSizes(hostId));
  const removeVolume = useRemoveVolume(hostId);
  const pruneVolumes = usePruneVolumes(hostId);
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = useState('');
  const [creating, setCreating] = useState(false);
  const [inspecting, setInspecting] = useState<VolumeView | null>(null);
  const [browsing, setBrowsing] = useState<VolumeView | null>(null);

  const remove = (v: VolumeView) =>
    confirm(removeVolumeConfirmation(v, () => removeVolume.mutateAsync(v)));
  const prune = (all: boolean) =>
    confirm(pruneVolumesConfirmation(all, () => pruneVolumes.mutateAsync(all)));

  if (volumes.isPending) return <Spinner />;
  if (volumes.error)
    return <ErrorMessage error={volumes.error} onRetry={() => void volumes.refetch()} />;
  return (
    <div>
      <PageHeader
        title={t('volumes.title')}
        description={t('volumes.summary', {
          count: volumes.data.length,
          unused: volumes.data.filter((v) => v.usedBy.length === 0).length,
        })}
        actions={
          <>
            <Button
              variant="outline"
              disabled={sizes.isFetching}
              onClick={() => void sizes.refetch()}
            >
              <RefreshCwIcon className={sizes.isFetching ? 'animate-spin' : undefined} />{' '}
              {t('volumes.size.refresh')}
            </Button>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">
                  <BrushCleaningIcon /> {t('volumes.cleanUp')}
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem variant="destructive" onSelect={() => prune(false)}>
                  {t('volumes.cleanUp.anonymous')}
                </DropdownMenuItem>
                <DropdownMenuItem variant="destructive" onSelect={() => prune(true)}>
                  {t('volumes.cleanUp.all')}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> {t('volumes.create')}
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('volumes.search')} />
      </div>
      {volumes.data.length === 0 ? (
        <EmptyState
          icon={HardDriveIcon}
          title={t('volumes.empty.title')}
          description={t('volumes.empty.description')}
        />
      ) : (
        <DataTable
          columns={volumeColumns({
            // A failed measurement shows every size as unknown; the list still works.
            sizes: sizes.error ? {} : sizes.data,
            onInspect: setInspecting,
            onBrowse: setBrowsing,
            onRemove: remove,
          })}
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
          sizes={sizes.error ? {} : sizes.data}
          onClose={() => setInspecting(null)}
        />
      )}
      {browsing && (
        <BrowseVolumeDialog hostId={hostId} volume={browsing} onClose={() => setBrowsing(null)} />
      )}
      {dialog}
    </div>
  );
}

export function VolumesPage() {
  return <HostGate>{(hostId) => <VolumesList key={hostId} hostId={hostId} />}</HostGate>;
}
