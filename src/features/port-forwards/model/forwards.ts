import type { ForwardTarget } from '@/shared/api';

/** A forward's target as people say it: `svc/api` or `pod/api-1`, in its Namespace. */
export function targetLabel(target: ForwardTarget): string {
  return `${target.kind === 'service' ? 'svc' : 'pod'}/${target.name}`;
}

/** The local port typed in the Forward dialog: `null` for any free one, `undefined` while invalid. */
export function parseLocalPort(text: string): number | null | undefined {
  const trimmed = text.trim();
  if (trimmed === '') return null;
  if (!/^\d+$/.test(trimmed)) return undefined;
  const port = Number(trimmed);
  return port >= 1 && port <= 65535 ? port : undefined;
}
