import { TransferDialog as Progress } from '@/shared/ui';
import { useImportImage, useLoadImages, useSaveImages } from '../../api/streams';

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
          title={`Saving ${transfer.names.length === 1 ? transfer.names[0] : `${transfer.names.length} images`}`}
          description={`To ${transfer.path}`}
          counted="Written"
          start={(onEvent, onError) => save(transfer.names, transfer.path, onEvent, onError)}
          onClose={onClose}
        />
      );
    case 'load':
      return (
        <Progress
          title="Loading images"
          description={`From ${transfer.path}`}
          counted="Sent"
          start={(onEvent, onError) => load(transfer.path, onEvent, onError)}
          done={(s) => (s.images.length > 0 ? `Done: ${s.images.join(', ')}` : 'Done.')}
          onClose={onClose}
        />
      );
    case 'import':
      return (
        <Progress
          title={`Importing ${transfer.reference}`}
          description={`From ${transfer.path}`}
          counted="Sent"
          start={(onEvent, onError) =>
            importImage(transfer.path, transfer.reference, transfer.changes, onEvent, onError)
          }
          done={(s) => `Done: ${s.images.join(', ')}`}
          onClose={onClose}
        />
      );
  }
}
