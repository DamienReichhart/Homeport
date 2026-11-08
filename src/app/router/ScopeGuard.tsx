import type { ReactNode } from 'react';
import { Navigate } from 'react-router-dom';
import type { NavScope } from '@/core/feature';
import { useSelection } from '@/core/selection';

/**
 * Shows a Host page only while a Host is selected, and a Cluster page only
 * while a Cluster is: opening one with the other kind selected (from a link
 * or after switching) goes to the Overview instead of an empty gate.
 */
export function ScopeGuard({ scope, children }: { scope: NavScope; children: ReactNode }) {
  const { selection } = useSelection();
  if (scope !== 'app' && selection && selection.kind !== scope) return <Navigate to="/" replace />;
  return <>{children}</>;
}
