import { useQuery } from '@tanstack/react-query';
import { BrushCleaningIcon, DownloadIcon, LayersIcon } from 'lucide-react';
import { useState } from 'react';
import { HostGate, resourceQueries } from '@/core/host';
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
  useConfirm,
} from '@/shared/ui';
import { usePruneImages, useRemoveImage } from '../api/mutations';
import { imageName } from '../model/imageName';
import { InspectDialog } from './dialogs/InspectDialog';
import { PullDialog } from './dialogs/PullDialog';
import { PushDialog } from './dialogs/PushDialog';
import { TagDialog } from './dialogs/TagDialog';
import { imageColumns } from './imageColumns';

function ImagesList({ hostId }: { hostId: string }) {
  const images = useQuery(resourceQueries.images(hostId));
  const removeImage = useRemoveImage(hostId);
  const pruneImages = usePruneImages(hostId);
  const [dialog, confirm] = useConfirm();
  const [query, setQuery] = useState('');
  const [pulling, setPulling] = useState(false);
  const [running, setRunning] = useState<Image | null>(null);
  const [tagging, setTagging] = useState<Image | null>(null);
  const [pushingImage, setPushingImage] = useState<Image | null>(null);
  const [inspecting, setInspecting] = useState<Image | null>(null);

  const remove = (image: Image) =>
    confirm({
      title: `Remove ${imageName(image)}?`,
      description:
        image.tags.length > 1
          ? `This removes every name of the image (${image.tags.join(', ')}) and its layers that nothing else uses.`
          : 'The image and the layers no other image uses are deleted. It can be pulled again later.',
      confirmLabel: 'Remove',
      destructive: true,
      onConfirm: () => removeImage.mutateAsync(image).catch(() => {}),
    });

  const prune = (all: boolean) =>
    confirm({
      title: all ? 'Remove all unused images?' : 'Remove dangling images?',
      description: all
        ? 'Every image no container uses is deleted, tagged ones included.'
        : 'Untagged images left behind by builds and pulls are deleted.',
      confirmLabel: 'Clean up',
      destructive: true,
      onConfirm: () => pruneImages.mutateAsync(all),
    });

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
            onRemove: remove,
          })}
          data={images.data}
          getRowId={(i) => i.id}
          filter={query}
          onRowClick={setInspecting}
        />
      )}
      <PullDialog hostId={hostId} open={pulling} onOpenChange={setPulling} />
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
