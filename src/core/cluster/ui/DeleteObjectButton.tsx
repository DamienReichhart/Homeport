import { EllipsisIcon, TrashIcon } from 'lucide-react';
import type { ClusterView } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/shared/ui';
import type { DeleteTarget } from '../model/deletion';
import { useDeleteObjects } from './useDeleteObjects';
import { t } from '@/shared/i18n';

/**
 * An object page's More actions menu, holding Delete: it asks first, naming
 * the object and its Cluster; `onDeleted` usually leaves its page. Kept out of
 * the header's row of buttons, as every page's destructive action is.
 */
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
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" size="icon-sm" aria-label={t('common.moreActions')}>
            <EllipsisIcon />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="end">
          <DropdownMenuItem variant="destructive" onSelect={() => remove([target], onDeleted)}>
            <TrashIcon /> {t('core.delete.action')}
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
      {dialog}
    </>
  );
}
