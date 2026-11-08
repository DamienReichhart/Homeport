import type { ServiceView } from '@/shared/api';

/** Whether fewer tasks run than should: the service isn't converging. */
export const underReplicated = (s: ServiceView) => s.running < s.desired;

/** Only replicated services have a number of replicas to set. */
export const canScale = (s: ServiceView) => s.mode === 'replicated';

/** A number of replicas as typed: a whole number from 0 to 10000, else `null`. */
export function parseReplicas(text: string): number | null {
  if (!/^\d+$/.test(text.trim())) return null;
  const n = Number(text.trim());
  return n <= 10_000 ? n : null;
}

/** "nginx:1.27", "8080→80/tcp"… for the ports column. */
export function portsLabel(s: ServiceView): string {
  return s.ports
    .map((p) =>
      p.published ? `${p.published}→${p.target}/${p.protocol}` : `${p.target}/${p.protocol}`,
    )
    .join(', ');
}
