const UNITS = ['B', 'KB', 'MB', 'GB', 'TB'];

/** `1536` → `1.5 KB`. Uses powers of 1024, like the Docker CLI. */
export function formatBytes(bytes: number, digits = 1): string {
  if (!Number.isFinite(bytes) || bytes <= 0) return '0 B';
  const exp = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), UNITS.length - 1);
  const value = bytes / 1024 ** exp;
  return `${value.toFixed(exp === 0 ? 0 : digits)} ${UNITS[exp]}`;
}

export function formatPercent(value: number): string {
  return `${value.toFixed(value >= 100 ? 0 : 1)}%`;
}

/** A Unix time in seconds as "3 minutes ago". */
export function formatRelative(unixSeconds: number, now = Date.now()): string {
  const seconds = Math.round(now / 1000 - unixSeconds);
  const rtf = new Intl.RelativeTimeFormat(undefined, { numeric: 'auto' });
  const steps: [number, Intl.RelativeTimeFormatUnit][] = [
    [60, 'second'],
    [60, 'minute'],
    [24, 'hour'],
    [30, 'day'],
    [12, 'month'],
  ];
  let value = seconds;
  for (const [size, unit] of steps) {
    if (Math.abs(value) < size) return rtf.format(-value, unit);
    value = Math.round(value / size);
  }
  return rtf.format(-value, 'year');
}

export function formatDateTime(unixSeconds: number): string {
  return new Date(unixSeconds * 1000).toLocaleString();
}

/** First 12 hex characters of an id, without any `sha256:` prefix. */
export function shortId(id: string): string {
  return id.replace(/^sha256:/, '').slice(0, 12);
}
