import { useQuery } from '@tanstack/react-query';
import type { VolumeView } from '@/shared/api';
import { formatDateTime } from '@/shared/lib';
import {
  Badge,
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  JsonView,
  KeyValue,
  Spinner,
} from '@/shared/ui';
import { volumeQueries } from '../../api/queries';
import { VolumeSize, type VolumeSizes } from '../VolumeSize';
import { t } from '@/shared/i18n';

/** A volume's facts, its driver among them, then its raw JSON. */
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
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle className="break-all">{volume.name}</DialogTitle>
        </DialogHeader>
        <KeyValue
          items={[
            [
              t('volumes.column.driver'),
              <span className="font-mono text-xs">{volume.driver}</span>,
            ],
            [t('volumes.column.stack'), volume.stack],
            [t('volumes.column.size'), <VolumeSize sizes={sizes} name={volume.name} />],
            [
              t('volumes.column.usedBy'),
              volume.usedBy.length ? (
                volume.usedBy.join(', ')
              ) : (
                <Badge variant="outline">{t('volumes.unused')}</Badge>
              ),
            ],
            [
              t('volumes.column.created'),
              volume.createdAt ? formatDateTime(volume.createdAt) : null,
            ],
            [
              t('volumes.mountpoint'),
              <span className="font-mono text-xs">{volume.mountpoint}</span>,
            ],
          ]}
        />
        {inspect.data ? <JsonView value={inspect.data} /> : <Spinner />}
      </DialogContent>
    </Dialog>
  );
}
