import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { commands, keys, type Container } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
} from '@/shared/ui';

export function RenameDialog({
  hostId,
  container,
  onClose,
}: {
  hostId: string;
  container: Container | null;
  onClose: () => void;
}) {
  const client = useQueryClient();
  const [name, setName] = useState(container?.name ?? '');
  const rename = useMutation({
    meta: { silent: true },
    mutationFn: () => commands.containersRename(hostId, container?.id ?? '', name),
    onSuccess: async () => {
      await client.invalidateQueries({ queryKey: keys.containers(hostId) });
      onClose();
    },
  });
  return (
    <Dialog open={container !== null} onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Rename {container?.name}</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            rename.mutate();
          }}
        >
          <Input
            value={name}
            onChange={(e) => setName(e.target.value)}
            autoFocus
            aria-label="New name"
          />
          {rename.error && <ErrorMessage error={rename.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit" disabled={rename.isPending}>
              Rename
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
