/**
 * Parsing of the Run dialog's sizes, durations and CPU counts, written the
 * way `docker run` takes them. Each parser returns `null` for an empty field
 * and `undefined` for one it can't read.
 */

const SIZE_UNITS: Record<string, number> = {
  '': 1,
  b: 1,
  k: 1024,
  m: 1024 ** 2,
  g: 1024 ** 3,
  t: 1024 ** 4,
};

/** `512m`, `2g`, `1.5GB` or `1024` (bytes) as bytes, powers of 1024 as Docker counts. */
export function parseSize(text: string): number | null | undefined {
  const t = text.trim().toLowerCase();
  if (t === '') return null;
  const match =
    /^(\d+(?:\.\d+)?)\s*([kmgt]?)(?:i?b)?$/.exec(t) ?? /^(\d+(?:\.\d+)?)\s*(b)$/.exec(t);
  if (!match) return undefined;
  const bytes = Math.round(Number(match[1]) * (SIZE_UNITS[match[2] ?? ''] ?? 1));
  return bytes > 0 ? bytes : undefined;
}

/** Bytes as the shortest exact `parseSize` input: `512m`, `2g`, or plain bytes. */
export function formatSize(bytes: number): string {
  for (const unit of ['t', 'g', 'm', 'k'] as const) {
    const size = SIZE_UNITS[unit] ?? 1;
    if (bytes % size === 0) return `${bytes / size}${unit}`;
  }
  return String(bytes);
}

const DURATION_UNITS: Record<string, number> = { h: 3600, m: 60, s: 1 };

/** `30s`, `1m30s`, `2h` or `45` (seconds) as whole seconds above 0. */
export function parseDuration(text: string): number | null | undefined {
  const t = text.trim().toLowerCase();
  if (t === '') return null;
  if (/^\d+$/.test(t)) return Number(t) > 0 ? Number(t) : undefined;
  if (!/^(\d+[hms])+$/.test(t)) return undefined;
  let seconds = 0;
  for (const [, n, unit] of t.matchAll(/(\d+)([hms])/g)) {
    seconds += Number(n) * (DURATION_UNITS[unit ?? 's'] ?? 1);
  }
  return seconds > 0 ? seconds : undefined;
}

/** Seconds as `1m30s`, `2h` or `45s`. */
export function formatDuration(seconds: number): string {
  const h = Math.floor(seconds / 3600);
  const m = Math.floor((seconds % 3600) / 60);
  const s = seconds % 60;
  return `${h ? `${h}h` : ''}${m ? `${m}m` : ''}${s || (!h && !m) ? `${s}s` : ''}`;
}

/** A number above 0, such as a CPU count (`0.5`); `integer` refuses fractions. */
export function parsePositive(text: string, integer = false): number | null | undefined {
  const t = text.trim();
  if (t === '') return null;
  if (!/^\d+(\.\d+)?$/.test(t)) return undefined;
  const n = Number(t);
  if (n <= 0 || (integer && !Number.isInteger(n))) return undefined;
  return n;
}
