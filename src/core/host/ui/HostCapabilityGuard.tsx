import type { ReactNode } from 'react';
import type { HostCapability } from '@/core/feature';
import { capabilityKnown, supports } from '../model/engine';
import { useHostSelection } from '../selection/context';
import { UnsupportedEngine } from './UnsupportedEngine';

/**
 * Shows the pages of a feature that `requires` something of the Host's
 * Engine only when the selected Host offers it, and says why otherwise, so
 * a link or a remembered page never opens one it can't show.
 */
export function HostCapabilityGuard({
  requires,
  children,
}: {
  requires: HostCapability;
  children: ReactNode;
}) {
  const { host } = useHostSelection();
  if (host && capabilityKnown(host, requires) && !supports(host, requires)) {
    return <UnsupportedEngine host={host} requires={requires} />;
  }
  return <>{children}</>;
}
