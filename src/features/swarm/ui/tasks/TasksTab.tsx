import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { useHosts, useHostSelection } from '@/core/host';
import type { SwarmTask } from '@/shared/api';
import { Button, DataTable, EmptyState, ErrorMessage, Label, Spinner, Switch } from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import { arrangeTasks } from '../../model/tasks';
import { taskColumns } from './taskColumns';
import { t } from '@/shared/i18n';

/**
 * A Swarm's tasks, or one service's (`serviceId`) or one node's (`?node=`):
 * what runs where, and why what doesn't.
 */
export function TasksTab({ swarmId, serviceId }: { swarmId: string; serviceId?: string }) {
  const location = useLocation();
  const navigate = useNavigate();
  const nodeId = serviceId ? null : new URLSearchParams(location.search).get('node');
  const filter = { serviceId: serviceId ?? null, nodeId };
  const tasks = useQuery(swarmFeatureQueries.tasks(swarmId, filter));
  const hosts = useHosts();
  const { select } = useHostSelection();
  const [history, setHistory] = useState(false);

  const openContainer = (task: SwarmTask) => {
    if (!task.hostId || !task.containerId) return;
    select(task.hostId);
    void navigate(`/containers/${task.containerId}`);
  };

  if (tasks.isPending) return <Spinner />;
  if (tasks.error) return <ErrorMessage error={tasks.error} onRetry={() => void tasks.refetch()} />;
  const node = nodeId ? tasks.data.find((task) => task.nodeId === nodeId) : null;
  const rows = arrangeTasks(tasks.data, history);
  return (
    <div className="grid gap-3">
      <div className="flex flex-wrap items-center gap-4">
        <div className="flex items-center gap-2">
          <Switch id="task-history" checked={history} onCheckedChange={setHistory} />
          <Label htmlFor="task-history">{t('swarm.tasks.history')}</Label>
        </div>
        {nodeId && (
          <span className="flex items-center gap-2 text-sm">
            {t('swarm.tasks.onNode', { name: node?.nodeHostname ?? nodeId.slice(0, 12) })}
            <Button
              variant="link"
              size="sm"
              onClick={() => void navigate(`/swarms/${swarmId}/tasks`, { replace: true })}
            >
              {t('swarm.tasks.allNodes')}
            </Button>
          </span>
        )}
      </div>
      {rows.length === 0 ? (
        <EmptyState title={t('swarm.tasks.empty')} />
      ) : (
        <DataTable
          columns={taskColumns({ swarmId, hosts: hosts.data, onOpenContainer: openContainer })}
          data={rows}
          getRowId={(task) => task.id}
        />
      )}
    </div>
  );
}
