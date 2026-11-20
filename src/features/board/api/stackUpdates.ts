import { useQueries } from '@tanstack/react-query';
import { containerFreshness, indexFreshness, resourceQueries } from '@/core/host';
import type { BoardRow } from '../model/board';
import { stackUpdates, type StackUpdate } from '../model/needsYou';

/**
 * The Stacks whose running images have a newer version published, on every
 * connected Host, from the freshness the Containers page already checks.
 */
export function useStackUpdates(rows: BoardRow[]): StackUpdate[] {
  const hostIds = [...new Set(rows.map((r) => r.hostId))];
  const freshness = useQueries({
    queries: hostIds.map((id) => resourceQueries.freshness(id)),
  });
  const indexes = new Map(hostIds.map((id, i) => [id, indexFreshness(freshness[i]?.data)]));
  return stackUpdates(
    rows.filter(
      (r) =>
        r.container.stack !== null &&
        containerFreshness(indexes.get(r.hostId) ?? new Map(), r.container) === 'newer',
    ),
  );
}
