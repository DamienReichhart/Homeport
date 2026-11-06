import type { NotificationSettings, Settings } from '@/shared/api';
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Label,
  Switch,
} from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';
import { t } from '@/shared/i18n';

const KINDS: { key: keyof NotificationSettings; label: MessageKey }[] = [
  { key: 'containerDied', label: 'settings.notifications.containerDied' },
  { key: 'containerUnhealthy', label: 'settings.notifications.containerUnhealthy' },
  { key: 'hostUnreachable', label: 'settings.notifications.hostUnreachable' },
  { key: 'hostBack', label: 'settings.notifications.hostBack' },
];

/** Which OS notifications Homeport shows, one switch per kind. */
export function NotificationsCard({
  settings,
  onChange,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
}) {
  const current = settings.notifications;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.notifications.title')}</CardTitle>
        <CardDescription>{t('settings.notifications.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-3">
        {KINDS.map(({ key, label }) => (
          <div key={key} className="flex items-center gap-2">
            <Switch
              id={`notify-${key}`}
              checked={current?.[key] ?? true}
              onCheckedChange={(on) =>
                onChange({
                  ...settings,
                  notifications: { ...allOn, ...current, [key]: on },
                })
              }
            />
            <Label htmlFor={`notify-${key}`}>{t(label)}</Label>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}

const allOn: NotificationSettings = {
  containerDied: true,
  containerUnhealthy: true,
  hostUnreachable: true,
  hostBack: true,
};
