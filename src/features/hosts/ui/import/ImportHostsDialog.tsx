import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { ImportFailure } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import { useImportHosts } from '../../api/mutations';
import { importPreviewQuery } from '../../api/queries';
import { rowsFrom, toChoices, type ImportRow, type ImportSource } from '../../model/importRows';
import { ImportRowView } from './ImportRowView';
import { t } from '@/shared/i18n';

const TITLES = {
  sshConfig: 'hosts.import.sshConfig.title',
  dockerContexts: 'hosts.import.dockerContexts.title',
} as const;
const DESCRIPTIONS = {
  sshConfig: 'hosts.import.sshConfig.description',
  dockerContexts: 'hosts.import.dockerContexts.description',
} as const;
const EMPTY = {
  sshConfig: 'hosts.import.sshConfig.empty',
  dockerContexts: 'hosts.import.dockerContexts.empty',
} as const;

/** Previews what `source` describes and creates the Hosts the user picks. */
export function ImportHostsDialog({
  source,
  onClose,
}: {
  source: ImportSource;
  onClose: () => void;
}) {
  const preview = useQuery(importPreviewQuery(source));
  const [edited, setEdited] = useState<ImportRow[] | null>(null);
  const [failed, setFailed] = useState<ImportFailure[]>([]);
  const rows = edited ?? (preview.data ? rowsFrom(preview.data) : []);
  const importHosts = useImportHosts(source, (outcome) => {
    if (outcome.failed.length === 0) return onClose();
    setFailed(outcome.failed);
    // Keep only what failed, so trying again doesn't create the others twice.
    const left = new Set(outcome.failed.map((f) => f.source));
    setEdited(rows.filter((r) => left.has(r.draft.source)));
  });
  const choices = toChoices(rows);
  const picked = rows.filter((r) => r.selected).length;

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-5xl">
        <DialogHeader>
          <DialogTitle>{t(TITLES[source])}</DialogTitle>
          <DialogDescription>{t(DESCRIPTIONS[source])}</DialogDescription>
        </DialogHeader>
        {preview.isPending ? (
          <Spinner />
        ) : preview.error ? (
          <ErrorMessage error={preview.error} onRetry={() => void preview.refetch()} />
        ) : rows.length === 0 ? (
          <p className="text-muted-foreground text-sm">{t(EMPTY[source])}</p>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead />
                <TableHead>{t('hosts.import.column.name')}</TableHead>
                <TableHead>{t('hosts.import.column.address')}</TableHead>
                <TableHead>{t('hosts.import.column.auth')}</TableHead>
                <TableHead>{t('hosts.import.column.notes')}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {rows.map((row, i) => (
                <ImportRowView
                  key={row.draft.source}
                  row={row}
                  rows={rows}
                  onChange={(next) => setEdited(rows.map((r, j) => (j === i ? next : r)))}
                />
              ))}
            </TableBody>
          </Table>
        )}
        {failed.map((f) => (
          <div key={f.source} className="grid gap-1.5">
            <span className="text-sm font-medium">
              {t('hosts.import.failed', { name: f.source })}
            </span>
            <ErrorMessage error={f.error} />
          </div>
        ))}
        {importHosts.error && <ErrorMessage error={importHosts.error} />}
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button
            disabled={!choices || importHosts.isPending}
            onClick={() => choices && importHosts.mutate(choices)}
          >
            {t('hosts.import.submit', { count: picked })}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
