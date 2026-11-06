import { TransferDialog } from '@/shared/ui';
import { useBackupVolume } from '../../api/streams';
import { VolumeActionFlow } from '../VolumeActionFlow';
import { t } from '@/shared/i18n';

/** Backs `volume` up to the `.tar` at `destination` on this machine. */
export function BackupVolumeDialog({
  hostId,
  volume,
  destination,
  onClose,
}: {
  hostId: string;
  volume: string;
  destination: string;
  onClose: () => void;
}) {
  const backup = useBackupVolume(hostId);
  return (
    <VolumeActionFlow
      hostId={hostId}
      volume={volume}
      mode="backup"
      title={t('volumes.backup.title', { volume })}
      onClose={onClose}
      transfer={({ pulledImage, onFinished }) => (
        <TransferDialog
          title={t('volumes.backup.title', { volume })}
          description={t('volumes.backup.to', { path: destination })}
          counted={t('volumes.transfer.written')}
          start={(onEvent, onError) => backup(volume, destination, pulledImage, onEvent, onError)}
          onClose={onFinished}
        />
      )}
    />
  );
}
