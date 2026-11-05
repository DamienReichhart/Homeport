import { TriangleAlertIcon } from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import { describeAddress, useHosts } from '@/core/host';
import type { ExportOutcome } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import {
  Alert,
  AlertDescription,
  AlertTitle,
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
} from '@/shared/ui';
import { useExportHosts } from '../../api/mutations';
import { BUNDLE_EXTENSION, MIN_PASSPHRASE, passphraseProblem } from '../../model/bundle';
import { PassphraseFields } from './PassphraseFields';
import { t } from '@/shared/i18n';

/** Picks Hosts and a passphrase, then writes them to an export file. */
export function ExportHostsDialog({ onClose }: { onClose: () => void }) {
  const hosts = useHosts();
  const [unpicked, setUnpicked] = useState<Set<string>>(new Set());
  const [passphrase, setPassphrase] = useState('');
  const [confirmation, setConfirmation] = useState('');
  const [done, setDone] = useState<ExportOutcome | null>(null);
  const exportHosts = useExportHosts();
  const ids = (hosts.data ?? []).map((v) => v.host.id).filter((id) => !unpicked.has(id));
  const problem = passphraseProblem(passphrase, confirmation);

  const run = async () => {
    const path = await pickSavePath(
      t('hosts.bundle.export.saveTitle'),
      `hosts.${BUNDLE_EXTENSION}`,
      [{ name: t('hosts.bundle.fileType'), extensions: [BUNDLE_EXTENSION] }],
    );
    if (!path) return;
    const outcome = await exportHosts.mutateAsync({ ids, path, passphrase }).catch(() => null);
    if (!outcome) return;
    toast.success(t('hosts.bundle.export.done', { count: outcome.exported }));
    if (outcome.withoutSecrets.length === 0) onClose();
    else setDone(outcome);
  };

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('hosts.bundle.export.title')}</DialogTitle>
          <DialogDescription>{t('hosts.bundle.export.description')}</DialogDescription>
        </DialogHeader>
        {done ? (
          <Alert>
            <TriangleAlertIcon />
            <AlertTitle>{t('hosts.bundle.export.withoutSecrets')}</AlertTitle>
            <AlertDescription>{done.withoutSecrets.join(', ')}</AlertDescription>
          </Alert>
        ) : (
          <>
            <div className="grid gap-2" role="group" aria-label={t('hosts.bundle.export.hosts')}>
              {hosts.data?.map(({ host }) => (
                <div key={host.id} className="flex items-center gap-2">
                  <Checkbox
                    id={`export-${host.id}`}
                    checked={!unpicked.has(host.id)}
                    onCheckedChange={(v) => {
                      const next = new Set(unpicked);
                      if (v === true) next.delete(host.id);
                      else next.add(host.id);
                      setUnpicked(next);
                    }}
                  />
                  <Label htmlFor={`export-${host.id}`} className="font-normal">
                    <span className="font-medium">{host.name}</span>
                    <span className="text-muted-foreground font-mono text-xs">
                      {describeAddress(host.connection)}
                    </span>
                  </Label>
                </div>
              ))}
            </div>
            <PassphraseFields
              passphrase={passphrase}
              confirmation={confirmation}
              onChange={(p, c) => {
                setPassphrase(p);
                setConfirmation(c);
              }}
            />
            {passphrase && problem && (
              <p className="text-destructive text-sm">{t(problem, { min: MIN_PASSPHRASE })}</p>
            )}
            {exportHosts.error && <ErrorMessage error={exportHosts.error} />}
          </>
        )}
        <DialogFooter>
          {done ? (
            <Button onClick={onClose}>{t('common.close')}</Button>
          ) : (
            <>
              <Button variant="ghost" onClick={onClose}>
                {t('common.cancel')}
              </Button>
              <Button
                disabled={ids.length === 0 || !!problem || exportHosts.isPending}
                onClick={() => void run()}
              >
                {t('hosts.bundle.export.submit', { count: ids.length })}
              </Button>
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
