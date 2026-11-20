import type { ServiceView } from '@/shared/api';
import { Badge } from '@/shared/ui';
import { updateTone } from '../../model/services';

/** A service's update state: named while under way or stuck, quiet once done. */
export function UpdateState({ service }: { service: ServiceView }) {
  const tone = updateTone(service.updateState);
  if (!service.updateState) return <span className="text-muted-foreground">—</span>;
  if (!tone)
    return (
      <span className="text-muted-foreground text-xs" title={service.updateMessage ?? undefined}>
        {service.updateState}
      </span>
    );
  return (
    <Badge
      variant={tone === 'danger' ? 'destructive' : 'warning'}
      title={service.updateMessage ?? undefined}
    >
      {service.updateState}
    </Badge>
  );
}
