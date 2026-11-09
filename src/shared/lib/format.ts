// Numbers, sizes, dates and times in the app's language (ADR 0011): every
// helper reads the active locale, so nothing threads one through components.
// The only place `toLocale*` and `Intl` formatters are used (ESLint enforces it).
import { formattingLocale, t, type MessageKey } from '@/shared/i18n';

const UNITS: MessageKey[] = [
  'format.bytes.b',
  'format.bytes.kb',
  'format.bytes.mb',
  'format.bytes.gb',
  'format.bytes.tb',
];

/** One formatter per locale and options, since building one is slow. */
const formatters = new Map<string, Intl.NumberFormat | Intl.DateTimeFormat>();
function cached<T extends Intl.NumberFormat | Intl.DateTimeFormat>(
  kind: string,
  make: (locale: string) => T,
): T {
  const locale = formattingLocale();
  const id = `${kind}|${locale}`;
  let formatter = formatters.get(id) as T | undefined;
  if (!formatter) {
    formatter = make(locale);
    formatters.set(id, formatter);
  }
  return formatter;
}

const decimals = (digits: number) =>
  cached(
    `n${digits}`,
    (l) =>
      new Intl.NumberFormat(l, {
        minimumFractionDigits: digits,
        maximumFractionDigits: digits,
        // A size is under 1024 of its unit: `1000 B`, never `1,000 B`.
        useGrouping: false,
      }),
  );

/** `1536` → `1.5 KB` (`1,5 Ko` in French). Uses powers of 1024, like the Docker CLI. */
export function formatBytes(bytes: number, digits = 1): string {
  if (!Number.isFinite(bytes) || bytes <= 0)
    return t('format.bytes.b', { value: decimals(0).format(0) });
  const exp = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), UNITS.length - 1);
  const value = decimals(exp === 0 ? 0 : digits).format(bytes / 1024 ** exp);
  return t(UNITS[exp] ?? 'format.bytes.b', { value });
}

/** `3.456` → `3.5%` (`3,5 %` in French); no decimals from 100 up. */
export function formatPercent(value: number): string {
  const digits = value >= 100 ? 0 : 1;
  return cached(
    `p${digits}`,
    (l) =>
      new Intl.NumberFormat(l, {
        style: 'percent',
        minimumFractionDigits: digits,
        maximumFractionDigits: digits,
      }),
  ).format(value / 100);
}

/** A count or other plain number with the language's grouping: `12,345`, `12 345`. */
export function formatNumber(value: number): string {
  return cached('n', (l) => new Intl.NumberFormat(l)).format(value);
}

/** A Unix time in seconds as "3 minutes ago". */
export function formatRelative(unixSeconds: number, now = Date.now()): string {
  const seconds = Math.round(now / 1000 - unixSeconds);
  const rtf = new Intl.RelativeTimeFormat(formattingLocale(), { numeric: 'auto' });
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

type When = Date | number | string;
/** A Unix time in seconds, an ISO string or a Date. */
const toDate = (when: When) =>
  when instanceof Date ? when : new Date(typeof when === 'number' ? when * 1000 : when);

/** Date and time: `9 Nov 2026, 14:05:09`, `9 nov. 2026, 14:05:09`. */
export function formatDateTime(when: When): string {
  return cached(
    'dt',
    (l) => new Intl.DateTimeFormat(l, { dateStyle: 'medium', timeStyle: 'medium' }),
  ).format(toDate(when));
}

/** The date alone: `9 Nov 2026`. */
export function formatDate(when: When): string {
  return cached('d', (l) => new Intl.DateTimeFormat(l, { dateStyle: 'medium' })).format(
    toDate(when),
  );
}

/** The time alone: `14:05:09`. */
export function formatTime(when: When): string {
  return cached('t', (l) => new Intl.DateTimeFormat(l, { timeStyle: 'medium' })).format(
    toDate(when),
  );
}

/** First 12 hex characters of an id, without any `sha256:` prefix. */
export function shortId(id: string): string {
  return id.replace(/^sha256:/, '').slice(0, 12);
}
