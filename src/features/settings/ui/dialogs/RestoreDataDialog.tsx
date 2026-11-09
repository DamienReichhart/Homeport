import { useState } from 'react';
import { toast } from 'sonner';
import type { BackupContents } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { usePreviewRestore, useRestoreData } from '../../api/backup';
import { t } from '@/shared/i18n';
import { formatDateTime } from '@/shared/lib';

/**
 * Opens the backup at `path` with its passphrase, lists what it holds, and
 * replaces everything Homeport holds with it once confirmed.
 */
export function RestoreDataDialog({ path, onClose }: { path: string; onClose: () => void }) {
  const [passphrase, setPassphrase] = useState('');
  const [contents, setContents] = useState<BackupContents | null>(null);
  const preview = usePreviewRestore();
  const restore = useRestoreData();

  const open = async () => {
    const opened = await preview.mutateAsync({ path, passphrase }).catch(() => null);
    if (opened) setContents(opened);
  };
  const replace = async () => {
    const restored = await restore.mutateAsync({ path, passphrase }).catch(() => null);
    if (!restored) return;
    toast.success(
      t('settings.restore.done', { hosts: restored.hosts.length, stacks: restored.stacks }),
    );
    onClose();
  };

  return (
    <Dialog open onOpenChange={(o) => !o && !restore.isPending && onClose()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('settings.restore.title')}</DialogTitle>
          <DialogDescription className="break-all">
            {t('settings.restore.from', { path })}
          </DialogDescription>
        </DialogHeader>
        {contents ? (
          <div className="grid gap-3 text-sm">
            <p>
              {t('settings.restore.madeBy', {
                version: contents.appVersion,
                date: formatDateTime(contents.createdAt),
              })}
            </p>
            <ul className="list-disc pl-5" aria-label={t('settings.restore.holds')}>
              <li>
                {t('settings.restore.hosts', { count: contents.hosts.length })}
                {contents.hosts.length > 0 && `: ${contents.hosts.join(', ')}`}
              </li>
              {contents.clusters.length > 0 && (
                <li>
                  {t('settings.restore.clusters', { count: contents.clusters.length })}:{' '}
                  {contents.clusters.join(', ')}
                </li>
              )}
              <li>{t('settings.restore.stacks', { count: contents.stacks })}</li>
              <li>{t('settings.restore.registries', { count: contents.registries })}</li>
              <li>{t('settings.restore.repositories', { count: contents.repositories })}</li>
              {contents.settings && <li>{t('settings.restore.settings')}</li>}
            </ul>
            <Alert variant="destructive">
              <AlertDescription>{t('settings.restore.replaces')}</AlertDescription>
            </Alert>
          </div>
        ) : (
          <form
            className="grid gap-1.5"
            onSubmit={(e) => {
              e.preventDefault();
              void open();
            }}
          >
            <Label htmlFor="restore-passphrase">{t('settings.backup.passphrase')}</Label>
            <Input
              id="restore-passphrase"
              type="password"
              autoComplete="current-password"
              value={passphrase}
              onChange={(e) => setPassphrase(e.target.value)}
              autoFocus
            />
          </form>
        )}
        {preview.error && !contents && <ErrorMessage error={preview.error} />}
        {restore.error && <ErrorMessage error={restore.error} />}
        <DialogFooter>
          <Button variant="ghost" disabled={restore.isPending} onClick={onClose}>
            {t('common.cancel')}
          </Button>
          {contents ? (
            <Button
              variant="destructive"
              disabled={restore.isPending}
              onClick={() => void replace()}
            >
              {t('settings.restore.confirm')}
            </Button>
          ) : (
            <Button disabled={!passphrase || preview.isPending} onClick={() => void open()}>
              {t('settings.restore.open')}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
