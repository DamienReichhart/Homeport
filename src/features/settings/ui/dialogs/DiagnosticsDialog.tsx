import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { toast } from 'sonner';
import { formatBytes, pickSavePath } from '@/shared/lib';
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
  Label,
  Spinner,
} from '@/shared/ui';
import { useExportDiagnostics } from '../../api/mutations';
import { diagnosticsQuery } from '../../api/queries';
import { t } from '@/shared/i18n';

/** `homeport-diagnostics-2026-11-09.zip`, by the local date. */
const fileName = (now = new Date()) => {
  const pad = (n: number) => String(n).padStart(2, '0');
  return `homeport-diagnostics-${now.getFullYear()}-${pad(now.getMonth() + 1)}-${pad(now.getDate())}.zip`;
};

/**
 * Shows what a diagnostics bundle would hold, then saves it where the user
 * chooses. The file goes nowhere else: attaching it to a bug report is theirs
 * to do.
 */
export function DiagnosticsDialog({ onClose }: { onClose: () => void }) {
  const [includeNames, setIncludeNames] = useState(false);
  const preview = useQuery(diagnosticsQuery(includeNames));
  const save = useExportDiagnostics();

  const run = async () => {
    const path = await pickSavePath(t('settings.diagnostics.saveTitle'), fileName(), [
      { name: t('settings.diagnostics.fileType'), extensions: ['zip'] },
    ]);
    if (!path) return;
    const saved = await save.mutateAsync({ path, includeNames }).then(
      () => true,
      () => false,
    );
    if (!saved) return;
    toast.success(t('settings.diagnostics.done', { path }));
    onClose();
  };

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>{t('settings.diagnostics.title')}</DialogTitle>
          <DialogDescription>{t('settings.diagnostics.description')}</DialogDescription>
        </DialogHeader>
        <div className="flex items-start gap-2">
          <Checkbox
            id="diagnostics-names"
            checked={includeNames}
            onCheckedChange={(on) => setIncludeNames(on === true)}
          />
          <Label htmlFor="diagnostics-names" className="grid gap-0.5 font-normal">
            <span className="font-medium">{t('settings.diagnostics.includeNames')}</span>
            <span className="text-muted-foreground">
              {t('settings.diagnostics.includeNamesHelp')}
            </span>
          </Label>
        </div>
        {preview.isPending && <Spinner />}
        {preview.error && (
          <ErrorMessage error={preview.error} onRetry={() => void preview.refetch()} />
        )}
        {preview.data && (
          <div className="grid min-w-0 gap-3">
            <div className="grid gap-1.5">
              <p className="text-sm font-medium">{t('settings.diagnostics.files')}</p>
              <ul className="text-muted-foreground grid gap-0.5 font-mono text-xs">
                {preview.data.files.map((file) => (
                  <li key={file.name} className="flex justify-between gap-4">
                    <span className="min-w-0 break-all">{file.name}</span>
                    <span className="shrink-0">{formatBytes(file.size)}</span>
                  </li>
                ))}
              </ul>
            </div>
            <div className="grid gap-1.5">
              <p className="text-sm font-medium">summary.json</p>
              <pre
                className="bg-muted max-h-[40vh] overflow-auto rounded-md p-3 font-mono text-xs"
                // Scrollable, so reachable with the keyboard.
                tabIndex={0}
                aria-label="summary.json"
              >
                {preview.data.summary}
              </pre>
            </div>
          </div>
        )}
        {save.error && <ErrorMessage error={save.error} />}
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button onClick={() => void run()} disabled={!preview.data || save.isPending}>
            {t('settings.diagnostics.save')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
