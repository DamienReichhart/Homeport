import { describeStatus } from '@/core/host';
import type { HostStatus } from '@/shared/api';
import { t } from '@/shared/i18n';

/** The status and its reason, shown when hovering a Host in a list. */
export function statusTitle(status: HostStatus): string {
  const { label, detail } = describeStatus(status);
  return detail ? t('hosts.statusTitle', { label, detail }) : label;
}
