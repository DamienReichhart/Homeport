import { queryOptions } from '@tanstack/react-query';
import { commands, type Host } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import { toastError } from '@/shared/ui';
import type { ImportSource } from '../model/importRows';

export const hostFeatureKeys = {
  sshSetup: (hostId: string) => ['hosts', hostId, 'ssh-setup'] as const,
  importPreview: (source: ImportSource) => ['hosts', 'import', source] as const,
};

/** What a source offers to import, read fresh each time the dialog opens. */
export const importPreviewQuery = (source: ImportSource) =>
  queryOptions({
    queryKey: hostFeatureKeys.importPreview(source),
    queryFn: () =>
      source === 'sshConfig'
        ? commands.hostsSshConfigPreview()
        : commands.hostsDockerContextsPreview(),
    staleTime: 0,
    gcTime: 0,
  });

/** The commands that authorize an SSH Host's key on its server; changes with the connection. */
export const sshSetupQuery = (host: Host) =>
  queryOptions({
    queryKey: [...hostFeatureKeys.sshSetup(host.id), host.connection],
    queryFn: () => commands.hostsSshSetup(host.id),
  });

/**
 * Asks for a file and reads it as text; `null` when cancelled or when the
 * read fails, which is reported as a toast.
 */
/** What a pasted Incus trust token says; a token that can't be used fails with why. */
export const trustTokenQuery = (token: string) =>
  queryOptions({
    queryKey: ['incus-trust-token', token],
    queryFn: () => commands.incusTrustTokenRead(token),
    enabled: token.trim() !== '',
    retry: false,
  });

export async function loadTextFile(title: string): Promise<string | null> {
  const path = await pickFile(title);
  if (!path) return null;
  try {
    return await commands.readTextFile(path);
  } catch (e) {
    toastError(e);
    return null;
  }
}
