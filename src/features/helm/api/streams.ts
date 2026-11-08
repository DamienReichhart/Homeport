import { useQueryClient } from '@tanstack/react-query';
import { useCallback } from 'react';
import type { Channel } from '@tauri-apps/api/core';
import {
  commands,
  subscribe,
  type AppError,
  type CliEvent,
  type InstallRequest,
  type RollbackRequest,
  type UninstallRequest,
  type UpgradeRequest,
} from '@/shared/api';
import { helmKeys } from './queries';

/** Starts a streamed helm run; returns its canceller. */
export type HelmRun = (
  onEvent: (event: CliEvent) => void,
  onError: (error: AppError) => void,
) => () => void;

/** Downloads the indexes of `names`, or of every repository; charts are searched afresh after. */
export function useUpdateRepositories(): (names: string[]) => HelmRun {
  const client = useQueryClient();
  return useCallback(
    (names) => (onEvent, onError) =>
      subscribe<CliEvent>(
        (ch) => commands.helmReposUpdateSubscribe(names, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'exit')
            void client.invalidateQueries({ queryKey: helmKeys.repositories });
        },
        onError,
      ),
    [client],
  );
}

/**
 * Runs helm commands that change a Cluster's releases, started by `start`;
 * its releases are read again when helm ends.
 */
function useClusterRun<R>(
  clusterId: string,
  start: (clusterId: string, request: R, channel: Channel<CliEvent>) => Promise<number>,
): (request: R) => HelmRun {
  const client = useQueryClient();
  return useCallback(
    (request) => (onEvent, onError) =>
      subscribe<CliEvent>(
        (ch) => start(clusterId, request, ch),
        (event) => {
          onEvent(event);
          if (event.type === 'exit')
            void client.invalidateQueries({ queryKey: helmKeys.all(clusterId) });
        },
        onError,
      ),
    [client, clusterId, start],
  );
}

/** Installs a chart on the Cluster. */
export const useInstall = (clusterId: string) =>
  useClusterRun<InstallRequest>(clusterId, commands.clustersHelmInstallSubscribe);

/** Upgrades a release to another chart version or values. */
export const useUpgrade = (clusterId: string) =>
  useClusterRun<UpgradeRequest>(clusterId, commands.clustersHelmUpgradeSubscribe);

/** Rolls a release back to one of its revisions. */
export const useRollback = (clusterId: string) =>
  useClusterRun<RollbackRequest>(clusterId, commands.clustersHelmRollbackSubscribe);

/** Uninstalls a release. */
export const useUninstall = (clusterId: string) =>
  useClusterRun<UninstallRequest>(clusterId, commands.clustersHelmUninstallSubscribe);
