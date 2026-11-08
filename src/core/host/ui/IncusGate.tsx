import type { ReactNode } from 'react';
import { Spinner } from '@/shared/ui';
import { useIncusProject } from '../api/incus';
import { HostGate } from './HostGate';

function WithProject({
  hostId,
  children,
}: {
  hostId: string;
  children: (hostId: string, project: string) => ReactNode;
}) {
  const [project] = useIncusProject(hostId);
  if (project === null) return <Spinner />;
  return <>{children(hostId, project)}</>;
}

/**
 * An Incus page's gate: the selected Host connected and running Incus,
 * then the project its pages show, once known.
 */
export function IncusGate({
  children,
}: {
  children: (hostId: string, project: string) => ReactNode;
}) {
  return (
    <HostGate requires="incus">
      {(hostId) => <WithProject hostId={hostId}>{children}</WithProject>}
    </HostGate>
  );
}
