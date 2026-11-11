import type { AlignedData } from 'uplot';
import {
  toAppError,
  type AppError,
  type ContainerHistory,
  type MetricsEvent,
  type NodeUsage,
  type PodUsage,
  type ResourceBounds,
  type ResourceUsage,
  type UsageSample,
} from '@/shared/api';
import { formatBytes, formatNumber } from '@/shared/lib';

/** What a page knows of a Cluster's metrics so far (Pods' or Nodes' CPU and memory). */
export interface MetricsState {
  /** None will come: metrics-server isn't installed, or the credentials may not read them. */
  unavailable: { reason: 'notInstalled' } | { reason: 'forbidden'; error: AppError } | null;
  /** The last read failed, and the next one comes in 15 s. */
  problem: { notResponding: boolean; error: AppError } | null;
  /** Every listed Pod's latest sample, by `namespace/name`. */
  pods: Map<string, PodUsage> | null;
  /** Every Node's latest sample, by name. */
  nodes: Map<string, NodeUsage> | null;
  /** A Pod's containers' samples since its page opened. */
  containers: ContainerHistory[] | null;
  /** A Node's samples since its page opened. */
  samples: UsageSample[] | null;
}

export const NO_METRICS: MetricsState = {
  unavailable: null,
  problem: null,
  pods: null,
  nodes: null,
  containers: null,
  samples: null,
};

/** A Pod's key in `MetricsState.pods`. */
export const podKey = (pod: { namespace: string; name: string }) => `${pod.namespace}/${pod.name}`;

/** The state after `event`. */
export function nextMetrics(state: MetricsState, event: MetricsEvent): MetricsState {
  switch (event.type) {
    case 'notInstalled':
      return { ...NO_METRICS, unavailable: { reason: 'notInstalled' } };
    case 'forbidden':
      return {
        ...NO_METRICS,
        unavailable: { reason: 'forbidden', error: toAppError(event.error) },
      };
    case 'notResponding':
      return { ...state, problem: { notResponding: true, error: toAppError(event.error) } };
    case 'failed':
      return { ...state, problem: { notResponding: false, error: toAppError(event.error) } };
    case 'pods':
      return { ...state, problem: null, pods: new Map(event.pods.map((p) => [podKey(p), p])) };
    case 'nodes':
      return { ...state, problem: null, nodes: new Map(event.nodes.map((n) => [n.name, n])) };
    case 'pod':
      return { ...state, problem: null, containers: event.containers };
    case 'node':
      return { ...state, problem: null, samples: event.samples };
  }
}

/** What use is set against: the allocatable of a Node, or a Pod's limit, else its request. */
export type Measure = 'limit' | 'request' | 'allocatable';

/** What a Pod's or container's use is measured against: its limit, else its request. */
export function measureOf(bounds: ResourceBounds): { kind: Measure; value: number } | null {
  if (bounds.limit) return { kind: 'limit', value: bounds.limit };
  if (bounds.request) return { kind: 'request', value: bounds.request };
  return null;
}

/** `used` as a share of `of`, `0.5` for half; `null` without anything to measure against. */
export function share(used: number, of: number | null | undefined): number | null {
  return of ? used / of : null;
}

/** CPU in millicores, as `kubectl top` writes it: `250m`, `1,250m`, `0.4m`. */
export function formatCpu(millicores: number): string {
  const value = millicores < 10 ? Math.round(millicores * 10) / 10 : Math.round(millicores);
  return `${formatNumber(value)}m`;
}

/** The format of a resource's figures: millicores for CPU, sizes for memory. */
export const usageFormat = (resource: keyof ResourceUsage): ((value: number) => string) =>
  resource === 'cpu' ? formatCpu : (value: number) => formatBytes(value);

/**
 * A chart's data: the samples' times, what they used of `resource`, then
 * one flat line per value of `lines` (a request, a limit, the allocatable).
 */
export function chartData(
  samples: UsageSample[],
  resource: keyof ResourceUsage,
  lines: number[],
): AlignedData {
  return [
    samples.map((s) => s.time),
    samples.map((s) => s.usage[resource] ?? 0),
    ...lines.map((value) => samples.map(() => value)),
  ];
}
