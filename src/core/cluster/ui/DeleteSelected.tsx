import { TrashIcon } from 'lucide-react';
import type { ClusterView } from '@/shared/api';
import { Button } from '@/shared/ui';
import type { DeleteTarget } from '../model/deletion';
import { useDeleteObjects } from './useDeleteObjects';
import { t } from '@/shared/i18n';

/** A list's "Delete N" for the rows picked; nothing while none is. */
export function DeleteSelected({
  cluster,
  targets,
}: {
  cluster: ClusterView;
  targets: DeleteTarget[];
}) {
  const [dialog, remove] = useDeleteObjects(cluster);
  return (
    <>
      {targets.length > 0 && (
        <Button
          variant="outline"
          size="sm"
          className="text-destructive"
          onClick={() => remove(targets)}
        >
          <TrashIcon /> {t('core.delete.selected', { count: targets.length })}
        </Button>
      )}
      {dialog}
    </>
  );
}
