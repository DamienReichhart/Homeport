import { useState } from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  Textarea,
} from '@/shared/ui';
import { importName } from '../../model/transfer';

/** Names the image a root filesystem tar becomes, before importing it. */
export function ImportDialog({
  path,
  onImport,
  onClose,
}: {
  path: string;
  onImport: (reference: string, changes: string[]) => void;
  onClose: () => void;
}) {
  const [reference, setReference] = useState(() => importName(path));
  const [changes, setChanges] = useState('');
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>Import a filesystem as an image</DialogTitle>
          <DialogDescription className="break-all">
            {path} becomes a one-layer image with no history or settings of its own; give it a
            command below to make it runnable.
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            onImport(
              reference.trim(),
              changes
                .split('\n')
                .map((l) => l.trim())
                .filter(Boolean),
            );
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="import-name">Image name</Label>
            <Input
              id="import-name"
              value={reference}
              onChange={(e) => setReference(e.target.value)}
              className="font-mono"
              autoFocus
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="import-changes">Changes</Label>
            <Textarea
              id="import-changes"
              value={changes}
              onChange={(e) => setChanges(e.target.value)}
              placeholder={'CMD ["/bin/sh"]\nENV LANG=C.UTF-8'}
              className="h-20 font-mono text-xs"
              spellCheck={false}
            />
            <p className="text-muted-foreground text-xs">
              Dockerfile instructions, one per line: CMD, ENTRYPOINT, ENV, EXPOSE, LABEL, ONBUILD,
              USER, VOLUME or WORKDIR.
            </p>
          </div>
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit" disabled={!reference.trim()}>
              Import
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
