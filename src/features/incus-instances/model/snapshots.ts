/** How long a new snapshot is kept, as the dialog offers it. */
export type Expiry = 'never' | '1d' | '7d' | '30d';

const DAYS: Record<Exclude<Expiry, 'never'>, number> = { '1d': 1, '7d': 7, '30d': 30 };

/** The RFC 3339 expiry of a snapshot taken at `now`; `null` to keep it. */
export function expiresAt(expiry: Expiry, now = Date.now()): string | null {
  if (expiry === 'never') return null;
  return new Date(now + DAYS[expiry] * 86_400_000).toISOString();
}
