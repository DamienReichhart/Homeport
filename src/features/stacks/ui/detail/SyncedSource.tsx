import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { repositoryLabel, repositoryQueries } from '@/features/repositories';
import type { SyncedStack } from '@/shared/api';
import { Card, CardContent, CardDescription, CardHeader, CardTitle, KeyValue } from '@/shared/ui';
import { shortCommit } from '../../model/newSynced';
import { SyncSettings } from './SyncSettings';
import type { StackOwner } from '@/shared/api';
import { t } from '@/shared/i18n';

/** Where a Synced Stack's compose file comes from, and which commits it has. */
export function SyncedSource({ owner, synced }: { owner: StackOwner; synced: SyncedStack }) {
  const repositories = useQuery(repositoryQueries.list());
  const repository = repositories.data?.find((r) => r.id === synced.repositoryId);
  const mono = (text: string) => <span className="font-mono text-xs">{text}</span>;
  const deployed = synced.deployedCommit;
  const fetched = synced.fetchedCommit;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('stacks.synced.source')}</CardTitle>
        <CardDescription>{t('stacks.synced.sourceDescription')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-4">
        <KeyValue
          items={[
            [
              t('stacks.synced.repository'),
              <span title={repository?.url}>
                {repository ? repositoryLabel(repository) : synced.repositoryId}
              </span>,
            ],
            [
              synced.refKind === 'tag' ? t('stacks.newSynced.tag') : t('stacks.newSynced.branch'),
              mono(synced.refName),
            ],
            [t('stacks.newSynced.path'), mono(synced.composePath)],
            [
              t('stacks.synced.deployedCommit'),
              deployed ? mono(shortCommit(deployed)) : t('stacks.synced.never'),
            ],
            ...(fetched && fetched !== deployed
              ? [
                  [t('stacks.synced.fetchedCommit'), mono(shortCommit(fetched))] as [
                    string,
                    ReactNode,
                  ],
                ]
              : []),
          ]}
        />
        <SyncSettings owner={owner} synced={synced} />
      </CardContent>
    </Card>
  );
}
