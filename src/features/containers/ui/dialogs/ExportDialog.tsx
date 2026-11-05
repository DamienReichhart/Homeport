import { useEffect, useRef, useState } from 'react';
import type { Container } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import { TransferDialog } from '@/shared/ui';
import { exportContainer } from '../../api/streams';
import { t } from '@/shared/i18n';

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
      t('containers.export.pick', { name: container.name }),
      `${container.name}.tar`,
      [{ name: t('containers.export.tarArchive'), extensions: ['tar'] }],
    ).then((picked) => (picked ? setPath(picked) : onClose()));
  }, [container.name, onClose]);
  if (!path) return null;
  return (
    <TransferDialog
      title={t('containers.export.title', { name: container.name })}
      description={t('containers.export.to', { path })}
      counted={t('containers.export.written')}
      start={(onEvent, onError) => exportContainer(hostId, container.id, path, onEvent, onError)}
      done={() => t('containers.export.finished')}
      onClose={onClose}
    />
  );
}
