import { describeStatus } from '@/core/host';
import type { HostStatus } from '@/shared/api';

/** The status and its reason, shown when hovering a Host in a list. */
export function statusTitle(status: HostStatus): string {
  const { label, detail } = describeStatus(status);
  return detail ? `${label}: ${detail}` : label;
}
