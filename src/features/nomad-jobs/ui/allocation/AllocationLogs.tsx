import { useMemo, useState } from 'react';
import { LogViewer } from '@/features/containers';
import { nomadLogs } from '../../api/streams';
import { TaskSelect } from './TaskSelect';

/** A task's stdout and stderr, followed live, one task at a time. */
export function AllocationLogs({
  clusterId,
  allocId,
  tasks,
}: {
  clusterId: string;
  allocId: string;
  tasks: string[];
}) {
  const [task, setTask] = useState(tasks[0] ?? '');
  const stream = useMemo(() => nomadLogs(clusterId, allocId, task), [clusterId, allocId, task]);
  return (
    <div className="h-[60vh]">
      <LogViewer
        stream={stream}
        streamKey={`${clusterId}/${allocId}/${task}`}
        toolbar={tasks.length > 1 && <TaskSelect tasks={tasks} value={task} onChange={setTask} />}
      />
    </div>
  );
}
