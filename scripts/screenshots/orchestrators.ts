/**
 * The IPC answers for the Swarm and Cluster screenshots, spread into the
 * mocks of `mocks.ts`. Cluster data only shows with `demo=clusters`, and
 * the demo Nomad Cluster's with `demo=nomad`, so the Host shots stay as they
 * were.
 */
import type { ResourceKind, WorkloadKind } from '@/shared/api';
import * as k from './clusters';
import * as n from './nomad';
import {
  joinInfo,
  serviceInspect,
  swarmConfigs,
  swarmNodes,
  swarmSecrets,
  swarmStacks,
  tasksOf,
} from './swarm';

type Handler = (args: Record<string, unknown>) => unknown;

/** Whether this capture shows the demo Clusters. */
export const clusterDemo = () => new URLSearchParams(location.search).get('demo') === 'clusters';

/** Whether this capture shows the demo Nomad Cluster. */
export const nomadDemo = () => new URLSearchParams(location.search).get('demo') === 'nomad';

export const swarmHandlers: Record<string, Handler> = {
  swarm_nodes_list: () => swarmNodes,
  swarm_tasks_list: (a) => tasksOf(a.filter as Parameters<typeof tasksOf>[0]),
  swarm_stacks_list: () => swarmStacks,
  swarm_stacks_parts: () => ({
    networks: ['shop_default'],
    secrets: ['shop_db_password'],
    configs: ['shop_nginx_conf'],
  }),
  swarm_secrets_list: () => swarmSecrets,
  swarm_configs_list: () => swarmConfigs,
  swarm_join_info: () => joinInfo,
  swarm_services_inspect: (a) => serviceInspect(a.id as string),
  swarm_networks_list: () => [],
};

const inNamespace = <T extends { namespace: string }>(list: T[], ns: unknown) =>
  ns ? list.filter((x) => x.namespace === ns) : list;

const YAML = (
  kind: string,
  name: string,
  ns: unknown,
) => `apiVersion: ${kind === 'Deployment' ? 'apps/v1' : 'v1'}
kind: ${kind}
metadata:
  name: ${name}
  namespace: ${String(ns ?? 'default')}
  labels:
    app: ${name.split('-')[0]}
    app.kubernetes.io/part-of: shop
spec:
  replicas: 3
  selector:
    matchLabels:
      app: ${name.split('-')[0]}
  template:
    spec:
      containers:
        - name: ${name.split('-')[0]}
          image: ghcr.io/acme/shop-api:2.5.0
          ports:
            - containerPort: 8080
`;

const KINDS: Partial<Record<ResourceKind, string>> = {
  deployment: 'Deployment',
  statefulSet: 'StatefulSet',
  pod: 'Pod',
  service: 'Service',
  node: 'Node',
};

export function clusterHandlers(): Record<string, Handler> {
  const on = clusterDemo();
  const nomad = nomadDemo();
  return {
    clusters_list: () => (on ? k.clusters : nomad ? [n.nomadCluster] : []),
    clusters_nomad_namespaces: () => n.nomadNamespaces,
    clusters_nomad_jobs: () => n.nomadJobs,
    clusters_nomad_job: () => n.shopApi,
    clusters_nomad_job_source: () => n.shopApiSource,
    clusters_nomad_job_plan: () => n.shopApiPlan,
    clusters_nomad_nodes: () => n.nomadNodes,
    clusters_nomad_watch_subscribe: () => 1,
    clusters_detect_local: () => (on ? k.localClusters : { found: [], missing: [] }),
    clusters_kubeconfig_contexts: () => k.kubeContexts,
    clusters_namespaces: () => k.namespaces,
    clusters_watch_subscribe: () => 1,
    clusters_metrics_subscribe: () => 1,
    clusters_summary: (a) => (a.id === k.KIND ? k.kindSummary : k.summary),
    clusters_workloads_list: (a) => inNamespace(k.workloads[a.kind as WorkloadKind], a.namespace),
    clusters_workload_get: (a) => k.workloadDetail(a.kind as WorkloadKind, a.name as string),
    clusters_cron_job_jobs: () => k.workloads.job,
    clusters_pods_list: (a) => {
      const selector = typeof a.selector === 'string' ? a.selector.replace(/^app=/, '') : null;
      return inNamespace(k.pods, a.namespace).filter(
        (p) => (!selector || p.name.startsWith(`${selector}-`)) && (!a.node || p.node === a.node),
      );
    },
    clusters_pod_get: (a) => k.podDetail(a.name as string),
    clusters_object_events: (a) =>
      k.events.filter(
        (e) => e.object.name === a.name || e.object.name.startsWith(`${String(a.name)}-`),
      ),
    clusters_resource_yaml: (a) =>
      YAML(KINDS[a.kind as ResourceKind] ?? 'Deployment', a.name as string, a.namespace),
    clusters_services_list: (a) => inNamespace(k.services, a.namespace),
    clusters_service_get: (a) => k.serviceDetail(a.name as string),
    clusters_ingresses_list: (a) => inNamespace(k.ingresses, a.namespace),
    clusters_ingress_get: () => k.ingressDetail,
    clusters_config_maps_list: (a) => inNamespace(k.configMaps, a.namespace),
    clusters_secrets_list: (a) => inNamespace(k.secrets, a.namespace),
    clusters_secret_get: (a) => {
      const view = k.secrets.find((s) => s.name === a.name) ?? k.secrets[0]!;
      return { view, labels: { app: 'postgres' }, annotations: {} };
    },
    clusters_secret_reveal: (a) => ({
      value: a.key === 'username' ? 'shop' : 'demo-only-not-a-real-pass',
      binary: false,
    }),
    clusters_nodes_list: () => k.nodes,
    // Pages the website doesn't photograph, kept empty so they render.
    clusters_helm_releases: () => ({ releases: [], onlyNamespace: null }),
    clusters_forward_list: () => [],
    clusters_api_resources: () => [],
    clusters_rbac_index: (a) => ({
      namespace: a.namespace ?? null,
      roles: [],
      bindings: [],
      subjects: [],
      refused: [],
    }),
    clusters_rbac_own: (a) => ({
      namespace: a.namespace ?? 'default',
      user: null,
      grants: { resources: [], urls: [] },
      incomplete: false,
      evaluationError: null,
    }),
    clusters_node_get: (a) => k.nodeDetail(a.name as string),
    clusters_events_list: (a) => inNamespace(k.events, a.namespace),
  };
}

/** The settings of a Cluster capture: the EKS Cluster selected, on its `shop` namespace. */
export const clusterSettings = (): {
  lastSelection?: { kind: 'cluster'; id: string };
  clusterNamespaces?: Record<string, string>;
} =>
  clusterDemo()
    ? {
        lastSelection: { kind: 'cluster' as const, id: k.EKS },
        clusterNamespaces: { [k.EKS]: 'shop' },
      }
    : nomadDemo()
      ? {
          lastSelection: { kind: 'cluster' as const, id: n.NOMAD },
          clusterNamespaces: { [n.NOMAD]: 'shop' },
        }
      : {};
