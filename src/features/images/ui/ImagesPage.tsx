import { useQuery } from '@tanstack/react-query';
import {
  BrushCleaningIcon,
  DownloadIcon,
  FileUpIcon,
  HammerIcon,
  LayersIcon,
  SaveIcon,
} from 'lucide-react';
import { useState } from 'react';
import { CheckNewerImages, HostGate, indexFreshness, resourceQueries } from '@/core/host';
import { RunContainerDialog } from '@/features/containers';
import type { Image } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
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
import { chooseArchiveToLoad, chooseArchiveToSave } from '../api/files';
import { usePruneImages, useRemoveImage } from '../api/mutations';
import { archiveName, saveNames } from '../model/transfer';
import { pruneImagesConfirmation, removeImageConfirmation } from './confirmations';
import { BuildDialog } from './dialogs/BuildDialog';
import { InspectDialog } from './dialogs/InspectDialog';
import { PullDialog } from './dialogs/PullDialog';
import { PushDialog } from './dialogs/PushDialog';
import { TagDialog } from './dialogs/TagDialog';
import { TransferDialog, type Transfer } from './dialogs/TransferDialog';
import { imageColumns } from './imageColumns';

function ImagesList({ hostId }: { hostId: string }) {
  const images = useQuery(resourceQueries.images(hostId));
  const freshness = indexFreshness(useQuery(resourceQueries.freshness(hostId)).data);
  const removeImage = useRemoveImage(hostId);
  const pruneImages = usePruneImages(hostId);
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = useState('');
  const [pulling, setPulling] = useState(false);
  const [building, setBuilding] = useState(false);
  const [running, setRunning] = useState<Image | null>(null);
  const [tagging, setTagging] = useState<Image | null>(null);
  const [pushingImage, setPushingImage] = useState<Image | null>(null);
  const [selected, setSelected] = useState<Image[]>([]);
  const [transfer, setTransfer] = useState<Transfer | null>(null);

  const saveImages = async (chosen: Image[]) => {
    const path = await chooseArchiveToSave(archiveName(chosen));
    if (path) setTransfer({ kind: 'save', names: saveNames(chosen), path });
  };
  const loadImages = async () => {
    const path = await chooseArchiveToLoad();
    if (path) setTransfer({ kind: 'load', path });
  };
  const [inspecting, setInspecting] = useState<Image | null>(null);

  const remove = (image: Image) =>
    confirm(removeImageConfirmation(image, () => removeImage.mutateAsync(image)));
  const prune = (all: boolean) =>
    confirm(pruneImagesConfirmation(all, () => pruneImages.mutateAsync(all)));

  if (images.isPending) return <Spinner />;
  if (images.error)
    return <ErrorMessage error={images.error} onRetry={() => void images.refetch()} />;
  const total = images.data.reduce((s, i) => s + i.size, 0);

  return (
    <div>
      <PageHeader
        title="Images"
        description={`${images.data.length} images · ${formatBytes(total)}`}
        actions={
          <>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">
                  <BrushCleaningIcon /> Clean up
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onSelect={() => prune(false)}>Dangling images</DropdownMenuItem>
                <DropdownMenuItem variant="destructive" onSelect={() => prune(true)}>
                  All unused images
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
            {selected.length > 0 && (
              <Button variant="outline" onClick={() => void saveImages(selected)}>
                <SaveIcon /> Save {selected.length === 1 ? 'image' : `${selected.length} images`}…
              </Button>
            )}
            <CheckNewerImages hostId={hostId} />
            <Button variant="outline" onClick={() => void loadImages()}>
              <FileUpIcon /> Load…
            </Button>
            <Button variant="outline" onClick={() => setBuilding(true)}>
              <HammerIcon /> Build
            </Button>
            <Button onClick={() => setPulling(true)}>
              <DownloadIcon /> Pull image
            </Button>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder="Search images" />
      </div>
      {images.data.length === 0 ? (
        <EmptyState
          icon={LayersIcon}
          title="No images"
          description="Pull one from a registry to get started."
          action={<Button onClick={() => setPulling(true)}>Pull image</Button>}
        />
      ) : (
        <DataTable
          columns={imageColumns({
            onRun: setRunning,
            onInspect: setInspecting,
            onTag: setTagging,
            onPush: setPushingImage,
            onSave: (image) => void saveImages([image]),
            onRemove: remove,
            freshness,
          })}
          data={images.data}
          getRowId={(i) => i.id}
          filter={query}
          onRowClick={setInspecting}
          onSelectionChange={setSelected}
        />
      )}
      <PullDialog hostId={hostId} open={pulling} onOpenChange={setPulling} />
      {building && <BuildDialog hostId={hostId} onClose={() => setBuilding(false)} />}
      {transfer && (
        <TransferDialog hostId={hostId} transfer={transfer} onClose={() => setTransfer(null)} />
      )}
      {running && (
        <RunContainerDialog
          key={running.id}
          hostId={hostId}
          image={running.tags[0] ?? running.id}
          open
          onOpenChange={(open) => !open && setRunning(null)}
        />
      )}
      {tagging && <TagDialog hostId={hostId} image={tagging} onClose={() => setTagging(null)} />}
      {pushingImage && (
        <PushDialog
          key={pushingImage.id}
          hostId={hostId}
          image={pushingImage}
          onClose={() => setPushingImage(null)}
        />
      )}
      {inspecting && (
        <InspectDialog hostId={hostId} image={inspecting} onClose={() => setInspecting(null)} />
      )}
      {dialog}
    </div>
  );
}

export function ImagesPage() {
  return <HostGate>{(hostId) => <ImagesList key={hostId} hostId={hostId} />}</HostGate>;
}
