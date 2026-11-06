import { useMutation, useQueryClient } from '@tanstack/react-query';
import { commands } from '@/shared/api';
import { applyTheme } from '@/shared/lib';

/** Backs up everything Homeport holds; the dialog shows a failure in place. */
export function useBackupData() {
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ path, passphrase }: { path: string; passphrase: string }) =>
      commands.dataBackup(path, passphrase),
  });
}

/** Opens a backup to list what it holds; the dialog shows a failure in place. */
export function usePreviewRestore() {
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ path, passphrase }: { path: string; passphrase: string }) =>
      commands.dataRestorePreview(path, passphrase),
  });
}

/**
 * Replaces everything Homeport holds with a backup's, then refreshes every
 * list, since every Host, Stack and setting may have changed.
 */
export function useRestoreData() {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ path, passphrase }: { path: string; passphrase: string }) =>
      commands.dataRestore(path, passphrase),
    onSuccess: async () => {
      await client.invalidateQueries();
      applyTheme((await commands.getSettings()).theme ?? 'system');
    },
  });
}
