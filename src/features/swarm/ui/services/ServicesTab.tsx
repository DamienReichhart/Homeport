import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import type { SwarmView } from '@/shared/api';
import { DataTable, EmptyState, ErrorMessage, SearchInput, Spinner } from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import { serviceColumns } from './serviceColumns';
import { useServiceActions } from './useServiceActions';
import { t } from '@/shared/i18n';

/** Every service of a Swarm, with its replicas; a row opens the service. */
export function ServicesTab({ swarm }: { swarm: SwarmView }) {
  const services = useQuery(swarmFeatureQueries.services(swarm.id));
  const navigate = useNavigate();
  const [dialogs, actions] = useServiceActions(swarm.id);
  const [query, setQuery] = useState('');

  if (services.isPending) return <Spinner />;
  if (services.error)
    return <ErrorMessage error={services.error} onRetry={() => void services.refetch()} />;
  return (
    <div className="grid gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={t('swarm.services.search')} />
      {services.data.length === 0 ? (
        <EmptyState title={t('swarm.services.empty')} />
      ) : (
        <DataTable
          columns={serviceColumns(actions)}
          data={services.data}
          getRowId={(s) => s.id}
          filter={query}
          onRowClick={(s) => void navigate(`/swarms/${swarm.id}/services/${s.id}`)}
        />
      )}
      {dialogs}
    </div>
  );
}
