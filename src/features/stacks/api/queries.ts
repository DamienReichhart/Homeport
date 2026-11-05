import { queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';

export const stackKeys = {
  /** A Managed Stack's stored definition; its environment is cached under it. */
  managed: (managedId: string) => ['managed-stack', managedId] as const,
  /** A Linked Stack's compose files, as they are in its folder. */
  linked: (linkedId: string) => ['linked-stack', linkedId] as const,
};

export const stackQueries = {
  managed: (managedId: string) =>
    queryOptions({
      queryKey: stackKeys.managed(managedId),
      queryFn: () => commands.stacksGet(managedId),
    }),
  linked: (linkedId: string) =>
    queryOptions({
      queryKey: stackKeys.linked(linkedId),
      queryFn: () => commands.stacksLinkedRead(linkedId, false),
      // The user edits these files in their own editor too.
      refetchOnWindowFocus: 'always',
      // A missing folder or file won't appear by asking again.
      retry: false,
    }),
  env: (managedId: string) =>
    queryOptions({
      queryKey: [...stackKeys.managed(managedId), 'env'],
      queryFn: () => commands.stacksGetEnv(managedId),
    }),
};
