import { describeStatus } from '@/core/host';
import type { HostView } from '@/shared/api';
import { cn } from '@/shared/lib/utils';
import { t } from '@/shared/i18n';

/** "Engine 28.5.1" while connected, or what is wrong, with the reason in its tooltip. */
export function HostState({ view }: { view: HostView }) {
  const { label, tone, detail } = describeStatus(view.status);
  if (view.status.state === 'connected') {
    return (
      <span className="text-muted-foreground whitespace-nowrap">
        <span className="sr-only">{label} · </span>
        {detail}
      </span>
    );
  }
  return (
    <span
      className={cn(
        'block max-w-[16rem] truncate',
        tone === 'danger' && 'text-destructive',
        tone === 'warning' && 'text-warning',
        tone === 'muted' && 'text-muted-foreground',
      )}
      title={detail ? t('hosts.statusTitle', { label, detail }) : label}
    >
      {label}
    </span>
  );
}
