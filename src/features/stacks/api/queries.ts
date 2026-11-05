import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands, type Candidate, type SourceKind } from '@/shared/api';

export const stackKeys = {
  /** A Managed Stack's stored definition; its environment is cached under it. */
  managed: (managedId: string) => ['managed-stack', managedId] as const,
  /** A Linked Stack's compose files, as they are in its folder. */
  linked: (linkedId: string) => ['linked-stack', linkedId] as const,
  /** A Stack's deploys; under the Host, so the end of a compose run refreshes it. */
  history: (hostId: string, stackId: string) =>
    [...hostKeys.scope(hostId), 'stack-history', stackId] as const,
  deploy: (deployId: string) => ['stack-deploy', deployId] as const,
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
  history: (hostId: string, stackId: string) =>
    queryOptions({
      queryKey: stackKeys.history(hostId, stackId),
      queryFn: () => commands.stacksHistory(stackId),
    }),
  deploy: (deployId: string) =>
    queryOptions({
      queryKey: stackKeys.deploy(deployId),
      queryFn: () => commands.stacksDeploy(deployId),
      // A recorded deploy never changes.
      staleTime: Infinity,
    }),
  /** What redeploying would change; `candidate` null compares the stored definition. */
  diff: (kind: SourceKind, stackId: string, candidate: Candidate | null) =>
    queryOptions({
      queryKey: ['stack-diff', kind, stackId, candidate] as const,
      queryFn: () => commands.stacksDiff(kind, stackId, candidate),
      // Always against what is deployed right now.
      staleTime: 0,
      gcTime: 0,
      retry: false,
    }),
  env: (managedId: string) =>
    queryOptions({
      queryKey: [...stackKeys.managed(managedId), 'env'],
      queryFn: () => commands.stacksGetEnv(managedId),
    }),
};
