import type { SwarmTask } from '@/shared/api';

/** "web.2" for a replica, "agent@node-b" for a global service's task. */
export function taskName(task: SwarmTask): string {
  if (task.slot !== null) return `${task.serviceName}.${task.slot}`;
  return `${task.serviceName}@${task.nodeHostname ?? task.nodeId?.slice(0, 12) ?? '?'}`;
}

/** States a task ends in without running. */
const FAILED = new Set(['failed', 'rejected', 'orphaned']);
export const isFailed = (task: SwarmTask) => FAILED.has(task.state);

/**
 * The tasks to show, grouped by service then slot (or node), newest first in
 * each. Without history only the tasks the Swarm wants running are kept: one
 * per slot; with it, also the earlier attempts it shut down or that failed.
 */
export function arrangeTasks(tasks: SwarmTask[], history: boolean): SwarmTask[] {
  const shown = history ? tasks : tasks.filter((t) => t.desiredState === 'running');
  const slotKey = (t: SwarmTask) =>
    t.slot !== null ? String(t.slot).padStart(6, '0') : (t.nodeHostname ?? t.nodeId ?? '');
  return [...shown].sort(
    (a, b) =>
      a.serviceName.localeCompare(b.serviceName) ||
      slotKey(a).localeCompare(slotKey(b)) ||
      (b.since ?? '').localeCompare(a.since ?? ''),
  );
}
