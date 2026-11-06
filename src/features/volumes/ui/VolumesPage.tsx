import { useQuery } from '@tanstack/react-query';
import {
  ArchiveRestoreIcon,
  BrushCleaningIcon,
  HardDriveIcon,
  PlusIcon,
  RefreshCwIcon,
} from 'lucide-react';
import { useState } from 'react';
import { HostGate, resourceQueries } from '@/core/host';
import type { VolumeView } from '@/shared/api';
import { pickFile, pickSavePath } from '@/shared/lib';
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
import { BackupVolumeDialog } from './dialogs/BackupVolumeDialog';
import { BrowseVolumeDialog } from './dialogs/BrowseVolumeDialog';
import { CloneVolumeDialog } from './dialogs/CloneVolumeDialog';
import { RestoreVolumeDialog } from './dialogs/RestoreVolumeDialog';
import { CreateVolumeDialog } from './dialogs/CreateVolumeDialog';
import { InspectVolumeDialog } from './dialogs/InspectVolumeDialog';
import { volumeColumns } from './volumeColumns';
import { t } from '@/shared/i18n';

const tar = () => [{ name: t('volumes.transfer.archives'), extensions: ['tar'] }];

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
  const [backingUp, setBackingUp] = useState<{ volume: string; destination: string } | null>(null);
  const [cloning, setCloning] = useState<VolumeView | null>(null);
  const [selected, setSelected] = useState<string | null>(null);
  const [restoring, setRestoring] = useState<{ source: string; existing?: string } | null>(null);

  const backup = async (v: VolumeView) => {
    const destination = await pickSavePath(
      t('volumes.backup.saveAs', { volume: v.name }),
      `${v.name}.tar`,
      tar(),
    );
    if (destination) setBackingUp({ volume: v.name, destination });
  };
  const restoreFrom = async (existing?: string) => {
    const source = await pickFile(t('volumes.restore.pick'), tar());
    if (source) setRestoring({ source, existing });
  };

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
            <Button variant="outline" onClick={() => void restoreFrom()}>
              <ArchiveRestoreIcon /> {t('volumes.restoreFromFile')}
            </Button>
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
            onBackup: (v) => void backup(v),
            onRestore: (v) => void restoreFrom(v.name),
            onClone: setCloning,
            onRemove: remove,
          })}
          data={volumes.data}
          getRowId={(v) => v.name}
          selectedId={selected}
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
      {backingUp && (
        <BackupVolumeDialog
          hostId={hostId}
          volume={backingUp.volume}
          destination={backingUp.destination}
          onClose={() => setBackingUp(null)}
        />
      )}
      {cloning && (
        <CloneVolumeDialog
          hostId={hostId}
          volume={cloning.name}
          volumes={volumes.data}
          onCloned={setSelected}
          onClose={() => setCloning(null)}
        />
      )}
      {restoring && (
        <RestoreVolumeDialog
          hostId={hostId}
          source={restoring.source}
          existing={restoring.existing}
          volumes={volumes.data}
          onClose={() => setRestoring(null)}
        />
      )}
      {dialog}
    </div>
  );
}

export function VolumesPage() {
  return <HostGate>{(hostId) => <VolumesList key={hostId} hostId={hostId} />}</HostGate>;
}
