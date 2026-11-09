/** Query keys of what Homeport reads from a Nomad Cluster, under one root per Cluster. */
export const nomadKeys = {
  cluster: (clusterId: string) => ['nomad', clusterId] as const,
  jobs: (clusterId: string, namespace: string | null) =>
    ['nomad', clusterId, 'jobs', namespace ?? ''] as const,
  job: (clusterId: string, namespace: string, jobId: string) =>
    ['nomad', clusterId, 'job', namespace, jobId] as const,
};
