import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type CommitSpec, type ContainerAction, type FileEntry } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { toRunSpec, type RunForm } from '../model/runForm';

const PAST: Record<ContainerAction, string> = {
  start: 'Started',
  stop: 'Stopped',
  restart: 'Restarted',
  pause: 'Paused',
  unpause: 'Resumed',
  kill: 'Killed',
};

function useRefreshContainers(hostId: string) {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: hostKeys.containers(hostId) });
}

/** Lifecycle mutations for containers, reporting the outcome as a toast. */
export function useContainerMutations(hostId: string) {
  const refresh = useRefreshContainers(hostId);
  const act = useMutation({
    mutationFn: async ({ ids, action }: { ids: string[]; action: ContainerAction }) => {
      await Promise.all(ids.map((id) => commands.containersAct(hostId, id, action)));
      return { ids, action };
    },
    onSuccess: ({ ids, action }) => {
      toast.success(
        `${PAST[action]} ${ids.length === 1 ? 'container' : `${ids.length} containers`}`,
      );
      void refresh();
    },
    onError: (e) => toastError(e),
  });
  const remove = useMutation({
    mutationFn: async ({ ids, volumes }: { ids: string[]; volumes: boolean }) => {
      await Promise.all(ids.map((id) => commands.containersRemove(hostId, id, true, volumes)));
      return ids;
    },
    onSuccess: (ids) => {
      toast.success(`Removed ${ids.length === 1 ? 'container' : `${ids.length} containers`}`);
      void refresh();
    },
    onError: (e) => toastError(e),
  });
  return { act, remove, refresh };
}

/** Renames a container; the dialog shows a failure in place. */
export function useRenameContainer(hostId: string, containerId: string, onRenamed: () => void) {
  const refresh = useRefreshContainers(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (name: string) => commands.containersRename(hostId, containerId, name),
    onSuccess: async () => {
      await refresh();
      onRenamed();
    },
  });
}

/** Creates and starts a container; `onStarted` gets its id. */
export function useRunContainer(hostId: string, onStarted: (id: string) => void) {
  const refresh = useRefreshContainers(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (form: RunForm) => commands.containersRun(hostId, toRunSpec(form)),
    onSuccess: async (id) => {
      toast.success('Container started');
      await refresh();
      onStarted(id);
    },
  });
}

/** Recreates a container from an edited form (Edit); `onDone` gets the new id. */
export function useRecreateContainer(
  hostId: string,
  containerId: string,
  onDone: (id: string) => void,
) {
  const refresh = useRefreshContainers(hostId);
  return useMutation({
    meta: { silent: true },
    mutationFn: (form: RunForm) =>
      commands.containersRecreate(hostId, containerId, toRunSpec(form)),
    onSuccess: async (id) => {
      toast.success('Container recreated');
      await refresh();
      onDone(id);
    },
  });
}

/**
 * Commits a container to an image, refreshing the images list; the dialog
 * shows a failure in place. The toast links to the Images page.
 */
export function useCommitContainer(
  hostId: string,
  containerId: string,
  onDone: () => void,
  openImages: () => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (spec: CommitSpec) => commands.containersCommit(hostId, containerId, spec),
    onSuccess: async (_, spec) => {
      toast.success(`Committed to ${spec.reference}`, {
        action: { label: 'Open images', onClick: openImages },
      });
      await client.invalidateQueries({ queryKey: hostKeys.images(hostId) });
      onDone();
    },
  });
}

/** Copies files between this machine and a container, reporting the outcome as a toast. */
export function useFileTransfers(hostId: string, containerId: string, onUploaded: () => void) {
  const download = useMutation({
    mutationFn: async ({ entry, target }: { entry: FileEntry; target: string }) =>
      commands.filesDownload(hostId, containerId, entry.path, target),
    onSuccess: (archived, { entry }) =>
      toast.success(archived ? `Saved ${entry.name} as a .tar archive` : `Saved ${entry.name}`),
    onError: (e) => toastError(e),
  });
  const upload = useMutation({
    mutationFn: async ({ path, sources }: { path: string; sources: string[] }) =>
      commands.filesUpload(hostId, containerId, path, sources),
    onSuccess: (_, { path, sources }) => {
      toast.success(`Uploaded ${sources.length} file${sources.length === 1 ? '' : 's'} to ${path}`);
      onUploaded();
    },
    onError: (e) => toastError(e),
  });
  return { download, upload };
}

/** Writes the lines a log viewer kept to a file. */
export function useSaveLogs() {
  return useMutation({
    mutationFn: ({ path, lines }: { path: string; lines: string[] }) =>
      commands.writeTextFile(path, `${lines.join('\n')}\n`),
    onSuccess: () => toast.success('Logs saved'),
    onError: (e) => toastError(e),
  });
}
