import { useState } from 'react';
import type { IncusInstance } from '@/shared/api';
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
  Switch,
} from '@/shared/ui';
import { useSnapshots } from '../api/mutations';
import { expiresAt, type Expiry } from '../model/snapshots';
import { t, type MessageKey } from '@/shared/i18n';

const EXPIRIES: Record<Expiry, MessageKey> = {
  never: 'incus.snapshots.expiry.never',
  '1d': 'incus.snapshots.expiry.1d',
  '7d': 'incus.snapshots.expiry.7d',
  '30d': 'incus.snapshots.expiry.30d',
};

/** Takes a snapshot: named (or `snapN`), stateful when the Instance runs and allows it. */
export function TakeSnapshotDialog({
  hostId,
  project,
  instance,
  statefulAllowed,
  open,
  onClose,
}: {
  hostId: string;
  project: string;
  instance: IncusInstance;
  statefulAllowed: boolean;
  open: boolean;
  onClose: () => void;
}) {
  const { take } = useSnapshots(hostId, project, instance.name);
  const [name, setName] = useState('');
  const [stateful, setStateful] = useState(false);
  const [expiry, setExpiry] = useState<Expiry>('never');
  const canStateful = statefulAllowed && instance.status === 'running';
  const close = () => {
    setName('');
    setStateful(false);
    setExpiry('never');
    take.reset();
    onClose();
  };
  return (
    <Dialog open={open} onOpenChange={(next) => !next && close()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('incus.snapshots.take.title', { name: instance.name })}</DialogTitle>
          <DialogDescription>{t('incus.snapshots.take.description')}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          <div className="grid gap-1.5">
            <Label htmlFor="snapshot-name">{t('incus.snapshots.take.name')}</Label>
            <Input
              id="snapshot-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t('incus.snapshots.take.namePlaceholder')}
            />
          </div>
          <div className="grid gap-1.5">
            <Label>{t('incus.snapshots.take.expiry')}</Label>
            <Select value={expiry} onValueChange={(v) => setExpiry(v as Expiry)}>
              <SelectTrigger className="w-full" aria-label={t('incus.snapshots.take.expiry')}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {(Object.keys(EXPIRIES) as Expiry[]).map((e) => (
                  <SelectItem key={e} value={e}>
                    {t(EXPIRIES[e])}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
          {canStateful && (
            <div className="flex items-center gap-2">
              <Switch id="snapshot-stateful" checked={stateful} onCheckedChange={setStateful} />
              <Label htmlFor="snapshot-stateful">{t('incus.snapshots.take.stateful')}</Label>
            </div>
          )}
          {take.error && <ErrorMessage error={take.error} />}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={close}>
            {t('common.cancel')}
          </Button>
          <Button
            disabled={take.isPending}
            onClick={() =>
              take
                .mutateAsync({
                  name: name.trim(),
                  stateful: canStateful && stateful,
                  expiresAt: expiresAt(expiry),
                })
                .then(close)
                .catch(() => {})
            }
          >
            {t('incus.snapshots.take')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
