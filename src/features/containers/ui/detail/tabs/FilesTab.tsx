import { useQuery } from '@tanstack/react-query';
import { UploadIcon } from 'lucide-react';
import { useState } from 'react';
import type { FileEntry } from '@/shared/api';
import { pickFiles, pickSavePath } from '@/shared/lib';
import { Button, FileBrowser } from '@/shared/ui';
import { useFileTransfers } from '../../../api/mutations';
import { containerQueries } from '../../../api/queries';
import { t } from '@/shared/i18n';

export function FilesTab({
  hostId,
  containerId,
  running,
}: {
  hostId: string;
  containerId: string;
  running: boolean;
}) {
  const [path, setPath] = useState('/');
  const listing = useQuery({
    ...containerQueries.files(hostId, containerId, path),
    enabled: running,
  });
  const { download: save, upload } = useFileTransfers(
    hostId,
    containerId,
    () => void listing.refetch(),
  );

  const download = async (entry: FileEntry) => {
    const dir = entry.kind === 'directory';
    const target = await pickSavePath(
      t('containers.files.saveOne', { name: entry.name }),
      dir ? `${entry.name}.tar` : entry.name,
    );
    if (!target) return;
    save.mutate({ entry, target });
  };

  if (!running)
    return <p className="text-muted-foreground text-sm">{t('containers.files.startFirst')}</p>;

  return (
    <FileBrowser
      path={path}
      onNavigate={setPath}
      entries={listing.data}
      error={listing.error}
      onRefresh={() => void listing.refetch()}
      onDownload={(entry) => void download(entry)}
      actions={
        <Button
          variant="outline"
          size="sm"
          onClick={async () => {
            const sources = await pickFiles(t('containers.files.uploadTo', { path }));
            if (sources.length === 0) return;
            upload.mutate({ path, sources });
          }}
        >
          <UploadIcon /> {t('containers.files.uploadHere')}
        </Button>
      }
    />
  );
}
