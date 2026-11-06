import { useQuery } from '@tanstack/react-query';
import {
  BrushCleaningIcon,
  DownloadIcon,
  FileArchiveIcon,
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
  WithShortcut,
} from '@/shared/ui';
import { chooseArchiveToLoad, chooseArchiveToSave, chooseFilesystemToImport } from '../api/files';
import { usePruneImages, useRemoveImage } from '../api/mutations';
import { archiveName, saveNames } from '../model/transfer';
import { pruneImagesConfirmation, removeImageConfirmation } from './confirmations';
import { BuildDialog } from './dialogs/BuildDialog';
import { ImportDialog } from './dialogs/ImportDialog';
import { InspectDialog } from './dialogs/InspectDialog';
import { PullDialog } from './dialogs/PullDialog';
import { PushDialog } from './dialogs/PushDialog';
import { TagDialog } from './dialogs/TagDialog';
import { TransferDialog, type Transfer } from './dialogs/TransferDialog';
import { imageColumns } from './imageColumns';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

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
  const [importing, setImporting] = useState<string | null>(null);
  const importFilesystem = async () => {
    const path = await chooseFilesystemToImport();
    if (path) setImporting(path);
  };
  const [inspecting, setInspecting] = useState<Image | null>(null);
  usePageAction(t('images.pull'), () => setPulling(true));

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
        title={t('images.title')}
        description={t('images.summary', { count: images.data.length, size: formatBytes(total) })}
        actions={
          <>
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline">
                  <BrushCleaningIcon /> {t('images.cleanUp')}
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent align="end">
                <DropdownMenuItem onSelect={() => prune(false)}>
                  {t('images.cleanUp.dangling')}
                </DropdownMenuItem>
                <DropdownMenuItem variant="destructive" onSelect={() => prune(true)}>
                  {t('images.cleanUp.all')}
                </DropdownMenuItem>
              </DropdownMenuContent>
            </DropdownMenu>
            {selected.length > 0 && (
              <Button variant="outline" onClick={() => void saveImages(selected)}>
                <SaveIcon /> {t('images.saveSelected', { count: selected.length })}
              </Button>
            )}
            <CheckNewerImages hostId={hostId} />
            <Button variant="outline" onClick={() => void loadImages()}>
              <FileUpIcon /> {t('images.load')}
            </Button>
            <Button variant="outline" onClick={() => void importFilesystem()}>
              <FileArchiveIcon /> {t('images.importTar')}
            </Button>
            <Button variant="outline" onClick={() => setBuilding(true)}>
              <HammerIcon /> {t('images.build')}
            </Button>
            <WithShortcut keys={PAGE_ACTION_KEYS} label={t('images.pull')}>
              <Button onClick={() => setPulling(true)}>
                <DownloadIcon /> {t('images.pull')}
              </Button>
            </WithShortcut>
          </>
        }
      />
      <div className="mb-3">
        <SearchInput value={query} onChange={setQuery} placeholder={t('images.search')} />
      </div>
      {images.data.length === 0 ? (
        <EmptyState
          icon={LayersIcon}
          title={t('images.empty.title')}
          description={t('images.empty.description')}
          action={<Button onClick={() => setPulling(true)}>{t('images.pull')}</Button>}
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
      {importing && (
        <ImportDialog
          path={importing}
          onClose={() => setImporting(null)}
          onImport={(reference, changes) => {
            setImporting(null);
            setTransfer({ kind: 'import', path: importing, reference, changes });
          }}
        />
      )}
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
