import { TrashIcon } from 'lucide-react';
import type { ClusterView } from '@/shared/api';
import { Button } from '@/shared/ui';
import type { DeleteTarget } from '../model/deletion';
import { useDeleteObjects } from './useDeleteObjects';
import { t } from '@/shared/i18n';

/** Deletes one object after a confirmation naming it; `onDeleted` usually leaves its page. */
export function DeleteObjectButton({
  cluster,
  target,
  onDeleted,
}: {
  cluster: ClusterView;
  target: DeleteTarget;
  onDeleted?: () => void;
}) {
  const [dialog, remove] = useDeleteObjects(cluster);
  return (
    <>
      <Button
        variant="outline"
        size="sm"
        className="text-destructive"
        onClick={() => remove([target], onDeleted)}
      >
        <TrashIcon /> {t('core.delete.action')}
      </Button>
      {dialog}
    </>
  );
}
