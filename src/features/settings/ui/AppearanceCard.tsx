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
import { LOCALES, resolveLocale, SYSTEM_LANGUAGE, t } from '@/shared/i18n';

/** The name of the language "Match the system" shows, in that language. */
const systemLanguage = () => LOCALES.find((l) => l.tag === resolveLocale(SYSTEM_LANGUAGE))?.name;

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
        <Label htmlFor="language" className="mt-3">
          {t('settings.appearance.language')}
        </Label>
        <Select
          value={settings.language ?? SYSTEM_LANGUAGE}
          onValueChange={(language) => onChange({ ...settings, language })}
        >
          <SelectTrigger
            id="language"
            className="w-56"
            aria-label={t('settings.appearance.language')}
          >
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={SYSTEM_LANGUAGE}>
              {t('settings.language.system', { language: systemLanguage() })}
            </SelectItem>
            {LOCALES.map((l) => (
              // Each language in its own words, read in its own voice.
              <SelectItem key={l.tag} value={l.tag} lang={l.tag} dir={l.dir}>
                {l.name}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        {error && <ErrorMessage error={error} />}
      </CardContent>
    </Card>
  );
}
