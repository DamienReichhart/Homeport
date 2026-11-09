/** Query keys of what Homeport reads from a Nomad Cluster, under one root per Cluster. */
export const nomadKeys = {
  cluster: (clusterId: string) => ['nomad', clusterId] as const,
  jobs: (clusterId: string, namespace: string | null) =>
    ['nomad', clusterId, 'jobs', namespace ?? ''] as const,
  job: (clusterId: string, namespace: string, jobId: string) =>
    ['nomad', clusterId, 'job', namespace, jobId] as const,
  allocation: (clusterId: string, allocId: string) =>
    ['nomad', clusterId, 'allocation', allocId] as const,
  nodes: (clusterId: string) => ['nomad', clusterId, 'nodes'] as const,
  namespaces: (clusterId: string) => ['nomad', clusterId, 'namespaces'] as const,
  node: (clusterId: string, nodeId: string) => ['nomad', clusterId, 'node', nodeId] as const,
};
