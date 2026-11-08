import { useState } from 'react';
import type { ServiceView } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { useScaleService } from '../../api/mutations';
import { parseReplicas } from '../../model/services';
import { t } from '@/shared/i18n';

/** Sets the replicas of a replicated service. */
export function ScaleDialog({
  swarmId,
  service,
  onClose,
}: {
  swarmId: string;
  service: ServiceView;
  onClose: () => void;
}) {
  const [text, setText] = useState(String(service.replicas ?? 1));
  const replicas = parseReplicas(text);
  const scale = useScaleService(swarmId, onClose);
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('swarm.services.scale.title', { name: service.name })}</DialogTitle>
          <DialogDescription>{t('swarm.services.scale.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            if (replicas !== null) scale.mutate({ service, replicas });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="replicas">{t('swarm.services.scale.replicas')}</Label>
            <Input
              id="replicas"
              inputMode="numeric"
              value={text}
              autoFocus
              onChange={(e) => setText(e.target.value)}
              aria-invalid={replicas === null}
            />
            {replicas === null && (
              <p className="text-destructive text-sm">{t('swarm.services.scale.invalid')}</p>
            )}
          </div>
          {scale.error && <ErrorMessage error={scale.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={replicas === null || scale.isPending}>
              {t('swarm.services.action.scale')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
