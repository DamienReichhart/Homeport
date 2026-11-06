import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { FileEntry, VolumeView } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  FileBrowser,
  Spinner,
} from '@/shared/ui';
import { useBrowseSession } from '../../api/helper';
import { useDownloadFromVolume } from '../../api/mutations';
import { volumeQueries } from '../../api/queries';
import { HelperImageGate } from '../HelperImageGate';
import { t } from '@/shared/i18n';

/** Browses a volume's files, read-only, through a helper container (ADR 0015). */
export function BrowseVolumeDialog({
  hostId,
  volume,
  onClose,
}: {
  hostId: string;
  volume: VolumeView;
  onClose: () => void;
}) {
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-4xl">
        <DialogHeader>
          <DialogTitle>{t('volumes.browse.title', { name: volume.name })}</DialogTitle>
          <DialogDescription>{t('volumes.browse.description')}</DialogDescription>
        </DialogHeader>
        <HelperImageGate hostId={hostId} onCancel={onClose}>
          {(pulledImage) => (
            <VolumeFiles hostId={hostId} volume={volume.name} pulledImage={pulledImage} />
          )}
        </HelperImageGate>
      </DialogContent>
    </Dialog>
  );
}

function VolumeFiles({
  hostId,
  volume,
  pulledImage,
}: {
  hostId: string;
  volume: string;
  pulledImage: boolean;
}) {
  const { session, error, retry } = useBrowseSession(hostId, volume, pulledImage);
  if (error) return <ErrorMessage error={error} onRetry={retry} />;
  if (!session) return <Spinner />;
  return <Listing hostId={hostId} session={session} />;
}

function Listing({ hostId, session }: { hostId: string; session: string }) {
  const [path, setPath] = useState('/');
  const listing = useQuery(volumeQueries.browse(hostId, session, path));
  const save = useDownloadFromVolume(hostId, session);
  const download = async (entry: FileEntry) => {
    const folder = entry.kind === 'directory';
    const target = await pickSavePath(
      t('volumes.browse.saveOne', { name: entry.name }),
      folder ? `${entry.name}.tar` : entry.name,
    );
    if (target) save.mutate({ entry, target });
  };
  return (
    <FileBrowser
      path={path}
      onNavigate={setPath}
      entries={listing.data}
      error={listing.error}
      onRefresh={() => void listing.refetch()}
      onDownload={(entry) => void download(entry)}
    />
  );
}
