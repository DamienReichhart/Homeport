import { queryOptions } from '@tanstack/react-query';
import { commands, type Host } from '@/shared/api';
import { pickFile } from '@/shared/lib';
import { toastError } from '@/shared/ui';

export const hostFeatureKeys = {
  sshSetup: (hostId: string) => ['hosts', hostId, 'ssh-setup'] as const,
};

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
