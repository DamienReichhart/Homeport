import { ShipWheelIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import { useHelm } from '@/core/cluster';
import { EmptyState, HintView, Spinner } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * Shows `children` when the helm CLI is on this machine, else what's missing
 * and how to install it; the rest of the Cluster's pages work without it.
 */
export function HelmGate({ children }: { children: ReactNode }) {
  const { helm, isPending } = useHelm();
  if (isPending) return <Spinner />;
  if (helm?.state === 'available') return children;
  return (
    <EmptyState
      icon={ShipWheelIcon}
      title={helm?.problem.message ?? t('helm.missing.title')}
      description={
        <div className="grid gap-3">
          <p>{t('helm.missing.description')}</p>
          {helm?.problem.hint && <HintView hint={helm.problem.hint} />}
        </div>
      }
    />
  );
}
