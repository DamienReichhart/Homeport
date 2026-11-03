import { useQuery } from '@tanstack/react-query';
import {
  DownloadIcon,
  FileIcon,
  FolderIcon,
  LinkIcon,
  RefreshCwIcon,
  UploadIcon,
} from 'lucide-react';
import { useState } from 'react';
import type { FileEntry } from '@/shared/api';
import { formatBytes, formatDateTime, pickFiles, pickSavePath } from '@/shared/lib';
import { Button, type ColumnDef, DataTable, ErrorMessage } from '@/shared/ui';
import { useFileTransfers } from '../api/mutations';
import { containerQueries } from '../api/queries';
import { crumbs } from '../model/files';

const ICONS = { directory: FolderIcon, file: FileIcon, symlink: LinkIcon, other: FileIcon };

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
    const target = await pickSavePath(`Save ${entry.name}`, dir ? `${entry.name}.tar` : entry.name);
    if (!target) return;
    save.mutate({ entry, target });
  };

  const columns: ColumnDef<FileEntry, unknown>[] = [
    {
      id: 'name',
      header: 'Name',
      accessorFn: (e) => e.name,
      cell: ({ row }) => {
        const Icon = ICONS[row.original.kind];
        return (
          <span className="flex items-center gap-2">
            <Icon className="text-muted-foreground size-4" />
            {row.original.name}
          </span>
        );
      },
    },
    {
      id: 'size',
      header: 'Size',
      accessorFn: (e) => e.size,
      cell: ({ row }) => (row.original.kind === 'file' ? formatBytes(row.original.size) : ''),
    },
    {
      id: 'modified',
      header: 'Modified',
      accessorFn: (e) => e.modified,
      cell: ({ row }) => formatDateTime(row.original.modified),
    },
    {
      id: 'mode',
      header: 'Mode',
      accessorFn: (e) => e.mode,
      cell: ({ row }) => <span className="font-mono text-xs">{row.original.mode}</span>,
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => (
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={`Download ${row.original.name}`}
          onClick={(e) => {
            e.stopPropagation();
            void download(row.original);
          }}
        >
          <DownloadIcon />
        </Button>
      ),
    },
  ];

  if (!running)
    return (
      <p className="text-muted-foreground text-sm">Start the container to browse its files.</p>
    );

  return (
    <div className="grid gap-3">
      <div className="flex flex-wrap items-center gap-2">
        <nav aria-label="Path" className="flex flex-wrap items-center gap-0.5 font-mono text-sm">
          {crumbs(path).map(([label, target], i) => (
            <span key={target} className="flex items-center">
              {i > 1 && <span className="text-muted-foreground">/</span>}
              <button
                type="button"
                className="hover:text-primary rounded px-1"
                onClick={() => setPath(target)}
              >
                {label}
              </button>
            </span>
          ))}
        </nav>
        <div className="ml-auto flex gap-2">
          <Button variant="outline" size="sm" onClick={() => void listing.refetch()}>
            <RefreshCwIcon /> Refresh
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={async () => {
              const sources = await pickFiles(`Upload to ${path}`);
              if (sources.length === 0) return;
              upload.mutate({ path, sources });
            }}
          >
            <UploadIcon /> Upload here
          </Button>
        </div>
      </div>
      {listing.error ? (
        <ErrorMessage error={listing.error} onRetry={() => void listing.refetch()} />
      ) : (
        <DataTable
          columns={columns}
          data={listing.data ?? []}
          getRowId={(e) => e.path}
          empty={listing.isPending ? 'Loading…' : 'This directory is empty.'}
          onRowClick={(e) => e.kind === 'directory' && setPath(e.path)}
        />
      )}
    </div>
  );
}
