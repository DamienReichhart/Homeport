import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useLocation } from 'react-router-dom';
import { useHosts } from '@/core/host';
import type { SwarmNode, SwarmView } from '@/shared/api';
import { DataTable, ErrorMessage, SearchInput, Spinner } from '@/shared/ui';
import { useUpdateNode } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import { LabelsDialog } from './LabelsDialog';
import { nodeColumns } from './nodeColumns';
import { RemoveNodeDialog } from './RemoveNodeDialog';
import { t } from '@/shared/i18n';

/** Every node of a Swarm, with their availability, role and labels. */
export function NodesTab({
  swarm,
  onShowTasks,
}: {
  swarm: SwarmView;
  onShowTasks?: (node: SwarmNode) => void;
}) {
  const nodes = useQuery(swarmFeatureQueries.nodes(swarm.id));
  const hosts = useHosts();
  const update = useUpdateNode(swarm.id);
  const location = useLocation();
  const marked = new URLSearchParams(location.search).get('node');
  const [query, setQuery] = useState('');
  const [labels, setLabels] = useState<SwarmNode | null>(null);
  const [removing, setRemoving] = useState<SwarmNode | null>(null);

  if (nodes.isPending) return <Spinner />;
  if (nodes.error) return <ErrorMessage error={nodes.error} onRetry={() => void nodes.refetch()} />;
  return (
    <div className="grid gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={t('swarm.nodes.search')} />
      <DataTable
        columns={nodeColumns({
          hosts: hosts.data,
          onChange: (node, change) => update.mutate({ node, change }),
          onEditLabels: setLabels,
          onShowTasks,
          onRemove: setRemoving,
        })}
        data={nodes.data}
        getRowId={(n) => n.id}
        filter={query}
        selectedId={marked}
      />
      {labels && (
        <LabelsDialog
          node={labels}
          onSave={(map) =>
            update.mutateAsync({
              node: labels,
              change: { availability: null, role: null, labels: map },
            })
          }
          onClose={() => setLabels(null)}
        />
      )}
      {removing && (
        <RemoveNodeDialog swarmId={swarm.id} node={removing} onClose={() => setRemoving(null)} />
      )}
    </div>
  );
}
