import { queryOptions } from '@tanstack/react-query';
import { commands } from '@/shared/api';

export const stackKeys = {
  /** A Managed Stack's stored definition; its environment is cached under it. */
  managed: (managedId: string) => ['managed-stack', managedId] as const,
};

export const stackQueries = {
  managed: (managedId: string) =>
    queryOptions({
      queryKey: stackKeys.managed(managedId),
      queryFn: () => commands.stacksGet(managedId),
    }),
  env: (managedId: string) =>
    queryOptions({
      queryKey: [...stackKeys.managed(managedId), 'env'],
      queryFn: () => commands.stacksGetEnv(managedId),
    }),
};
