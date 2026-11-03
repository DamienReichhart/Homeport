import type { StatsSample } from '@/shared/api';

/** Five minutes of samples at about one per second. */
export const STATS_WINDOW = 300;

/** A stats sample and when it arrived, in seconds. */
export interface StatsPoint {
  time: number;
  sample: StatsSample;
}

/** Turns cumulative byte counters into per-second rates between samples. */
export function rates(points: StatsPoint[], pick: (s: StatsSample) => number): (number | null)[] {
  return points.map((p, i) => {
    const prev = points[i - 1];
    if (!prev) return null;
    const dt = p.time - prev.time;
    const delta = pick(p.sample) - pick(prev.sample);
    return dt > 0 && delta >= 0 ? delta / dt : null;
  });
}
