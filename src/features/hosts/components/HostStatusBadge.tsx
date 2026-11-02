import type { HostStatus } from '@/shared/api';
import { StatusDot } from '@/shared/ui';
import { describeStatus } from '../status';

export function HostStatusBadge({ status }: { status: HostStatus }) {
  const { label, tone } = describeStatus(status);
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={tone} pulse={status.state === 'connecting'} />
      {label}
    </span>
  );
}
