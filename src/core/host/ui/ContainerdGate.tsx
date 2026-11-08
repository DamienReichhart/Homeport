import type { ReactNode } from 'react';
import { Spinner } from '@/shared/ui';
import { useContainerdNamespace } from '../api/containerd';
import { HostGate } from './HostGate';

function WithNamespace({
  hostId,
  children,
}: {
  hostId: string;
  children: (hostId: string, namespace: string) => ReactNode;
}) {
  const [namespace] = useContainerdNamespace(hostId);
  if (namespace === null) return <Spinner />;
  return <>{children(hostId, namespace)}</>;
}

/**
 * A containerd page's gate: the selected Host connected and running
 * containerd, then the namespace its pages show, once known.
 */
export function ContainerdGate({
  children,
}: {
  children: (hostId: string, namespace: string) => ReactNode;
}) {
  return (
    <HostGate requires="containerd">
      {(hostId) => <WithNamespace hostId={hostId}>{children}</WithNamespace>}
    </HostGate>
  );
}
