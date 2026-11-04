import type { HealthConfig } from '@/shared/api';
import { formatDuration } from './parse';
import { t } from '@/shared/i18n';

/** The Engine's defaults for a healthcheck setting left unset. */
const DEFAULTS = { intervalS: 30, timeoutS: 30, retries: 3, startPeriodS: 0 };

/** The healthcheck's timing, as `every 30s, timeout 30s, 3 retries, start period 0s`. */
export function healthTiming(config: HealthConfig): string {
  const interval = config.intervalS ?? DEFAULTS.intervalS;
  const timeout = config.timeoutS ?? DEFAULTS.timeoutS;
  const retries = config.retries ?? DEFAULTS.retries;
  const start = config.startPeriodS ?? DEFAULTS.startPeriodS;
  return t('containers.health.timing', {
    interval: formatDuration(interval),
    timeout: formatDuration(timeout),
    retries,
    start: formatDuration(start),
  });
}

/** How long a check took, from its RFC 3339 start and end. */
export function checkDuration(start: string, end: string): string | null {
  const ms = Date.parse(end) - Date.parse(start);
  if (!Number.isFinite(ms) || ms < 0) return null;
  return ms < 1000 ? `${ms} ms` : `${(ms / 1000).toFixed(1)} s`;
}

/** Outputs longer than this many lines are folded. */
export const OUTPUT_LINES = 6;
