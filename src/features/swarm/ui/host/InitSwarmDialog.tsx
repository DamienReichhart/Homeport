import { useState } from 'react';
import type { HostView } from '@/shared/api';
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
import { useInitSwarm } from '../../api/mutations';
import { advertiseFromConnection } from '../../model/membership';
import { t } from '@/shared/i18n';

/** Makes a Host the first manager of a new Swarm. */
export function InitSwarmDialog({ view, onClose }: { view: HostView; onClose: () => void }) {
  const [advertise, setAdvertise] = useState(() => advertiseFromConnection(view.host.connection));
  const [listen, setListen] = useState('0.0.0.0:2377');
  const init = useInitSwarm(view.host.id, onClose);
  return (
    <Dialog open onOpenChange={(open) => !open && !init.isPending && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('swarm.membership.init.title', { name: view.host.name })}</DialogTitle>
          <DialogDescription>{t('swarm.membership.init.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            init.mutate({ advertise, listen });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="advertise">{t('swarm.membership.advertise')}</Label>
            <Input
              id="advertise"
              value={advertise}
              placeholder={t('swarm.membership.advertisePlaceholder')}
              onChange={(e) => setAdvertise(e.target.value)}
            />
            <p className="text-muted-foreground text-xs">{t('swarm.membership.advertiseHelp')}</p>
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="listen">{t('swarm.membership.listen')}</Label>
            <Input id="listen" value={listen} onChange={(e) => setListen(e.target.value)} />
          </div>
          {init.error && <ErrorMessage error={init.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={init.isPending}>
              {t('swarm.membership.init.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
