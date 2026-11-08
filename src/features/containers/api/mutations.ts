import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import { commands, type CommitSpec, type ContainerAction, type FileEntry } from '@/shared/api';
import { toastError } from '@/shared/ui';
import { toRunSpec, type RunForm } from '../model/runForm';
import { t, type MessageKey } from '@/shared/i18n';

/** The toast after an action, by action; each message takes the number of containers. */
const DONE: Record<ContainerAction, MessageKey> = {
  start: 'containers.done.start',
  stop: 'containers.done.stop',
  restart: 'containers.done.restart',
  pause: 'containers.done.pause',
  unpause: 'containers.done.unpause',
  kill: 'containers.done.kill',
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
      toast.success(t(DONE[action], { count: ids.length }));
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
      toast.success(t('containers.done.remove', { count: ids.length }));
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
    mutationFn: (form: RunForm) => commands.containersRun(hostId, toRunSpec(form), null),
    onSuccess: async (id) => {
      toast.success(t('containers.run.started'));
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
      toast.success(t('containers.edit.recreated'));
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
      toast.success(t('containers.commit.done', { reference: spec.reference }), {
        action: { label: t('containers.commit.openImages'), onClick: openImages },
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
      toast.success(
        archived
          ? t('containers.files.savedArchive', { name: entry.name })
          : t('containers.files.saved', { name: entry.name }),
      ),
    onError: (e) => toastError(e),
  });
  const upload = useMutation({
    mutationFn: async ({ path, sources }: { path: string; sources: string[] }) =>
      commands.filesUpload(hostId, containerId, path, sources),
    onSuccess: (_, { path, sources }) => {
      toast.success(t('containers.files.uploaded', { count: sources.length, path }));
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
    onSuccess: () => toast.success(t('containers.logs.saved')),
    onError: (e) => toastError(e),
  });
}
