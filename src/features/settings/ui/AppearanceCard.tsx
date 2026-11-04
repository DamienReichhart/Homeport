import type { Settings } from '@/shared/api';
import type { Theme } from '@/shared/lib';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/shared/ui';
import { THEMES } from '../model/themes';
import { t } from '@/shared/i18n';

export function AppearanceCard({
  settings,
  onChange,
  error,
}: {
  settings: Settings;
  onChange: (next: Settings) => void;
  /** Why the last save failed. */
  error: Error | null;
}) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('settings.appearance.title')}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-1.5">
        <Label htmlFor="theme">{t('settings.appearance.theme')}</Label>
        <Select
          value={settings.theme ?? 'system'}
          onValueChange={(theme) => onChange({ ...settings, theme: theme as Theme })}
        >
          <SelectTrigger id="theme" className="w-56" aria-label={t('settings.appearance.theme')}>
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {THEMES.map((theme) => (
              <SelectItem key={theme.value} value={theme.value}>
                {t(theme.label)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        {error && <ErrorMessage error={error} />}
      </CardContent>
    </Card>
  );
}
