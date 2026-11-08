import type { Container } from '@/shared/api';
import { useConfirm } from '@/shared/ui';
import { ContainerMenu } from './ContainerMenu';
import { useContainerDialogs } from './dialogs/ContainerDialogs';

/**
 * A container's start/stop toggle and menu, with the dialogs and
 * confirmations they open, for a list that isn't bound to one Host.
 */
export function ContainerRowMenu({ hostId, container }: { hostId: string; container: Container }) {
  const [dialog, confirm] = useConfirm();
  const [dialogs, openDialog] = useContainerDialogs(hostId);
  return (
    <>
      <ContainerMenu
        hostId={hostId}
        container={container}
        onOpenDialog={openDialog}
        confirm={confirm}
      />
      {dialog}
      {dialogs}
    </>
  );
}
