import type { ServiceView } from '@/shared/api';
import { Badge } from '@/shared/ui';
import { underReplicated } from '../../model/services';

/**
 * The running/desired badge: amber while fewer run than should, red when
 * none of them runs, quiet otherwise.
 */
export function ReplicasBadge({ service }: { service: ServiceView }) {
  const variant = !underReplicated(service)
    ? 'outline'
    : service.running === 0
      ? 'destructive'
      : 'warning';
  return (
    <Badge variant={variant} className="tabular-nums">
      {service.running}/{service.desired}
    </Badge>
  );
}
