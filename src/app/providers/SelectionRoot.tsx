import type { ReactNode } from 'react';
import { useHosts } from '@/core/host';
import { SelectionProvider, type Candidate } from '@/core/selection';

/** The selection, falling back over the Hosts the app knows. */
export function SelectionRoot({ children }: { children: ReactNode }) {
  const hosts = useHosts();
  const hostCandidates: Candidate[] | undefined = hosts.data?.map((h) => ({
    id: h.host.id,
    usable: h.status.state === 'connected',
  }));
  return (
    <SelectionProvider hosts={hostCandidates} clusters={[]}>
      {children}
    </SelectionProvider>
  );
}
