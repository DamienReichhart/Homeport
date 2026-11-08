import type { ReactNode } from 'react';
import { useClusters } from '@/core/cluster';
import { useHosts } from '@/core/host';
import { SelectionProvider, type Candidate } from '@/core/selection';

/** The selection, falling back over the Hosts and Clusters the app knows. */
export function SelectionRoot({ children }: { children: ReactNode }) {
  const hosts = useHosts();
  const clusters = useClusters();
  const hostCandidates: Candidate[] | undefined = hosts.data?.map((h) => ({
    id: h.host.id,
    usable: h.status.state === 'connected',
  }));
  const clusterCandidates: Candidate[] | undefined = clusters.data?.map((c) => ({
    id: c.cluster.id,
    usable: c.status.state === 'connected',
  }));
  return (
    <SelectionProvider hosts={hostCandidates} clusters={clusterCandidates}>
      {children}
    </SelectionProvider>
  );
}
