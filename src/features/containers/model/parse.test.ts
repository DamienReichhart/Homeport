import { formatDuration, formatSize, parseDuration, parsePositive, parseSize } from './parse';

describe('run dialog parsing', () => {
  it('reads sizes the way docker run does', () => {
    expect(parseSize('512m')).toBe(512 * 1024 ** 2);
    expect(parseSize('2g')).toBe(2 * 1024 ** 3);
    expect(parseSize(' 1.5GB ')).toBe(1.5 * 1024 ** 3);
    expect(parseSize('64MiB')).toBe(64 * 1024 ** 2);
    expect(parseSize('1024')).toBe(1024);
    expect(parseSize('100b')).toBe(100);
    expect(parseSize('')).toBeNull();
    for (const bad of ['m', '12x', '-1g', '0', '1,5g']) expect(parseSize(bad)).toBeUndefined();
  });

  it('writes sizes back in the largest exact unit', () => {
    expect(formatSize(512 * 1024 ** 2)).toBe('512m');
    expect(formatSize(2 * 1024 ** 3)).toBe('2g');
    expect(formatSize(1500)).toBe('1500');
    expect(parseSize(formatSize(3 * 1024))).toBe(3 * 1024);
  });

  it('reads durations in hours, minutes and seconds', () => {
    expect(parseDuration('30s')).toBe(30);
    expect(parseDuration('1m30s')).toBe(90);
    expect(parseDuration('2h')).toBe(7200);
    expect(parseDuration('45')).toBe(45);
    expect(parseDuration('')).toBeNull();
    for (const bad of ['0s', '0', '1.5s', 's', '10 minutes', '-5s']) {
      expect(parseDuration(bad)).toBeUndefined();
    }
  });

  it('writes durations back', () => {
    expect(formatDuration(90)).toBe('1m30s');
    expect(formatDuration(7200)).toBe('2h');
    expect(formatDuration(5)).toBe('5s');
  });

  it('reads positive numbers', () => {
    expect(parsePositive('0.5')).toBe(0.5);
    expect(parsePositive('3', true)).toBe(3);
    expect(parsePositive('1.5', true)).toBeUndefined();
    expect(parsePositive('0')).toBeUndefined();
    expect(parsePositive('abc')).toBeUndefined();
    expect(parsePositive(' ')).toBeNull();
  });
});
