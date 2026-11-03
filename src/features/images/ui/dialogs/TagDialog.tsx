import { useState } from 'react';
import type { Image } from '@/shared/api';
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
import { useTagImage } from '../../api/mutations';
import { imageName } from '../../model/imageName';

/** Gives an image another name, e.g. to push it to a registry. */
export function TagDialog({
  hostId,
  image,
  onClose,
}: {
  hostId: string;
  image: Image;
  onClose: () => void;
}) {
  const [target, setTarget] = useState('');
  const tag = useTagImage(hostId, image.id, onClose);
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Tag {imageName(image)}</DialogTitle>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            tag.mutate(target);
          }}
        >
          <Input
            value={target}
            onChange={(e) => setTarget(e.target.value)}
            placeholder="registry.example.com/app:v2"
            autoFocus
            aria-label="New name"
          />
          {tag.error && <ErrorMessage error={tag.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              Cancel
            </Button>
            <Button type="submit" disabled={tag.isPending || !target.trim()}>
              Add tag
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
