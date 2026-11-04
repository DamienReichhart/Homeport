import { useEffect, useRef, useState } from 'react';
import type { Container } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import { TransferDialog } from '@/shared/ui';
import { exportContainer } from '../../api/streams';

const TAR = [{ name: 'Tar archive', extensions: ['tar'] }];

/** Asks where to save a container's filesystem, then exports it there with progress. */
export function ExportDialog({
  hostId,
  container,
  onClose,
}: {
  hostId: string;
  container: Container;
  onClose: () => void;
}) {
  const [path, setPath] = useState<string | null>(null);
  const asked = useRef(false);
  useEffect(() => {
    if (asked.current) return;
    asked.current = true;
    void pickSavePath(
      `Export the filesystem of ${container.name}`,
      `${container.name}.tar`,
      TAR,
    ).then((picked) => (picked ? setPath(picked) : onClose()));
  }, [container.name, onClose]);
  if (!path) return null;
  return (
    <TransferDialog
      title={`Exporting ${container.name}`}
      description={`To ${path}`}
      counted="Written"
      start={(onEvent, onError) => exportContainer(hostId, container.id, path, onEvent, onError)}
      done={() => 'Done. Import it on the Images page, or with docker import.'}
      onClose={onClose}
    />
  );
}
