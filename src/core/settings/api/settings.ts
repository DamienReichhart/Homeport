import { queryOptions, type QueryClient } from '@tanstack/react-query';
import { commands, type Settings } from '@/shared/api';

export const settingsKey = ['settings'] as const;

/** The user's settings, as the backend stores them. */
export const settingsQuery = () =>
  queryOptions({ queryKey: settingsKey, queryFn: () => commands.getSettings() });

/** Stores `settings` and keeps the cached copy in step. */
export async function saveSettings(client: QueryClient, settings: Settings): Promise<Settings> {
  const saved = await commands.updateSettings(settings);
  client.setQueryData(settingsKey, saved);
  return saved;
}

/** Changes some settings and keeps the others as they are stored. */
export async function patchSettings(
  client: QueryClient,
  patch: Partial<Settings>,
): Promise<Settings> {
  const current = await commands.getSettings();
  return saveSettings(client, { ...current, ...patch });
}
