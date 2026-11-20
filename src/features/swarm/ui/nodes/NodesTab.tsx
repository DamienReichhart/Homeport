import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useLocation } from 'react-router-dom';
import { useHosts } from '@/core/host';
import type { NodeChange, SwarmNode, SwarmView } from '@/shared/api';
import { DataTable, ErrorMessage, SearchInput, Spinner, useConfirmedAction } from '@/shared/ui';
import { useUpdateNode } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import { LabelsDialog } from './LabelsDialog';
import { nodeColumns } from './nodeColumns';
import { RemoveNodeDialog } from './RemoveNodeDialog';
import { t } from '@/shared/i18n';

/**
 * Every node of a Swarm, with their availability, role and labels; a row
 * shows the node's tasks. Draining and demoting say what they do first,
 * since both move work around the Swarm.
 */
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
  const [confirmDialog, confirm] = useConfirmedAction();

  const applyChange = (node: SwarmNode, change: NodeChange) => {
    if (change.availability === 'drain')
      return confirm({
        title: t('swarm.nodes.drain.title', { name: node.hostname }),
        description: t('swarm.nodes.drain.description'),
        confirmLabel: t('swarm.nodes.action.drain'),
        action: () => update.mutateAsync({ node, change }),
      });
    if (change.role === 'worker')
      return confirm({
        title: t('swarm.nodes.demote.title', { name: node.hostname }),
        description: t('swarm.nodes.demote.description'),
        confirmLabel: t('swarm.nodes.action.demote'),
        action: () => update.mutateAsync({ node, change }),
      });
    update.mutate({ node, change });
  };

  if (nodes.isPending) return <Spinner />;
  if (nodes.error) return <ErrorMessage error={nodes.error} onRetry={() => void nodes.refetch()} />;
  return (
    <div className="grid gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={t('swarm.nodes.search')} />
      <DataTable
        columns={nodeColumns({
          hosts: hosts.data,
          onChange: applyChange,
          onEditLabels: setLabels,
          onShowTasks,
          onRemove: setRemoving,
        })}
        data={nodes.data}
        getRowId={(n) => n.id}
        filter={query}
        selectedId={marked}
        onRowClick={onShowTasks}
      />
      {confirmDialog}
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
