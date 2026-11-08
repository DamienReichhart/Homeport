import { useState } from 'react';
import { useNavigate, useSearchParams } from 'react-router-dom';
import { HostGate, useHosts, useHostSelection } from '@/core/host';
import { SwarmGate, swarmLabel, useSwarms } from '@/core/swarm';
import type { StackOwner } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  Label,
  PageHeader,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { NewLinkedStack } from './NewLinkedStack';
import { NewManagedStack } from './NewManagedStack';
import { NewSyncedStack } from './NewSyncedStack';
import { t } from '@/shared/i18n';

type Mode = 'managed' | 'linked' | 'synced';

const HOST = 'host';

/** Where the new Stack goes: the selected Host, or a Swarm with a connected manager. */
function TargetPicker({ owner }: { owner: StackOwner }) {
  const navigate = useNavigate();
  const { host } = useHostSelection();
  const swarms = useSwarms();
  const hosts = useHosts().data;
  const reachable = (swarms.data ?? []).filter((s) => s.reachable);
  if (reachable.length === 0 && owner.kind === 'host') return null;
  return (
    <div className="grid max-w-sm gap-1.5">
      <Label>{t('stacks.newPage.target')}</Label>
      <Select
        value={owner.kind === 'swarm' ? owner.id : HOST}
        onValueChange={(v) =>
          void navigate(v === HOST ? '/stacks/new' : `/stacks/new?swarm=${encodeURIComponent(v)}`, {
            replace: true,
          })
        }
      >
        <SelectTrigger aria-label={t('stacks.newPage.target')}>
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {host && (
            <SelectItem value={HOST}>
              {t('stacks.newPage.targetHost', { name: host.host.name })}
            </SelectItem>
          )}
          {reachable.map((s) => (
            <SelectItem key={s.id} value={s.id}>
              {swarmLabel(s, hosts)}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </div>
  );
}

function NewStack({ owner }: { owner: StackOwner }) {
  const [params] = useSearchParams();
  const adopting = params.get('adopt');
  const [mode, setMode] = useState<Mode>('managed');

  return (
    <div className="grid gap-4">
      <PageHeader
        title={adopting ? t('stacks.newPage.adoptTitle', { name: adopting }) : t('stacks.new')}
        description={
          adopting
            ? t('stacks.newPage.adoptDescription')
            : mode === 'linked'
              ? t('stacks.newLinked.description')
              : mode === 'synced'
                ? t('stacks.newSynced.description')
                : t('stacks.newPage.description')
        }
      />
      {!adopting && <TargetPicker owner={owner} />}
      {adopting ? (
        <Alert>
          <AlertTitle>{t('stacks.newPage.sameDefinition.title')}</AlertTitle>
          <AlertDescription>{t('stacks.newPage.sameDefinition.description')}</AlertDescription>
        </Alert>
      ) : (
        <ToggleGroup
          type="single"
          variant="outline"
          value={mode}
          onValueChange={(m) => m && setMode(m as Mode)}
          aria-label={t('stacks.newPage.source')}
          className="justify-start"
        >
          <ToggleGroupItem value="managed">{t('stacks.newPage.writeCompose')}</ToggleGroupItem>
          <ToggleGroupItem value="linked">{t('stacks.newPage.useFolder')}</ToggleGroupItem>
          <ToggleGroupItem value="synced">{t('stacks.newPage.useRepository')}</ToggleGroupItem>
        </ToggleGroup>
      )}
      {mode === 'linked' && !adopting ? (
        <NewLinkedStack owner={owner} />
      ) : mode === 'synced' && !adopting ? (
        <NewSyncedStack owner={owner} />
      ) : (
        <NewManagedStack owner={owner} adopting={adopting} />
      )}
    </div>
  );
}

/** A new Stack for the selected Host, or for the Swarm `?swarm=` names. */
export function NewStackPage() {
  const [params] = useSearchParams();
  const swarm = params.get('swarm');
  if (swarm)
    return (
      <SwarmGate key={swarm} swarmId={swarm}>
        {(s) => <NewStack owner={{ kind: 'swarm', id: s.id }} />}
      </SwarmGate>
    );
  return (
    <HostGate>
      {(hostId) => <NewStack key={hostId} owner={{ kind: 'host', id: hostId }} />}
    </HostGate>
  );
}
