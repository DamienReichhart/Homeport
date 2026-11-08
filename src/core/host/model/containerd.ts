import type { ContainerdNamespace } from '@/shared/api';

/** The namespace nerdctl and `ctr` use when none is named. */
export const DEFAULT_NAMESPACE = 'default';

const holds = (n: ContainerdNamespace) => n.containers > 0 || n.images > 0;

/**
 * The namespace a containerd Host's pages show until one is picked:
 * `default`, unless it holds nothing and exactly one other namespace does.
 */
export function preferredNamespace(namespaces: ContainerdNamespace[]): string {
  const holding = namespaces.filter(holds);
  const defaultHolds = holding.some((n) => n.name === DEFAULT_NAMESPACE);
  if (!defaultHolds && holding.length === 1) return holding[0]!.name;
  return DEFAULT_NAMESPACE;
}

/**
 * The namespace remembered for `hostId`, else the preferred one of
 * `namespaces`; `null` while neither is known.
 */
export function chosenContainerdNamespace(
  hostId: string,
  remembered: Partial<Record<string, string>> | undefined,
  namespaces: ContainerdNamespace[] | undefined,
): string | null {
  const value = remembered?.[hostId];
  if (value) return value;
  return namespaces ? preferredNamespace(namespaces) : null;
}
