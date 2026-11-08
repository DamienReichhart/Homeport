import type { ResourceKind } from '@/shared/api';

const WORKLOADS: Partial<Record<ResourceKind, string>> = {
  deployment: 'deployments',
  statefulSet: 'statefulsets',
  daemonSet: 'daemonsets',
  job: 'jobs',
  cronJob: 'cronjobs',
};

const NAMESPACED: Partial<Record<ResourceKind, string>> = {
  pod: 'pods',
  service: 'services',
  ingress: 'ingresses',
  configMap: 'configmaps',
  secret: 'secrets',
};

/** Kubernetes' kind names, as Events give them. */
const KINDS: Record<string, ResourceKind> = {
  Namespace: 'namespace',
  Deployment: 'deployment',
  StatefulSet: 'statefulSet',
  DaemonSet: 'daemonSet',
  Job: 'job',
  CronJob: 'cronJob',
  Pod: 'pod',
  Service: 'service',
  Ingress: 'ingress',
  ConfigMap: 'configMap',
  Secret: 'secret',
  Node: 'node',
};

const part = encodeURIComponent;

/** The page of a Cluster object, `null` for kinds Homeport has no page for. */
export function objectPath(
  kind: ResourceKind,
  namespace: string | null,
  name: string,
): string | null {
  if (kind === 'node') return `/nodes/${part(name)}`;
  const workload = WORKLOADS[kind];
  if (workload && namespace) return `/workloads/${workload}/${part(namespace)}/${part(name)}`;
  const page = NAMESPACED[kind];
  if (page && namespace) return `/${page}/${part(namespace)}/${part(name)}`;
  return null;
}

/** The page of an object named by its Kubernetes kind (`Deployment`), as in an Event. */
export function kindPath(kind: string, namespace: string | null, name: string): string | null {
  const known = KINDS[kind];
  return known ? objectPath(known, namespace, name) : null;
}
