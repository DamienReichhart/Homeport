import { useQuery } from '@tanstack/react-query';
import { useState, type ReactNode } from 'react';
import type { ReleaseDetails } from '@/shared/api';
import { ErrorMessage, Spinner, ToggleGroup, ToggleGroupItem } from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { ReleaseResources } from './ReleaseResources';
import { TextBlock } from './TextBlock';
import { t } from '@/shared/i18n';

export type DetailPart = 'values' | 'notes' | 'manifest' | 'resources';

function Values({ details }: { details: ReleaseDetails }) {
  const [which, setWhich] = useState<'user' | 'computed'>('user');
  return (
    <div className="grid gap-3">
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        value={which}
        onValueChange={(v) => v && setWhich(v as 'user' | 'computed')}
        className="justify-self-start"
      >
        <ToggleGroupItem value="user">{t('helm.values.user')}</ToggleGroupItem>
        <ToggleGroupItem value="computed">{t('helm.values.computed')}</ToggleGroupItem>
      </ToggleGroup>
      <TextBlock
        text={which === 'user' ? details.userValues : details.computedValues}
        label={t('helm.values.label')}
        empty={t('helm.values.none')}
      />
    </div>
  );
}

/** One part of what a revision of a release was installed with, under `picker`. */
export function RevisionDetails({
  clusterId,
  namespace,
  name,
  revision,
  part,
  picker,
}: {
  clusterId: string;
  namespace: string;
  name: string;
  revision: number;
  part: DetailPart;
  picker: ReactNode;
}) {
  const details = useQuery(helmQueries.details(clusterId, namespace, name, revision));
  let content: ReactNode;
  if (details.isPending) content = <Spinner />;
  else if (details.error)
    content = <ErrorMessage error={details.error} onRetry={() => void details.refetch()} />;
  else if (part === 'values') content = <Values details={details.data} />;
  else if (part === 'notes')
    content = (
      <TextBlock
        text={details.data.notes}
        label={t('helm.notes.label')}
        empty={t('helm.notes.none')}
      />
    );
  else if (part === 'manifest')
    content = (
      <TextBlock
        text={details.data.manifest}
        label={t('helm.manifest.label')}
        empty={t('helm.manifest.none')}
      />
    );
  else
    content = (
      <ReleaseResources
        clusterId={clusterId}
        namespace={namespace}
        resources={details.data.resources}
      />
    );
  return (
    <div className="grid gap-3">
      <div>{picker}</div>
      {content}
    </div>
  );
}
