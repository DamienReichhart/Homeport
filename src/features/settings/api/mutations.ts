import { useMutation, useQueryClient } from '@tanstack/react-query';
import { saveSettings } from '@/core/settings';
import { commands, type Settings } from '@/shared/api';
import { applyTheme } from '@/shared/lib';
import { toastError } from '@/shared/ui';
import { startAtLoginKey } from './queries';

/** Saves the settings and applies the theme; the page shows a failure in place. */
export function useSaveSettings() {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (next: Settings) => saveSettings(client, next),
    onSuccess: (saved) => applyTheme(saved.theme ?? 'system'),
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
