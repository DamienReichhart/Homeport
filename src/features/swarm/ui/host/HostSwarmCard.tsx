import { useQuery } from '@tanstack/react-query';
import { LogInIcon, LogOutIcon, NetworkIcon } from 'lucide-react';
import { useState } from 'react';
import { swarmQueries } from '@/core/swarm';
import type { HostView } from '@/shared/api';
import {
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from '@/shared/ui';
import { leaveRisk } from '../../model/membership';
import { HostSwarmLink } from '../HostSwarmLink';
import { InitSwarmDialog } from './InitSwarmDialog';
import { JoinSwarmDialog } from './JoinSwarmDialog';
import { LeaveSwarmDialog } from './LeaveSwarmDialog';
import { t } from '@/shared/i18n';

type Open = 'init' | 'join' | 'leave' | null;

/**
 * A connected Host's place in a Swarm: Init or Join outside one; its role,
 * a link to the Swarm and Leave inside one.
 */
export function HostSwarmCard({ view }: { view: HostView }) {
  const [open, setOpen] = useState<Open>(null);
  const status = view.status.state === 'connected' ? view.status : null;
  const membership = status?.swarm ?? null;
  const swarmId = membership?.swarmId ?? null;
  const summary = useQuery({ ...swarmQueries.summary(swarmId ?? ''), enabled: !!swarmId });
  if (!status) return null;
  const close = () => setOpen(null);
  return (
    <Card>
      <CardHeader>
        <CardTitle className="flex items-center gap-2">
          <NetworkIcon className="size-4" /> {t('swarm.membership.card.title')}
        </CardTitle>
        <CardDescription>
          {membership ? (
            <span className="grid gap-1">
              <HostSwarmLink hostId={view.host.id} />
              {membership.state !== 'active' && (
                <span>{t('swarm.membership.card.state', { state: membership.state })}</span>
              )}
              {membership.error && <span className="text-destructive">{membership.error}</span>}
            </span>
          ) : (
            t('swarm.membership.card.none')
          )}
        </CardDescription>
        <CardAction className="flex gap-2">
          {membership ? (
            <Button variant="outline" onClick={() => setOpen('leave')}>
              <LogOutIcon /> {t('swarm.membership.leave.open')}
            </Button>
          ) : (
            <>
              <Button variant="outline" onClick={() => setOpen('join')}>
                <LogInIcon /> {t('swarm.membership.join.open')}
              </Button>
              <Button onClick={() => setOpen('init')}>{t('swarm.membership.init.open')}</Button>
            </>
          )}
        </CardAction>
      </CardHeader>
      {!membership && (
        <CardContent className="text-muted-foreground text-sm">
          {t('swarm.membership.card.ports')}
        </CardContent>
      )}
      {open === 'init' && <InitSwarmDialog view={view} onClose={close} />}
      {open === 'join' && <JoinSwarmDialog view={view} onClose={close} />}
      {open === 'leave' && membership && (
        <LeaveSwarmDialog
          view={view}
          risk={leaveRisk(membership.role, summary.data?.managers)}
          onClose={close}
        />
      )}
    </Card>
  );
}
