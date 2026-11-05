import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import {
  commands,
  type ComposeFile,
  type FolderProject,
  type LinkedFiles,
  type LinkedStack,
  type ManagedStack,
  type SourceKind,
  type StackAction,
} from '@/shared/api';
import { toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';
import { stackKeys } from './queries';

/** Start, stop, restart and remove for any Stack, through the Engine API. */
export function useStackMutations(hostId: string) {
  const client = useQueryClient();
  const refresh = () =>
    Promise.all([
      client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) }),
      client.invalidateQueries({ queryKey: hostKeys.containers(hostId) }),
    ]);
  const act = useMutation({
    mutationFn: ({ name, action }: { name: string; action: StackAction }) =>
      commands.stacksAct(hostId, name, action),
    onSuccess: refresh,
    onError: (e) => toastError(e),
  });
  const remove = useMutation({
    mutationFn: (name: string) => commands.stacksRemove(hostId, name),
    onSuccess: async (_, name) => {
      toast.success(t('stacks.removed', { name }));
      await refresh();
    },
    onError: (e) => toastError(e),
  });
  return { act, remove };
}

interface NewStack {
  name: string;
  files: ComposeFile[];
  env: string;
  deploy: boolean;
}

/**
 * Stores a new Managed Stack. `onSaved` runs once the Stack list has it,
 * with whether the user asked to deploy it too. The page shows a failure in
 * place.
 */
export function useCreateStack(
  hostId: string,
  onSaved: (stack: ManagedStack, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ name, files, env }: NewStack) => commands.stacksCreate(hostId, name, files, env),
    onSuccess: async (stack, { deploy }) => {
      await client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) });
      onSaved(stack, deploy);
    },
  });
}

/** Tells what Compose would make of a folder picked for a Linked Stack. */
export function useInspectFolder(onFound: (project: FolderProject) => void) {
  return useMutation({
    meta: { silent: true },
    mutationFn: (folder: string) => commands.stacksLinkedInspect(folder),
    onSuccess: onFound,
  });
}

interface NewLinked {
  name: string;
  workingDir: string;
  configFiles: string[];
  deploy: boolean;
}

/** Records a new Linked Stack; `onSaved` runs once the Stack list has it. */
export function useCreateLinked(
  hostId: string,
  onSaved: (stack: LinkedStack, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ name, workingDir, configFiles }: NewLinked) =>
      commands.stacksLinkedCreate(hostId, name, workingDir, configFiles),
    onSuccess: async (stack, { deploy }) => {
      await client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) });
      onSaved(stack, deploy);
    },
  });
}

interface Definition {
  files: ComposeFile[];
  env: string;
  deploy: boolean;
}

/** Stores a Managed Stack's edited definition; `onSaved` gets whether to redeploy. */
export function useSaveDefinition(
  managedId: string,
  onSaved: (stack: ManagedStack, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ files, env }: Definition) => commands.stacksUpdate(managedId, files, env),
    onSuccess: async (saved, { deploy }) => {
      await client.invalidateQueries({ queryKey: stackKeys.managed(managedId) });
      toast.success(t('stacks.definitionSaved'));
      onSaved(saved, deploy);
    },
  });
}

/** Reads a Linked Stack's files again and notes them as seen. */
export function useReloadLinked(linkedId: string, onLoaded: (files: LinkedFiles) => void) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: () => commands.stacksLinkedRead(linkedId, true),
    onSuccess: (files) => {
      client.setQueryData(stackKeys.linked(linkedId), files);
      onLoaded(files);
    },
  });
}

interface LinkedWrite {
  files: { path: string; content: string; expected: string | null }[];
  deploy: boolean;
}

/**
 * Writes edited compose files of a Linked Stack in their folder, one after
 * the other. A file changed on disk since its `expected` fingerprint is
 * refused unless `expected` is null. `onSaved` gets the files as written.
 */
export function useWriteLinked(
  linkedId: string,
  onSaved: (files: LinkedFiles, deploy: boolean) => void,
) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: async ({ files }: LinkedWrite) => {
      let result: LinkedFiles | null = null;
      for (const f of files)
        result = await commands.stacksLinkedWrite(linkedId, f.path, f.content, f.expected);
      return result;
    },
    onSuccess: (files, { deploy }) => {
      if (!files) return;
      client.setQueryData(stackKeys.linked(linkedId), files);
      toast.success(t('stacks.linked.saved'));
      onSaved(files, deploy);
    },
  });
}

/** Sets which compose files of its folder a Linked Stack uses, in order. */
export function useSetLinkedFiles(linkedId: string, onSaved: (files: LinkedFiles) => void) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (configFiles: string[]) => commands.stacksLinkedSetFiles(linkedId, configFiles),
    onSuccess: (files) => {
      client.setQueryData(stackKeys.linked(linkedId), files);
      onSaved(files);
    },
  });
}

/** Sets the Compose profiles a Managed or Linked Stack deploys. */
export function useSetProfiles(kind: SourceKind, stackId: string) {
  const client = useQueryClient();
  return useMutation({
    meta: { silent: true },
    mutationFn: (profiles: string[]) => commands.stacksSetProfiles(kind, stackId, profiles),
    onSuccess: async () => {
      await client.invalidateQueries({
        queryKey: kind === 'managed' ? stackKeys.managed(stackId) : stackKeys.linked(stackId),
      });
      toast.success(t('stacks.profiles.saved'));
    },
  });
}

/** Forgets a Linked Stack: its folder and containers are left alone. */
export function useForgetLinked(hostId: string, linkedId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => commands.stacksLinkedForget(linkedId),
    onSuccess: () => client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) }),
  });
}

/** Deletes a Managed Stack's stored definition, leaving its containers: it becomes External. */
export function useForgetDefinition(hostId: string, managedId: string) {
  const client = useQueryClient();
  return useMutation({
    mutationFn: () => commands.stacksForget(managedId),
    onSuccess: () => client.invalidateQueries({ queryKey: hostKeys.stacks(hostId) }),
  });
}
