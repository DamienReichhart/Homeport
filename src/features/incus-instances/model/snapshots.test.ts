import { describe, expect, it } from 'vitest';
import { expiresAt } from './snapshots';

describe('snapshot expiry', () => {
  it('counts days from now, or keeps it', () => {
    const now = Date.parse('2026-11-08T10:00:00Z');
    expect(expiresAt('never', now)).toBeNull();
    expect(expiresAt('7d', now)).toBe('2026-11-15T10:00:00.000Z');
  });
});
