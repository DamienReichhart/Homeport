import { FileUpIcon } from 'lucide-react';
import { useState } from 'react';
import { connectionLabel, describeAddress } from '@/core/host';
import type { ImportFailure } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import { useImportBundle, usePreviewBundle } from '../../api/mutations';
import {
  BUNDLE_EXTENSION,
  bundleChoices,
  bundleRowProblem,
  bundleRows,
  type BundleRow,
} from '../../model/bundle';
import { t } from '@/shared/i18n';

/** Opens an export file with its passphrase and creates the Hosts picked from it. */
export function ImportBundleDialog({ onClose }: { onClose: () => void }) {
  const [path, setPath] = useState<string | null>(null);
  const [passphrase, setPassphrase] = useState('');
  const [rows, setRows] = useState<BundleRow[] | null>(null);
  const [failed, setFailed] = useState<ImportFailure[]>([]);
  const preview = usePreviewBundle();
  const importBundle = useImportBundle((outcome) => {
    if (outcome.failed.length === 0) return onClose();
    setFailed(outcome.failed);
    const left = new Set(outcome.failed.map((f) => f.source));
    setRows((rows ?? []).filter((r) => left.has(r.host.name)));
  });
  const choices = rows && bundleChoices(rows);

  const open = () => {
    if (!path) return;
    preview.mutate({ path, passphrase }, { onSuccess: (hosts) => setRows(bundleRows(hosts)) });
  };

  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{t('hosts.bundle.import.title')}</DialogTitle>
          <DialogDescription>{t('hosts.bundle.import.description')}</DialogDescription>
        </DialogHeader>
        {!rows ? (
          <form
            className="grid gap-3"
            onSubmit={(e) => {
              e.preventDefault();
              open();
            }}
          >
            <div className="flex flex-wrap items-center gap-2">
              <Button
                type="button"
                variant="outline"
                onClick={async () => {
                  const picked = await pickFile(t('hosts.bundle.import.chooseTitle'), [
                    { name: t('hosts.bundle.fileType'), extensions: [BUNDLE_EXTENSION] },
                  ]);
                  if (picked) setPath(picked);
                }}
              >
                <FileUpIcon /> {t('hosts.bundle.import.choose')}
              </Button>
              {path && <span className="font-mono text-xs break-all">{path}</span>}
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="bundle-open-passphrase">{t('hosts.bundle.passphrase')}</Label>
              <Input
                id="bundle-open-passphrase"
                type="password"
                autoComplete="off"
                value={passphrase}
                onChange={(e) => setPassphrase(e.target.value)}
              />
            </div>
            {preview.error && <ErrorMessage error={preview.error} />}
            <Button
              type="submit"
              className="justify-self-start"
              disabled={!path || !passphrase || preview.isPending}
            >
              {t('hosts.bundle.import.open')}
            </Button>
          </form>
        ) : (
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead />
                <TableHead>{t('hosts.import.column.name')}</TableHead>
                <TableHead>{t('hosts.import.column.address')}</TableHead>
                <TableHead>{t('hosts.import.column.notes')}</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {rows.map((row, i) => {
                const set = (patch: Partial<BundleRow>) =>
                  setRows(rows.map((r, j) => (j === i ? { ...r, ...patch } : r)));
                const problem = bundleRowProblem(row, rows);
                return (
                  <TableRow key={row.host.index} className="align-top">
                    <TableCell>
                      <Checkbox
                        checked={row.selected}
                        onCheckedChange={(v) => set({ selected: v === true })}
                        aria-label={t('hosts.import.pick', { name: row.host.name })}
                      />
                    </TableCell>
                    <TableCell className="min-w-40">
                      <Input
                        value={row.name}
                        onChange={(e) => set({ name: e.target.value })}
                        aria-label={t('hosts.import.name', { name: row.host.name })}
                      />
                    </TableCell>
                    <TableCell className="whitespace-normal">
                      <span className="text-muted-foreground text-xs">
                        {connectionLabel(row.host.connection.kind)}
                      </span>{' '}
                      <span className="font-mono text-xs break-all">
                        {describeAddress(row.host.connection)}
                      </span>
                    </TableCell>
                    <TableCell className="max-w-64 text-xs whitespace-normal">
                      {row.host.nameTaken && (
                        <p className="text-warning">
                          {t('hosts.bundle.import.renamed', { name: row.host.name })}
                        </p>
                      )}
                      {row.host.secretsMissing && (
                        <p className="text-warning">{t('hosts.bundle.import.secretsMissing')}</p>
                      )}
                      {problem && <p className="text-destructive">{t(problem)}</p>}
                    </TableCell>
                  </TableRow>
                );
              })}
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
        {importBundle.error && <ErrorMessage error={importBundle.error} />}
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          {rows && (
            <Button
              disabled={!choices || importBundle.isPending}
              onClick={() => path && choices && importBundle.mutate({ path, passphrase, choices })}
            >
              {t('hosts.import.submit', { count: rows.filter((r) => r.selected).length })}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
