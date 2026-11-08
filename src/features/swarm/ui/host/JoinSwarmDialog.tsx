import { useState } from 'react';
import { useHosts } from '@/core/host';
import { swarmLabel, useSwarms } from '@/core/swarm';
import type { HostView, JoinRole, JoinTarget } from '@/shared/api';
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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { useJoinSwarm } from '../../api/mutations';
import { t } from '@/shared/i18n';

const PASTE = 'paste';

/**
 * Joins a Host to a Swarm Homeport knows, its token fetched through a
 * connected manager, or to one by a pasted token and manager address.
 */
export function JoinSwarmDialog({ view, onClose }: { view: HostView; onClose: () => void }) {
  const swarms = useSwarms();
  const hosts = useHosts();
  const known = (swarms.data ?? []).filter((s) => s.reachable);
  const [choice, setChoice] = useState<string>(() => known[0]?.id ?? PASTE);
  const [role, setRole] = useState<JoinRole>('worker');
  const [token, setToken] = useState('');
  const [address, setAddress] = useState('');
  const [advertise, setAdvertise] = useState('');
  const join = useJoinSwarm(view.host.id, () => close());

  function close() {
    setToken('');
    join.reset();
    onClose();
  }

  const target: JoinTarget =
    choice === PASTE
      ? { kind: 'token', token, addresses: [address] }
      : { kind: 'known', swarmId: choice };
  const ready = choice !== PASTE || (token.trim() !== '' && address.trim() !== '');

  return (
    <Dialog open onOpenChange={(open) => !open && !join.isPending && close()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('swarm.membership.join.title', { name: view.host.name })}</DialogTitle>
          <DialogDescription>{t('swarm.membership.join.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            join.mutate({ target, role, advertise });
          }}
        >
          <div className="grid gap-1.5">
            <Label>{t('swarm.membership.join.swarm')}</Label>
            <Select value={choice} onValueChange={setChoice}>
              <SelectTrigger aria-label={t('swarm.membership.join.swarm')}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {known.map((s) => (
                  <SelectItem key={s.id} value={s.id}>
                    {swarmLabel(s, hosts.data)}
                  </SelectItem>
                ))}
                <SelectItem value={PASTE}>{t('swarm.membership.join.paste')}</SelectItem>
              </SelectContent>
            </Select>
          </div>
          {choice === PASTE && (
            <>
              <div className="grid gap-1.5">
                <Label htmlFor="join-token">{t('swarm.membership.join.token')}</Label>
                <Input
                  id="join-token"
                  autoComplete="off"
                  spellCheck={false}
                  className="font-mono text-xs"
                  placeholder="SWMTKN-1-…"
                  value={token}
                  onChange={(e) => setToken(e.target.value)}
                />
              </div>
              <div className="grid gap-1.5">
                <Label htmlFor="join-address">{t('swarm.membership.join.address')}</Label>
                <Input
                  id="join-address"
                  placeholder={t('swarm.membership.join.addressPlaceholder')}
                  value={address}
                  onChange={(e) => setAddress(e.target.value)}
                />
              </div>
            </>
          )}
          <div className="grid gap-1.5">
            <Label>{t('swarm.membership.join.as')}</Label>
            <ToggleGroup
              type="single"
              variant="outline"
              value={role}
              onValueChange={(v) => v && setRole(v as JoinRole)}
              className="justify-self-start"
            >
              <ToggleGroupItem value="worker">{t('swarm.role.worker')}</ToggleGroupItem>
              <ToggleGroupItem value="manager">{t('swarm.role.manager')}</ToggleGroupItem>
            </ToggleGroup>
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="join-advertise">{t('swarm.membership.advertise')}</Label>
            <Input
              id="join-advertise"
              value={advertise}
              placeholder={t('swarm.membership.advertisePlaceholder')}
              onChange={(e) => setAdvertise(e.target.value)}
            />
          </div>
          {join.error && <ErrorMessage error={join.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={close}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!ready || join.isPending}>
              {t('swarm.membership.join.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
