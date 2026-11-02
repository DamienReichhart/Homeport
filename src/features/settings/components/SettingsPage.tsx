import { useState } from 'react';
import { toAppError } from '@/shared/api';
import { useAsync } from '@/shared/hooks';
import { applyTheme, type Theme } from '@/shared/lib';
import { ErrorMessage, Spinner } from '@/shared/ui';
import { settingsApi } from '../api/settingsApi';

const THEMES: Theme[] = ['system', 'light', 'dark'];

export function SettingsPage() {
  const settings = useAsync(settingsApi.get);
  const [saveError, setSaveError] = useState<Error | null>(null);

  if (settings.status === 'loading') return <Spinner />;
  if (settings.status === 'error') return <ErrorMessage error={settings.error} />;

  async function onThemeChange(theme: Theme) {
    setSaveError(null);
    try {
      const saved = await settingsApi.update({ ...settings.data, theme });
      applyTheme(saved.theme ?? 'system');
      settings.reload();
    } catch (error) {
      setSaveError(toAppError(error));
    }
  }

  return (
    <section>
      <h1>Settings</h1>
      <label>
        Theme{' '}
        <select
          value={settings.data.theme ?? 'system'}
          onChange={(e) => void onThemeChange(e.target.value as Theme)}
        >
          {THEMES.map((t) => (
            <option key={t} value={t}>
              {t}
            </option>
          ))}
        </select>
      </label>
      {saveError && <ErrorMessage error={saveError} />}
    </section>
  );
}
