import { PlayIcon, RotateCwIcon, SquareIcon, Trash2Icon } from 'lucide-react';
import type { Container } from '@/shared/api';
import { Button, type ConfirmOptions } from '@/shared/ui';
import { useContainerMutations } from '../../api/mutations';
import { removeConfirmation } from '../removeConfirmation';

/** Start, stop, restart and remove for the selected containers. */
export function SelectionActions({
  hostId,
  ids,
  selected,
  onRemoved,
  confirm,
}: {
  hostId: string;
  /** Every selected id, as given to the Engine. */
  ids: string[];
  /** The selected containers still in the list, for the confirmation. */
  selected: Container[];
  onRemoved: () => void;
  confirm: (options: ConfirmOptions) => void;
}) {
  const { act, remove } = useContainerMutations(hostId);
  return (
    <div className="ml-auto flex items-center gap-1.5">
      <span className="text-muted-foreground text-sm">{selected.length} selected</span>
      <Button size="sm" variant="outline" onClick={() => act.mutate({ ids, action: 'start' })}>
        <PlayIcon /> Start
      </Button>
      <Button size="sm" variant="outline" onClick={() => act.mutate({ ids, action: 'stop' })}>
        <SquareIcon /> Stop
      </Button>
      <Button size="sm" variant="outline" onClick={() => act.mutate({ ids, action: 'restart' })}>
        <RotateCwIcon /> Restart
      </Button>
      <Button
        size="sm"
        variant="destructive"
        onClick={() =>
          confirm(
            removeConfirmation(selected, async () => {
              await remove.mutateAsync({ ids, volumes: true });
              onRemoved();
            }),
          )
        }
      >
        <Trash2Icon /> Remove
      </Button>
    </div>
  );
}
