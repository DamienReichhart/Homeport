import { useQuery } from '@tanstack/react-query';
import type { VolumeView } from '@/shared/api';
import { Dialog, DialogContent, DialogHeader, DialogTitle, JsonView, Spinner } from '@/shared/ui';
import { volumeQueries } from '../../api/queries';

export function InspectVolumeDialog({
  hostId,
  volume,
  onClose,
}: {
  hostId: string;
  volume: VolumeView;
  onClose: () => void;
}) {
  const inspect = useQuery(volumeQueries.inspect(hostId, volume.name));
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{volume.name}</DialogTitle>
        </DialogHeader>
        <p className="text-sm">
          {volume.usedBy.length
            ? `Used by ${volume.usedBy.join(', ')}.`
            : 'No container uses this volume.'}
        </p>
        {inspect.data ? <JsonView value={inspect.data} /> : <Spinner />}
      </DialogContent>
    </Dialog>
  );
}
