import { useState, type ReactNode } from 'react';
import type { Container } from '@/shared/api';
import { CommitDialog } from './CommitDialog';
import { DuplicateDialog } from './DuplicateDialog';
import { EditDialog } from './EditDialog';
import { RenameDialog } from './RenameDialog';

/** The dialogs a container's menu opens. */
export type ContainerDialog = 'rename' | 'duplicate' | 'edit' | 'commit';

export interface ContainerDialogRequest {
  kind: ContainerDialog;
  container: Container;
}

/** `const [dialogs, open] = useContainerDialogs(hostId)`: render `dialogs`, call `open(...)`. */
export function useContainerDialogs(
  hostId: string,
): [ReactNode, (request: ContainerDialogRequest) => void] {
  const [request, setRequest] = useState<ContainerDialogRequest | null>(null);
  const close = () => setRequest(null);
  const key = request && `${request.kind}/${request.container.id}`;
  let dialog: ReactNode = null;
  switch (request?.kind) {
    case 'rename':
      dialog = (
        <RenameDialog key={key} hostId={hostId} container={request.container} onClose={close} />
      );
      break;
    case 'edit':
      dialog = (
        <EditDialog
          key={key}
          hostId={hostId}
          container={request.container}
          onClose={close}
          onCommitFirst={() => setRequest({ kind: 'commit', container: request.container })}
        />
      );
      break;
    case 'commit':
      dialog = (
        <CommitDialog key={key} hostId={hostId} container={request.container} onClose={close} />
      );
      break;
    case 'duplicate':
      dialog = (
        <DuplicateDialog key={key} hostId={hostId} container={request.container} onClose={close} />
      );
      break;
  }
  return [dialog, setRequest];
}
