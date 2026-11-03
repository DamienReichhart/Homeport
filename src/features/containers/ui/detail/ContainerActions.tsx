import { PauseIcon, PlayIcon, RotateCwIcon, SquareIcon, Trash2Icon } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import type { Container } from '@/shared/api';
import { Button, type ConfirmOptions } from '@/shared/ui';
import { useContainerMutations } from '../../api/mutations';
import { removeConfirmation } from '../removeConfirmation';

/** The lifecycle buttons in a container page's header. */
export function ContainerActions({
  hostId,
  container,
  confirm,
}: {
  hostId: string;
  container: Container;
  confirm: (options: ConfirmOptions) => void;
}) {
  const navigate = useNavigate();
  const { act, remove, refresh } = useContainerMutations(hostId);
  const running = container.state === 'running';
  const paused = container.state === 'paused';
  const ids = [container.id];

  return (
    <>
      {running || paused ? (
        <Button variant="outline" onClick={() => act.mutate({ ids, action: 'stop' })}>
          <SquareIcon /> Stop
        </Button>
      ) : (
        <Button variant="outline" onClick={() => act.mutate({ ids, action: 'start' })}>
          <PlayIcon /> Start
        </Button>
      )}
      <Button
        variant="outline"
        disabled={!running && !paused}
        onClick={() => act.mutate({ ids, action: 'restart' })}
      >
        <RotateCwIcon /> Restart
      </Button>
      <Button
        variant="outline"
        disabled={!running && !paused}
        onClick={() => act.mutate({ ids, action: paused ? 'unpause' : 'pause' })}
      >
        {paused ? <PlayIcon /> : <PauseIcon />} {paused ? 'Resume' : 'Pause'}
      </Button>
      <Button
        variant="destructive"
        onClick={() =>
          confirm(
            removeConfirmation([container], async () => {
              await remove.mutateAsync({ ids, volumes: true });
              await refresh();
              navigate('/containers');
            }),
          )
        }
      >
        <Trash2Icon /> Remove
      </Button>
    </>
  );
}
