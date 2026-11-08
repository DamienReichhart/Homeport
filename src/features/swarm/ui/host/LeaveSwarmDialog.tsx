import { useState } from 'react';
import type { HostView } from '@/shared/api';
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
  Input,
  Label,
} from '@/shared/ui';
import { useLeaveSwarm } from '../../api/mutations';
import { confirmsLeave, type LeaveRisk } from '../../model/membership';
import { t } from '@/shared/i18n';

/**
 * Has a Host leave its Swarm. A manager needs Force; the last manager also
 * needs the Host's name typed, since the Swarm goes with it.
 */
export function LeaveSwarmDialog({
  view,
  risk,
  onClose,
}: {
  view: HostView;
  risk: LeaveRisk;
  onClose: () => void;
}) {
  const [force, setForce] = useState(false);
  const [typed, setTyped] = useState('');
  const leave = useLeaveSwarm(view.host.id, onClose);
  const allowed =
    risk === 'worker' ||
    (risk === 'manager' && force) ||
    (risk === 'lastManager' && confirmsLeave(typed, view.host.name));
  return (
    <Dialog open onOpenChange={(open) => !open && !leave.isPending && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle className="text-destructive">
            {t('swarm.membership.leave.title', { name: view.host.name })}
          </DialogTitle>
          <DialogDescription>
            {risk === 'worker'
              ? t('swarm.membership.leave.worker')
              : risk === 'manager'
                ? t('swarm.membership.leave.manager')
                : t('swarm.membership.leave.last')}
          </DialogDescription>
        </DialogHeader>
        {risk === 'manager' && (
          <div className="flex items-center gap-2">
            <Checkbox
              id="leave-force"
              checked={force}
              onCheckedChange={(v) => setForce(v === true)}
            />
            <Label htmlFor="leave-force">{t('swarm.membership.leave.force')}</Label>
          </div>
        )}
        {risk === 'lastManager' && (
          <div className="grid gap-1.5">
            <Label htmlFor="leave-confirm">
              {t('swarm.membership.leave.typeName', { name: view.host.name })}
            </Label>
            <Input id="leave-confirm" value={typed} onChange={(e) => setTyped(e.target.value)} />
          </div>
        )}
        {leave.error && <ErrorMessage error={leave.error} />}
        <DialogFooter>
          <Button variant="ghost" onClick={onClose}>
            {t('common.cancel')}
          </Button>
          <Button
            variant="destructive"
            disabled={!allowed || leave.isPending}
            onClick={() => leave.mutate(risk !== 'worker')}
          >
            {t('swarm.membership.leave.submit')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
