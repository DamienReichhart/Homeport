import { formatBytes, formatPercent, formatRelative, shortId } from './format';

describe('format', () => {
  it('formats sizes in binary units', () => {
    expect(formatBytes(0)).toBe('0 B');
    expect(formatBytes(512)).toBe('512 B');
    expect(formatBytes(1536)).toBe('1.5 KB');
    expect(formatBytes(3 * 1024 ** 3)).toBe('3.0 GB');
  });

  it('formats percentages', () => {
    expect(formatPercent(3.456)).toBe('3.5%');
    expect(formatPercent(250)).toBe('250%');
  });

  it('describes times relative to now', () => {
    const now = 1_700_000_000_000;
    expect(formatRelative(now / 1000 - 90, now)).toMatch(/2 minutes ago/);
    expect(formatRelative(now / 1000 - 3 * 86400, now)).toMatch(/3 days ago/);
  });

  it('shortens ids', () => {
    expect(shortId('sha256:0123456789abcdef0123')).toBe('0123456789ab');
  });
});
