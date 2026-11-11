import { useMutation, useQueryClient, type QueryClient } from '@tanstack/react-query';
import { applyLanguage, patchSettings, saveSettings } from '@/core/settings';
import { commands, type Settings } from '@/shared/api';
import { applyTheme, type Theme } from '@/shared/lib';
import { toastError } from '@/shared/ui';
import { startAtLoginKey } from './queries';

/** Saves the settings and applies the theme and language; the page shows a failure in place. */
export function useSaveSettings() {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (next: Settings) => saveSettings(client, next),
    onSuccess: async (saved) => {
      applyTheme(saved.theme ?? 'system');
      await applyLanguage(saved.language);
    },
  });
}

/** Opens the folder Homeport writes its log to, reporting a failure as a toast. */
export function openLogFolder() {
  commands.openLogFolder().catch(toastError);
}

/** Adds or removes the OS's login entry; the switch shows what the OS then has. */
export function useSetStartAtLogin() {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (enabled: boolean) => commands.appSetStartAtLogin(enabled),
    onSuccess: (enabled) => client.setQueryData(startAtLoginKey, enabled),
  });
}

/** Saves `theme` and applies it, as the palette's Theme commands do. */
export async function setTheme(client: QueryClient, theme: Theme) {
  const saved = await patchSettings(client, { theme });
  applyTheme(saved.theme ?? 'system');
}

/** Writes the diagnostics bundle to a path; the dialog shows a failure in place. */
export function useExportDiagnostics() {
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ path, includeNames }: { path: string; includeNames: boolean }) =>
      commands.diagnosticsExport(path, includeNames),
  });
}

/**
 * Gives up on the key the stored secrets were sealed with, whose keychain
 * entry is missing, then refreshes everything that depends on secrets.
 */
export function useKeychainStartOver() {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: () => commands.keychainStartOver(),
    onSuccess: () => client.invalidateQueries(),
  });
}
