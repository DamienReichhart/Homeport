import { TransferDialog as Progress } from '@/shared/ui';
import { useImportImage, useLoadImages, useSaveImages } from '../../api/streams';
import { t } from '@/shared/i18n';

/** A save of `names` to `path`, a load from `path`, or an import of the root filesystem at `path`. */
export type Transfer =
  | { kind: 'save'; names: string[]; path: string }
  | { kind: 'load'; path: string }
  | { kind: 'import'; path: string; reference: string; changes: string[] };

/** Runs one save, load or import and shows its progress, with a way to cancel. */
export function TransferDialog({
  hostId,
  transfer,
  onClose,
}: {
  hostId: string;
  transfer: Transfer;
  onClose: () => void;
}) {
  const save = useSaveImages(hostId);
  const load = useLoadImages(hostId);
  const importImage = useImportImage(hostId);
  switch (transfer.kind) {
    case 'save':
      return (
        <Progress
          title={
            transfer.names.length === 1
              ? t('images.transfer.savingOne', { name: transfer.names[0] })
              : t('images.transfer.saving', { count: transfer.names.length })
          }
          description={t('images.transfer.to', { path: transfer.path })}
          counted={t('images.transfer.written')}
          start={(onEvent, onError) => save(transfer.names, transfer.path, onEvent, onError)}
          onClose={onClose}
        />
      );
    case 'load':
      return (
        <Progress
          title={t('images.transfer.loading')}
          description={t('images.transfer.from', { path: transfer.path })}
          counted={t('images.transfer.sent')}
          start={(onEvent, onError) => load(transfer.path, onEvent, onError)}
          done={(s) =>
            s.images.length > 0
              ? t('images.transfer.doneWith', { images: s.images.join(', ') })
              : t('images.done')
          }
          onClose={onClose}
        />
      );
    case 'import':
      return (
        <Progress
          title={t('images.transfer.importing', { reference: transfer.reference })}
          description={t('images.transfer.from', { path: transfer.path })}
          counted={t('images.transfer.sent')}
          start={(onEvent, onError) =>
            importImage(transfer.path, transfer.reference, transfer.changes, onEvent, onError)
          }
          done={(s) => t('images.transfer.doneWith', { images: s.images.join(', ') })}
          onClose={onClose}
        />
      );
  }
}
