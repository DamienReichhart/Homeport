import { setLocale } from '@/shared/i18n';
import {
  formatBytes,
  formatDate,
  formatDateTime,
  formatNumber,
  formatPercent,
  formatRelative,
  shortId,
} from './format';

const now = 1_700_000_000_000;

describe('format, in English', () => {
  it('formats sizes in binary units', () => {
    expect(formatBytes(0)).toBe('0 B');
    expect(formatBytes(512)).toBe('512 B');
    expect(formatBytes(1536)).toBe('1.5 KB');
    expect(formatBytes(3 * 1024 ** 3)).toBe('3.0 GB');
  });

  it('formats percentages and numbers', () => {
    expect(formatPercent(3.456)).toBe('3.5%');
    expect(formatPercent(250)).toBe('250%');
    expect(formatNumber(12345)).toBe('12,345');
  });

  it('describes times relative to now', () => {
    expect(formatRelative(now / 1000 - 1, now)).toBe('1 second ago');
    expect(formatRelative(now / 1000 - 90, now)).toBe('2 minutes ago');
    expect(formatRelative(now / 1000 - 3 * 86400, now)).toBe('3 days ago');
  });

  it('formats dates from seconds, ISO strings and Dates alike', () => {
    const iso = '2026-11-04T14:05:09Z';
    const seconds = Date.parse(iso) / 1000;
    expect(formatDateTime(seconds)).toBe(formatDateTime(iso));
    expect(formatDateTime(new Date(iso))).toBe(formatDateTime(iso));
    expect(formatDate(iso)).toMatch(/2026/);
  });

  it('shortens ids', () => {
    expect(shortId('sha256:0123456789abcdef0123')).toBe('0123456789ab');
  });
});

describe('format, in French', () => {
  beforeAll(() => setLocale('fr'));
  afterAll(() => setLocale('en'));

  it('uses the decimal comma, French units and spacing', () => {
    expect(formatBytes(1536)).toBe('1,5 Ko');
    expect(formatBytes(3 * 1024 ** 3)).toBe('3,0 Go');
    expect(formatPercent(3.456)).toBe('3,5 %');
    expect(formatNumber(12345)).toBe('12 345');
  });

  it('says relative times and dates in French, singular and plural', () => {
    expect(formatRelative(now / 1000 - 1, now)).toBe('il y a 1 seconde');
    expect(formatRelative(now / 1000 - 90, now)).toBe('il y a 2 minutes');
    expect(formatDate('2026-11-04T12:00:00Z')).toBe('4 nov. 2026');
  });
});
