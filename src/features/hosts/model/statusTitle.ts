import { describeStatus } from '@/core/host';
import type { HostStatus } from '@/shared/api';
import { t } from '@/shared/i18n';

/** The status and its reason, shown when hovering a Host in a list. */
export function statusTitle(status: HostStatus): string {
  return summaryTitle(describeStatus(status));
}

/** A status's label and reason, in one line. */
export function summaryTitle({ label, detail }: { label: string; detail?: string }): string {
  return detail ? t('hosts.statusTitle', { label, detail }) : label;
}
