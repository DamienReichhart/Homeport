import { useState } from 'react';
import type { Settings } from '@/shared/api';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Input,
  Label,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/** How often Synced Stacks' repositories are checked while Homeport is open. */
export function SyncCard({
  settings,
  onChange,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
}) {
  const saved = settings.syncIntervalMinutes ?? 5;
  const [minutes, setMinutes] = useState(String(saved));
  const commit = () => {
    const n = Math.round(Number(minutes));
    const next = Number.isFinite(n) && n >= 1 ? Math.min(n, 1440) : saved;
    setMinutes(String(next));
    if (next !== saved) onChange({ ...settings, syncIntervalMinutes: next });
  };
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.sync.title')}</CardTitle>
        <CardDescription>{t('stacks.synced.whileOpen')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-1.5">
        <Label htmlFor="sync-interval">{t('settings.sync.interval')}</Label>
        <div className="flex items-center gap-2">
          <Input
            id="sync-interval"
            type="number"
            min={1}
            max={1440}
            className="w-24"
            value={minutes}
            onChange={(e) => setMinutes(e.target.value)}
            onBlur={commit}
            onKeyDown={(e) => e.key === 'Enter' && commit()}
          />
          <span className="text-muted-foreground text-sm">{t('settings.sync.minutes')}</span>
        </div>
      </CardContent>
    </Card>
  );
}
