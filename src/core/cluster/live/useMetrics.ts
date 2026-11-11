import { useEffect, useState } from 'react';
import type { MetricsTarget } from '@/shared/api';
import { streamMetrics } from '../api/metrics';
import { nextMetrics, NO_METRICS, type MetricsState } from '../model/metrics';

/**
 * While mounted, polls the CPU and memory of `target` (none with `null`)
 * every 15 s, and stops when unmounted: a Pod's or a Node's page keeps
 * the samples read since it opened, in memory only. A Cluster without
 * metrics-server says so once, and nothing is polled after.
 */
export function useMetrics(clusterId: string, target: MetricsTarget | null): MetricsState {
  const key = target && JSON.stringify({ clusterId, target });
  const [state, setState] = useState({ key, metrics: NO_METRICS });
  useEffect(() => {
    if (!key) return;
    const update = (next: (metrics: MetricsState) => MetricsState) =>
      setState((s) => ({ key, metrics: next(s.key === key ? s.metrics : NO_METRICS) }));
    return streamMetrics(
      clusterId,
      (JSON.parse(key) as { target: MetricsTarget }).target,
      (event) => update((metrics) => nextMetrics(metrics, event)),
      // The Cluster went away meanwhile: like any failed read, said in place.
      (error) => update((metrics) => ({ ...metrics, problem: { notResponding: false, error } })),
    );
  }, [clusterId, key]);
  // A new target starts from nothing, not from the last one's samples.
  return state.key === key ? state.metrics : NO_METRICS;
}
