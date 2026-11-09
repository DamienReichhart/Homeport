import type { NomadGroupCounts, NomadJobView, NomadScaling } from '@/shared/api';

/** How many allocations a job's task groups run against how many they want, summed. */
export function jobCounts(job: NomadJobView): { running: number; desired: number | null } {
  const running = job.groups.reduce((sum, g) => sum + g.running, 0);
  const known = job.groups.every((g) => g.desired !== null);
  const desired = known ? job.groups.reduce((sum, g) => sum + (g.desired ?? 0), 0) : null;
  return { running, desired };
}

/** Whether a task group runs fewer allocations than it wants. */
export function shortOf(group: NomadGroupCounts): boolean {
  return group.desired !== null && group.running < group.desired;
}

/** A count typed in the scale dialog, `null` unless a whole number within the policy's bounds. */
export function parseCount(text: string, scaling: NomadScaling | null): number | null {
  if (!/^\d+$/.test(text.trim())) return null;
  const count = Number(text.trim());
  if (!Number.isSafeInteger(count)) return null;
  if (scaling?.min != null && count < scaling.min) return null;
  if (scaling?.max != null && count > scaling.max) return null;
  return count;
}

/** The path of a job's page. */
export const jobPath = (namespace: string, jobId: string) =>
  `/nomad/jobs/${encodeURIComponent(namespace)}/${encodeURIComponent(jobId)}`;
