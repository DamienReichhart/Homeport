import { useQuery } from '@tanstack/react-query';
import { NetworkIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import type { SwarmView } from '@/shared/api';
import { EmptyState, ErrorMessage, Spinner } from '@/shared/ui';
import { swarmQueries, useSwarms } from '../api/queries';
import { t } from '@/shared/i18n';

/**
 * Renders `children` with the Swarm `swarmId` once one of its managers
 * answers, and explains why otherwise (no manager connected, Swarm gone).
 */
export function SwarmGate({
  swarmId,
  children,
}: {
  swarmId: string;
  children: (swarm: SwarmView) => ReactNode;
}) {
  const swarms = useSwarms();
  const summary = useQuery(swarmQueries.summary(swarmId));
  const swarm = swarms.data?.find((s) => s.id === swarmId);

  if (swarms.isPending) return <Spinner />;
  if (!swarm)
    return (
      <EmptyState
        icon={NetworkIcon}
        title={t('core.swarm.gone.title')}
        description={t('core.swarm.gone.description')}
      />
    );
  if (summary.error)
    return <ErrorMessage error={summary.error} onRetry={() => void summary.refetch()} />;
  return <>{children(swarm)}</>;
}
