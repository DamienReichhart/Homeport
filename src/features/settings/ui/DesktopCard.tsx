import { useQuery } from '@tanstack/react-query';
import type { Settings } from '@/shared/api';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Label,
  Switch,
} from '@/shared/ui';
import { machineQuery } from '../api/queries';
import { t } from '@/shared/i18n';

/** How Homeport sits on the desktop: its tray icon. */
export function DesktopCard({
  settings,
  onChange,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
}) {
  const machine = useQuery(machineQuery());
  // Until the machine is checked, assume a tray, so the switch doesn't flicker.
  const tray = machine.data?.tray ?? true;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.desktop.title')}</CardTitle>
        <CardDescription>{t('settings.desktop.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-1.5">
        <div className="flex items-center gap-2">
          <Switch
            id="close-to-tray"
            checked={(settings.closeToTray ?? false) && tray}
            disabled={!tray}
            aria-describedby="close-to-tray-hint"
            onCheckedChange={(closeToTray) => onChange({ ...settings, closeToTray })}
          />
          <Label htmlFor="close-to-tray">{t('settings.desktop.closeToTray')}</Label>
        </div>
        <p id="close-to-tray-hint" className="text-muted-foreground text-sm">
          {tray ? t('settings.desktop.closeToTrayHint') : t('settings.desktop.noTray')}
        </p>
      </CardContent>
    </Card>
  );
}
