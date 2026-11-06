import { TriangleAlertIcon } from 'lucide-react';
import { useState } from 'react';
import { toast } from 'sonner';
import type { BackupOutcome } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import {
  Alert,
  AlertDescription,
  AlertTitle,
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
import { useBackupData } from '../../api/backup';
import {
  BACKUP_EXTENSION,
  backupFileName,
  MIN_PASSPHRASE,
  passphraseProblem,
} from '../../model/backup';
import { t } from '@/shared/i18n';

/** Asks for a passphrase twice and where to save, then backs everything up. */
export function BackupDataDialog({ onClose }: { onClose: () => void }) {
  const [passphrase, setPassphrase] = useState('');
  const [confirmation, setConfirmation] = useState('');
  const [done, setDone] = useState<BackupOutcome | null>(null);
  const backup = useBackupData();
  const problem = passphraseProblem(passphrase, confirmation);

  const run = async () => {
    const path = await pickSavePath(t('settings.backup.saveTitle'), backupFileName(), [
      { name: t('settings.backup.fileType'), extensions: [BACKUP_EXTENSION] },
    ]);
    if (!path) return;
    const outcome = await backup.mutateAsync({ path, passphrase }).catch(() => null);
    if (!outcome) return;
    toast.success(
      t('settings.backup.done', {
        hosts: outcome.contents.hosts.length,
        stacks: outcome.contents.stacks,
      }),
    );
    if (outcome.withoutSecrets.length === 0) onClose();
    else setDone(outcome);
  };

  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('settings.backup.title')}</DialogTitle>
          <DialogDescription>{t('settings.backup.description')}</DialogDescription>
        </DialogHeader>
        {done ? (
          <Alert>
            <TriangleAlertIcon />
            <AlertTitle>{t('settings.backup.withoutSecrets')}</AlertTitle>
            <AlertDescription>{done.withoutSecrets.join(', ')}</AlertDescription>
          </Alert>
        ) : (
          <div className="grid gap-3 sm:grid-cols-2">
            <div className="grid gap-1.5">
              <Label htmlFor="backup-passphrase">{t('settings.backup.passphrase')}</Label>
              <Input
                id="backup-passphrase"
                type="password"
                autoComplete="new-password"
                value={passphrase}
                onChange={(e) => setPassphrase(e.target.value)}
              />
              <p className="text-muted-foreground text-xs">
                {t('settings.backup.passphraseHint', { min: MIN_PASSPHRASE })}
              </p>
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="backup-confirmation">{t('settings.backup.confirmation')}</Label>
              <Input
                id="backup-confirmation"
                type="password"
                autoComplete="new-password"
                value={confirmation}
                onChange={(e) => setConfirmation(e.target.value)}
              />
            </div>
          </div>
        )}
        {!done && passphrase && problem && (
          <p className="text-muted-foreground text-sm">{t(problem, { min: MIN_PASSPHRASE })}</p>
        )}
        {backup.error && <ErrorMessage error={backup.error} />}
        <DialogFooter>
          {done ? (
            <Button onClick={onClose}>{t('common.close')}</Button>
          ) : (
            <>
              <Button variant="ghost" onClick={onClose}>
                {t('common.cancel')}
              </Button>
              <Button disabled={problem !== null || backup.isPending} onClick={() => void run()}>
                {t('settings.backup.start')}
              </Button>
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
