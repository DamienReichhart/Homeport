import type { AppError, PodUsage } from '@/shared/api';
import { chartData, formatCpu, measureOf, nextMetrics, NO_METRICS, podKey, share } from './metrics';

const error = { code: 'forbidden', message: 'no' } as AppError;
const pod = (name: string): PodUsage => ({
  namespace: 'shop',
  name,
  time: 0,
  usage: { cpu: 10, memory: 1 },
  containers: [],
});

describe('metrics', () => {
  it('measures a Pod against its limit, else its request', () => {
    expect(measureOf({ request: 250, limit: 1000 })).toEqual({ kind: 'limit', value: 1000 });
    expect(measureOf({ request: 250, limit: null })).toEqual({ kind: 'request', value: 250 });
    expect(measureOf({ request: null, limit: null })).toBeNull();
    expect(measureOf({ request: 0, limit: null })).toBeNull();
  });

  it('gives use as a share of what it is measured against', () => {
    expect(share(250, 1000)).toBe(0.25);
    expect(share(1500, 1000)).toBe(1.5);
    expect(share(250, null)).toBeNull();
    expect(share(250, 0)).toBeNull();
  });

  it('writes CPU in millicores', () => {
    expect(formatCpu(250)).toBe('250m');
    expect(formatCpu(1250.4)).toBe('1,250m');
    expect(formatCpu(0.42)).toBe('0.4m');
  });

  it('keeps the latest samples and forgets a passing failure once one comes', () => {
    let state = nextMetrics(NO_METRICS, { type: 'notResponding', error });
    expect(state.problem?.notResponding).toBe(true);
    expect(state.problem?.error.message).toBe('no');
    state = nextMetrics(state, { type: 'pods', pods: [pod('web-1'), pod('web-2')] });
    expect(state.problem).toBeNull();
    expect(state.pods?.get(podKey(pod('web-2')))?.name).toBe('web-2');
    state = nextMetrics(state, { type: 'failed', error });
    expect(state.pods?.size).toBe(2);
  });

  it('says why none will come, dropping what came before', () => {
    const listed = nextMetrics(NO_METRICS, { type: 'pods', pods: [pod('web-1')] });
    expect(nextMetrics(listed, { type: 'notInstalled' })).toEqual({
      ...NO_METRICS,
      unavailable: { reason: 'notInstalled' },
    });
    const forbidden = nextMetrics(listed, { type: 'forbidden', error }).unavailable;
    expect(forbidden?.reason).toBe('forbidden');
    expect(forbidden?.reason === 'forbidden' && forbidden.error.code).toBe('forbidden');
    expect(nextMetrics(listed, { type: 'forbidden', error }).pods).toBeNull();
  });

  it('charts what was used with a flat line per bound', () => {
    const samples = [
      { time: 10, usage: { cpu: 100, memory: 5 } },
      { time: 25, usage: { cpu: 300, memory: 6 } },
    ];
    expect(chartData(samples, 'cpu', [250, 500])).toEqual([
      [10, 25],
      [100, 300],
      [250, 250],
      [500, 500],
    ]);
    expect(chartData(samples, 'memory', [])).toEqual([
      [10, 25],
      [5, 6],
    ]);
  });
});
