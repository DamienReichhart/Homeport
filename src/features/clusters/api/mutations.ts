import { useMutation, useQueryClient } from '@tanstack/react-query';
import { toast } from 'sonner';
import { clusterKeys } from '@/core/cluster';
import { commands, type ClusterView, type ContextChoice, type NomadForm } from '@/shared/api';
import { t } from '@/shared/i18n';

function useRefreshClusters() {
  const client = useQueryClient();
  return () => client.invalidateQueries({ queryKey: clusterKeys.list });
}

/** Lists the contexts of a pasted kubeconfig; the page shows a failure in place. */
export function useReadPastedKubeconfig() {
  return useMutation({
    meta: { silent: true },
    mutationFn: (text: string) => commands.clustersKubeconfigContexts(text),
  });
}

/** Adds the picked contexts as Clusters; the page shows a failure in place. */
export function useImportClusters(onDone: (added: ClusterView[]) => void) {
  const refresh = useRefreshClusters();
  return useMutation({
    meta: { silent: true },
    mutationFn: ({ contexts, text }: { contexts: ContextChoice[]; text: string | null }) =>
      commands.clustersImport(contexts, text),
    onSuccess: async (added) => {
      await refresh();
      toast.success(t('clusters.import.done', { count: added.length }));
      onDone(added);
    },
  });
}

/** Reads a Cluster's context again from its kubeconfig, or from `text` when pasted. */
export function useRefreshCluster(clusterId: string) {
  const refresh = useRefreshClusters();
  return useMutation({
    meta: { silent: true },
    mutationFn: (text: string | null) => commands.clustersRefresh(clusterId, text),
    onSuccess: async () => {
      await refresh();
      toast.success(t('clusters.refresh.done'));
    },
  });
}

/** Renames a Cluster; the page shows a failure in place. */
export function useRenameCluster(clusterId: string) {
  const refresh = useRefreshClusters();
  return useMutation({
    meta: { silent: true },
    mutationFn: (name: string) => commands.clustersUpdate(clusterId, name),
    onSuccess: async () => {
      await refresh();
      toast.success(t('clusters.detail.renamed'));
    },
  });
}

/** Reconnecting and enabling or disabling a Cluster. */
export function useClusterConnection(clusterId: string) {
  const refresh = useRefreshClusters();
  const reconnect = useMutation({
    mutationFn: () => commands.clustersReconnect(clusterId),
    onSuccess: refresh,
  });
  const enable = useMutation({
    mutationFn: (enabled: boolean) => commands.clustersSetEnabled(clusterId, enabled),
    onSuccess: refresh,
  });
  return { reconnect, enable };
}

/** Forgets a Cluster and its credentials. */
export function useRemoveCluster(clusterId: string) {
  const refresh = useRefreshClusters();
  return useMutation({
    mutationFn: () => commands.clustersRemove(clusterId),
    onSuccess: refresh,
  });
}

/** Refreshes the Cluster made from a context, from the kubeconfig the import page reads. */
export function useRefreshFromImport() {
  const refresh = useRefreshClusters();
  return useMutation({
    mutationFn: ({ clusterId, text }: { clusterId: string; text: string | null }) =>
      commands.clustersRefresh(clusterId, text),
    onSuccess: async () => {
      await refresh();
      toast.success(t('clusters.refresh.done'));
    },
  });
}

/** Adds a Nomad Cluster; the form shows a failure in place. */
export function useAddNomadCluster(onDone: (added: ClusterView) => void) {
  const refresh = useRefreshClusters();
  return useMutation({
    meta: { silent: true },
    mutationFn: (form: NomadForm) => commands.clustersNomadAdd(form),
    onSuccess: async (added) => {
      await refresh();
      toast.success(t('clusters.nomad.added', { name: added.cluster.name }));
      onDone(added);
    },
  });
}

/** Changes a Nomad Cluster's settings and credentials; the form shows a failure in place. */
export function useUpdateNomadCluster(clusterId: string) {
  const refresh = useRefreshClusters();
  return useMutation({
    meta: { silent: true },
    mutationFn: (form: NomadForm) => commands.clustersNomadUpdate(clusterId, form),
    onSuccess: async () => {
      await refresh();
      toast.success(t('clusters.nomad.saved'));
    },
  });
}

/**
 * Connects once to the Nomad Cluster a form describes, without saving it;
 * with `clusterId`, the credentials left empty are that Cluster's.
 */
export function useTestNomadCluster(clusterId: string | null) {
  return useMutation({
    meta: { silent: true },
    mutationFn: (form: NomadForm) => commands.clustersNomadTest(form, clusterId),
  });
}
