import type { ServiceView } from '@/shared/api';
import { Badge } from '@/shared/ui';
import { underReplicated } from '../../model/services';

/** The running/desired badge, in warning tone while fewer run than should. */
export function ReplicasBadge({ service }: { service: ServiceView }) {
  return (
    <Badge
      variant="outline"
      className={underReplicated(service) ? 'border-warning text-warning' : undefined}
    >
      {service.running}/{service.desired}
    </Badge>
  );
}
