import { ArchiveIcon, ArchiveRestoreIcon } from 'lucide-react';
import { useState } from 'react';
import type { Settings } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Input,
  Label,
} from '@/shared/ui';
import { BACKUP_EXTENSION } from '../model/backup';
import { BackupDataDialog } from './dialogs/BackupDataDialog';
import { RestoreDataDialog } from './dialogs/RestoreDataDialog';
import { t } from '@/shared/i18n';

/**
 * What Homeport keeps on this computer: backing it all up and restoring
 * it, and how long Engine events are kept.
 */
export function DataCard({
  settings,
  onChange,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
}) {
  const saved = settings.keepEventsDays ?? 30;
  const [days, setDays] = useState(String(saved));
  const [backingUp, setBackingUp] = useState(false);
  const [restoring, setRestoring] = useState<string | null>(null);
  const restoreFrom = async () => {
    const path = await pickFile(t('settings.restore.pick'), [
      { name: t('settings.backup.fileType'), extensions: [BACKUP_EXTENSION] },
    ]);
    if (path) setRestoring(path);
  };
  const commit = () => {
    const n = Math.round(Number(days));
    const next = Number.isFinite(n) && n >= 1 ? Math.min(n, 365) : saved;
    setDays(String(next));
    if (next !== saved) onChange({ ...settings, keepEventsDays: next });
  };
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.data.title')}</CardTitle>
        <CardDescription>{t('settings.data.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <div className="grid gap-1.5">
          <p className="text-sm">{t('settings.data.backupHint')}</p>
          <div className="flex flex-wrap gap-2">
            <Button variant="outline" onClick={() => setBackingUp(true)}>
              <ArchiveIcon /> {t('settings.data.backup')}
            </Button>
            <Button variant="outline" onClick={() => void restoreFrom()}>
              <ArchiveRestoreIcon /> {t('settings.data.restore')}
            </Button>
          </div>
        </div>
        <div className="grid gap-1.5">
          <Label htmlFor="keep-events">{t('settings.data.keepEvents')}</Label>
          <div className="flex items-center gap-2">
            <Input
              id="keep-events"
              type="number"
              min={1}
              max={365}
              className="w-24"
              value={days}
              onChange={(e) => setDays(e.target.value)}
              onBlur={commit}
              onKeyDown={(e) => e.key === 'Enter' && commit()}
            />
            <span className="text-muted-foreground text-sm">{t('settings.data.days')}</span>
          </div>
          <p className="text-muted-foreground text-sm">{t('settings.data.keepEventsHint')}</p>
        </div>
      </CardContent>
      {backingUp && <BackupDataDialog onClose={() => setBackingUp(false)} />}
      {restoring && <RestoreDataDialog path={restoring} onClose={() => setRestoring(null)} />}
    </Card>
  );
}
