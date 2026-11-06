import { useState } from 'react';
import { useSearchParams } from 'react-router-dom';
import { HostGate } from '@/core/host';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  PageHeader,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { NewLinkedStack } from './NewLinkedStack';
import { NewManagedStack } from './NewManagedStack';
import { NewSyncedStack } from './NewSyncedStack';
import { t } from '@/shared/i18n';

type Mode = 'managed' | 'linked' | 'synced';

function NewStack({ hostId }: { hostId: string }) {
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
        <NewLinkedStack hostId={hostId} />
      ) : mode === 'synced' && !adopting ? (
        <NewSyncedStack hostId={hostId} />
      ) : (
        <NewManagedStack hostId={hostId} adopting={adopting} />
      )}
    </div>
  );
}

export function NewStackPage() {
  return <HostGate>{(hostId) => <NewStack key={hostId} hostId={hostId} />}</HostGate>;
}
