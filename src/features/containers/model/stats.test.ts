import type { StatsSample } from '@/shared/api';
import { rates } from './stats';

const sample = (netRx: number): StatsSample => ({
  cpuPercent: 0,
  memoryUsage: 0,
  memoryLimit: 0,
  netRx,
  netTx: 0,
  blockRead: 0,
  blockWrite: 0,
  pids: 1,
});

describe('rates', () => {
  it('turns counters into per-second rates and skips resets', () => {
    const points = [
      { time: 10, sample: sample(1000) },
      { time: 12, sample: sample(3000) },
      { time: 13, sample: sample(500) },
    ];
    expect(rates(points, (s) => s.netRx)).toEqual([null, 1000, null]);
  });
});
