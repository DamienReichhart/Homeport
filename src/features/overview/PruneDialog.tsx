import { useMutation, useQueryClient } from '@tanstack/react-query';
import { useState } from 'react';
import { toast } from 'sonner';
import { commands, keys, type PruneSelection } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Label,
} from '@/shared/ui';

const OPTIONS: {
  key: keyof PruneSelection;
  label: string;
  detail: string;
  destructive?: boolean;
}[] = [
  {
    key: 'containers',
    label: 'Stopped containers',
    detail: 'Removes every container that is not running.',
  },
  {
    key: 'images',
    label: 'Dangling images',
    detail: 'Untagged layers left behind by builds and pulls.',
  },
  {
    key: 'allImages',
    label: 'All unused images',
    detail: 'Also removes tagged images no container uses.',
  },
  {
    key: 'networks',
    label: 'Unused networks',
    detail: 'Custom networks no container is attached to.',
  },
  { key: 'buildCache', label: 'Build cache', detail: 'Cached build layers.' },
  {
    key: 'volumes',
    label: 'Unused volumes',
    detail: 'Deletes the data in every volume no container uses, named volumes included.',
    destructive: true,
  },
];

/** Global prune for one Host, with an explicit choice of what goes. */
export function PruneDialog({
  hostId,
  open,
  onOpenChange,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const client = useQueryClient();
  const [selection, setSelection] = useState<PruneSelection>({
    containers: true,
    images: true,
    allImages: false,
    networks: true,
    volumes: false,
    buildCache: true,
  });
  const prune = useMutation({
    meta: { silent: true },
    mutationFn: () =>
      commands.systemPrune(hostId, {
        ...selection,
        images: selection.images || selection.allImages,
      }),
    onSuccess: async (r) => {
      toast.success(
        `Removed ${r.containers} containers, ${r.images} images, ${r.networks} networks and ${r.volumes} volumes — ${formatBytes(r.spaceReclaimed)} reclaimed.`,
      );
      await client.invalidateQueries({ queryKey: keys.host(hostId) });
      onOpenChange(false);
    },
  });

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Clean up</DialogTitle>
          <DialogDescription>
            Choose what to remove from this host. This can&apos;t be undone.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          {OPTIONS.map((o) => (
            <div key={o.key} className="flex items-start gap-3">
              <Checkbox
                id={o.key}
                checked={selection[o.key]}
                onCheckedChange={(v) => setSelection((s) => ({ ...s, [o.key]: v === true }))}
              />
              <Label htmlFor={o.key} className="grid gap-0.5 font-normal">
                <span className={o.destructive ? 'text-destructive font-medium' : 'font-medium'}>
                  {o.label}
                </span>
                <span
                  className={
                    o.destructive ? 'text-destructive text-xs' : 'text-muted-foreground text-xs'
                  }
                >
                  {o.detail}
                </span>
              </Label>
            </div>
          ))}
        </div>
        {prune.error && <ErrorMessage error={prune.error} />}
        <DialogFooter>
          <Button variant="ghost" onClick={() => onOpenChange(false)}>
            Cancel
          </Button>
          <Button
            variant="destructive"
            className={
              selection.volumes ? 'bg-destructive text-white hover:bg-destructive/90' : undefined
            }
            disabled={prune.isPending || !Object.values(selection).some(Boolean)}
            onClick={() => prune.mutate()}
          >
            {selection.volumes ? 'Delete, including volume data' : 'Clean up'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
