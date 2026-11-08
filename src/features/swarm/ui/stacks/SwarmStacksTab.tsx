import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { DataTable, EmptyState, ErrorMessage, SearchInput, Spinner } from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import { swarmStackColumns } from './swarmStackColumns';
import { t } from '@/shared/i18n';

/** Every Stack deployed to a Swarm, from a terminal or from Homeport. */
export function SwarmStacksTab({ swarmId }: { swarmId: string }) {
  const stacks = useQuery(swarmFeatureQueries.stacks(swarmId));
  const navigate = useNavigate();
  const [query, setQuery] = useState('');
  if (stacks.isPending) return <Spinner />;
  if (stacks.error)
    return <ErrorMessage error={stacks.error} onRetry={() => void stacks.refetch()} />;
  return (
    <div className="grid gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={t('swarm.stacks.search')} />
      {stacks.data.length === 0 ? (
        <EmptyState title={t('swarm.stacks.empty')} description={t('swarm.stacks.emptyHint')} />
      ) : (
        <DataTable
          columns={swarmStackColumns()}
          data={stacks.data}
          getRowId={(s) => s.name}
          filter={query}
          onRowClick={(s) =>
            void navigate(`/swarms/${swarmId}/stacks/${encodeURIComponent(s.name)}`)
          }
        />
      )}
    </div>
  );
}
