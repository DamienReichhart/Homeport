import type { IncusProject } from '@/shared/api';

/** The project every Incus has. */
export const DEFAULT_PROJECT = 'default';

/**
 * The project an Incus Host's pages show until one is picked: `default`
 * when the connection may use it, else the first it lists (the restricted
 * socket's own project).
 */
export function preferredProject(projects: IncusProject[]): string {
  if (projects.length === 0 || projects.some((p) => p.name === DEFAULT_PROJECT))
    return DEFAULT_PROJECT;
  return projects[0]!.name;
}

/**
 * The project remembered for `hostId` if the Host still has it, else the
 * preferred one of `projects`; `null` while neither is known.
 */
export function chosenIncusProject(
  hostId: string,
  remembered: Partial<Record<string, string>> | undefined,
  projects: IncusProject[] | undefined,
): string | null {
  const value = remembered?.[hostId];
  if (value && (!projects || projects.length === 0 || projects.some((p) => p.name === value)))
    return value;
  return projects ? preferredProject(projects) : null;
}
