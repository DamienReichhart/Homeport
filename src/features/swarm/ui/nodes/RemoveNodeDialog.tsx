import { useState } from 'react';
import type { SwarmNode } from '@/shared/api';
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
import { useRemoveNode } from '../../api/mutations';
import { t } from '@/shared/i18n';

/** Removing a node; one that's still up needs Force. */
export function RemoveNodeDialog({
  swarmId,
  node,
  onClose,
}: {
  swarmId: string;
  node: SwarmNode;
  onClose: () => void;
}) {
  const ready = node.state === 'ready';
  const [force, setForce] = useState(false);
  const remove = useRemoveNode(swarmId, onClose);
  return (
    <Dialog open onOpenChange={(open) => !open && !remove.isPending && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-destructive">
            {t('swarm.nodes.remove.title', { name: node.hostname })}
          </DialogTitle>
          <DialogDescription>
            {ready ? t('swarm.nodes.remove.ready') : t('swarm.nodes.remove.down')}
          </DialogDescription>
        </DialogHeader>
        {ready && (
          <div className="flex items-center gap-2">
            <Checkbox id="force" checked={force} onCheckedChange={(v) => setForce(v === true)} />
            <Label htmlFor="force">{t('swarm.nodes.remove.force')}</Label>
          </div>
        )}
        {remove.error && <ErrorMessage error={remove.error} />}
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button
            variant="destructive"
            disabled={remove.isPending || (ready && !force)}
            onClick={() => remove.mutate({ node, force })}
          >
            {t('common.remove')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
