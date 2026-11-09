import { describe, expect, test } from 'bun:test';
import { formatDate, formatSize } from './format';

describe('formatSize', () => {
  test('gives one decimal under 10 MB and whole megabytes above', () => {
    expect(formatSize(5.25 * 1024 * 1024, 'en-GB')).toBe('5.3 MB');
    expect(formatSize(12.6 * 1024 * 1024, 'en-GB')).toBe('13 MB');
  });

  test('writes the number and unit the way the language does', () => {
    expect(formatSize(5.25 * 1024 * 1024, 'fr')).toMatch(/^5,3\s+Mo$/);
  });
});

describe('formatDate', () => {
  test('writes a long British date in UTC', () => {
    expect(formatDate('2026-11-03T23:30:00Z', 'en-GB')).toBe('3 November 2026');
  });

  test('writes the date in the language asked for', () => {
    expect(formatDate('2026-11-04T10:00:00Z', 'fr')).toBe('4 novembre 2026');
  });

  test('gives nothing for an unknown date', () => {
    expect(formatDate('', 'en-GB')).toBe('');
  });
});
