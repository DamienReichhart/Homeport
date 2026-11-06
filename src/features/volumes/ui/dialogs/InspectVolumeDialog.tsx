import { useQuery } from '@tanstack/react-query';
import type { VolumeView } from '@/shared/api';
import { Dialog, DialogContent, DialogHeader, DialogTitle, JsonView, Spinner } from '@/shared/ui';
import { volumeQueries } from '../../api/queries';
import { VolumeSize, type VolumeSizes } from '../VolumeSize';
import { t } from '@/shared/i18n';

export function InspectVolumeDialog({
  hostId,
  volume,
  sizes,
  onClose,
}: {
  hostId: string;
  volume: VolumeView;
  sizes: VolumeSizes;
  onClose: () => void;
}) {
  const inspect = useQuery(volumeQueries.inspect(hostId, volume.name));
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>{volume.name}</DialogTitle>
        </DialogHeader>
        <p className="flex items-center gap-2 text-sm">
          {t('volumes.size.label')} <VolumeSize sizes={sizes} name={volume.name} />
        </p>
        <p className="text-sm">
          {volume.usedBy.length
            ? t('volumes.usedBy', { containers: volume.usedBy.join(', ') })
            : t('volumes.notUsed')}
        </p>
        {inspect.data ? <JsonView value={inspect.data} /> : <Spinner />}
      </DialogContent>
    </Dialog>
  );
}
