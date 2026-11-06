import { DownloadIcon, FileIcon, FolderIcon, LinkIcon, RefreshCwIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import type { FileEntry } from '@/shared/api';
import { formatBytes, formatDateTime } from '@/shared/lib';
import { Button } from '../primitives/button';
import { ErrorMessage } from '../feedback/ErrorMessage';
import { type ColumnDef, DataTable } from './DataTable';
import { crumbs } from './fileBrowser.model';
import { t } from '@/shared/i18n';

const ICONS = { directory: FolderIcon, file: FileIcon, symlink: LinkIcon, other: FileIcon };

/**
 * A folder's listing with breadcrumbs: opens folders, downloads entries. The
 * caller lists `path` and saves what's downloaded; `actions` sit beside
 * Refresh (an upload button, say).
 */
export function FileBrowser({
  path,
  onNavigate,
  entries,
  error,
  onRefresh,
  onDownload,
  actions,
}: {
  path: string;
  onNavigate: (path: string) => void;
  /** `undefined` while the folder is listed. */
  entries: FileEntry[] | undefined;
  error: unknown;
  onRefresh: () => void;
  onDownload: (entry: FileEntry) => void;
  actions?: ReactNode;
}) {
  const columns: ColumnDef<FileEntry, unknown>[] = [
    {
      id: 'name',
      header: t('ui.files.column.name'),
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
      header: t('ui.files.column.size'),
      accessorFn: (e) => e.size,
      cell: ({ row }) => (row.original.kind === 'file' ? formatBytes(row.original.size) : ''),
    },
    {
      id: 'modified',
      header: t('ui.files.column.modified'),
      accessorFn: (e) => e.modified,
      cell: ({ row }) => formatDateTime(row.original.modified),
    },
    {
      id: 'mode',
      header: t('ui.files.column.mode'),
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
          aria-label={t('ui.files.downloadOne', { name: row.original.name })}
          onClick={(e) => {
            e.stopPropagation();
            onDownload(row.original);
          }}
        >
          <DownloadIcon />
        </Button>
      ),
    },
  ];

  return (
    <div className="grid gap-3">
      <div className="flex flex-wrap items-center gap-2">
        <nav
          aria-label={t('ui.files.path')}
          className="flex flex-wrap items-center gap-0.5 font-mono text-sm"
        >
          {crumbs(path).map(([label, target], i) => (
            <span key={target} className="flex items-center">
              {i > 1 && <span className="text-muted-foreground">/</span>}
              <button
                type="button"
                className="hover:text-primary rounded px-1"
                onClick={() => onNavigate(target)}
              >
                {label}
              </button>
            </span>
          ))}
        </nav>
        <div className="ml-auto flex gap-2">
          <Button variant="outline" size="sm" onClick={onRefresh}>
            <RefreshCwIcon /> {t('common.refresh')}
          </Button>
          {actions}
        </div>
      </div>
      {error ? (
        <ErrorMessage error={error} onRetry={onRefresh} />
      ) : (
        <DataTable
          columns={columns}
          data={entries ?? []}
          getRowId={(e) => e.path}
          empty={entries === undefined ? t('common.loading') : t('ui.files.empty')}
          onRowClick={(e) => e.kind === 'directory' && onNavigate(e.path)}
        />
      )}
    </div>
  );
}
