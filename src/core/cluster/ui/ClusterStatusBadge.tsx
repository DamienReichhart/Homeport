import type { ClusterStatus } from '@/shared/api';
import { StatusDot } from '@/shared/ui';
import { describeClusterStatus } from '../model/status';

export function ClusterStatusBadge({ status }: { status: ClusterStatus }) {
  const { label, tone } = describeClusterStatus(status);
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={tone} pulse={status.state === 'connecting'} />
      {label}
    </span>
  );
}
