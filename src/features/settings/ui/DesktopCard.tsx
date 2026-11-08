import { useQuery } from '@tanstack/react-query';
import type { Settings } from '@/shared/api';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Label,
  Switch,
} from '@/shared/ui';
import { useSetStartAtLogin } from '../api/mutations';
import { machineQuery, startAtLoginQuery } from '../api/queries';
import { t } from '@/shared/i18n';

/** How Homeport sits on the desktop: its tray icon, and starting at login. */
export function DesktopCard({
  settings,
  onChange,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
}) {
  const machine = useQuery(machineQuery());
  const startAtLogin = useQuery(startAtLoginQuery());
  const setStartAtLogin = useSetStartAtLogin();
  // Until the machine is checked, assume a tray, so the switch doesn't flicker.
  const tray = machine.data?.tray ?? true;
  const atLogin = startAtLogin.data ?? false;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.desktop.title')}</CardTitle>
        <CardDescription>{t('settings.desktop.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <Toggle
          id="close-to-tray"
          label={t('settings.desktop.closeToTray')}
          hint={tray ? t('settings.desktop.closeToTrayHint') : t('settings.desktop.noTray')}
          checked={(settings.closeToTray ?? false) && tray}
          disabled={!tray}
          onChange={(closeToTray) => onChange({ ...settings, closeToTray })}
        />
        <Toggle
          id="start-at-login"
          label={t('settings.desktop.startAtLogin')}
          hint={t('settings.desktop.startAtLoginHint')}
          checked={atLogin}
          disabled={startAtLogin.isPending || setStartAtLogin.isPending}
          onChange={(enabled) => setStartAtLogin.mutate(enabled)}
        />
        <Toggle
          id="start-hidden"
          label={t('settings.desktop.startHidden')}
          hint={
            tray ? t('settings.desktop.startHiddenHint') : t('settings.desktop.startHiddenNoTray')
          }
          checked={(settings.startHidden ?? false) && tray}
          disabled={!atLogin || !tray}
          onChange={(startHidden) => onChange({ ...settings, startHidden })}
        />
        {(startAtLogin.error ?? setStartAtLogin.error) && (
          <ErrorMessage
            error={startAtLogin.error ?? setStartAtLogin.error}
            onRetry={startAtLogin.error ? () => void startAtLogin.refetch() : undefined}
          />
        )}
      </CardContent>
    </Card>
  );
}

function Toggle({
  id,
  label,
  hint,
  checked,
  disabled,
  onChange,
}: {
  id: string;
  label: string;
  hint: string;
  checked: boolean;
  disabled: boolean;
  onChange: (checked: boolean) => void;
}) {
  return (
    <div className="grid gap-1.5">
      <div className="flex items-center gap-2">
        <Switch
          id={id}
          checked={checked}
          disabled={disabled}
          aria-describedby={`${id}-hint`}
          onCheckedChange={onChange}
        />
        <Label htmlFor={id}>{label}</Label>
      </div>
      <p id={`${id}-hint`} className="text-muted-foreground text-sm">
        {hint}
      </p>
    </div>
  );
}
