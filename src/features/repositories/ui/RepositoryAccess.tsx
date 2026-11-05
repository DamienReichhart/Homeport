import type { ReactNode } from 'react';
import type { Repository } from '@/shared/api';
import { Button, useConfirm } from '@/shared/ui';
import { useResetHostKey } from '../api/mutations';
import { repositoryParts } from '../model/repository';
import { DeployKeyCard } from './DeployKeyCard';
import { TestAccess } from './TestAccess';
import { TokenForm } from './TokenForm';
import { t, tRich } from '@/shared/i18n';

const mono = (chunks: ReactNode[]) => <span className="font-mono">{chunks}</span>;

/** How Homeport reaches a stored repository, and the way to test and change it. */
export function RepositoryAccess({ repository }: { repository: Repository }) {
  const reset = useResetHostKey();
  const [dialog, confirm] = useConfirm();
  const { host } = repositoryParts(repository.url);
  return (
    <div className="grid gap-3">
      {repository.auth === 'sshKey' && <DeployKeyCard repository={repository} />}
      {repository.auth === 'httpsToken' && <TokenForm repository={repository} onSaved={() => {}} />}
      {repository.auth === 'none' && (
        <p className="text-muted-foreground text-sm">{t('repositories.public')}</p>
      )}
      {repository.hostKeyFingerprint && (
        <div className="text-muted-foreground flex flex-wrap items-center justify-between gap-2 text-xs">
          <span>
            {tRich('repositories.hostKey.trusted', {
              host,
              fingerprint: repository.hostKeyFingerprint,
              mono,
            })}
          </span>
          <Button
            variant="ghost"
            size="sm"
            onClick={() =>
              confirm({
                title: t('repositories.hostKey.resetTitle', { host }),
                description: t('repositories.hostKey.resetDescription', { host }),
                confirmLabel: t('repositories.hostKey.reset'),
                destructive: true,
                onConfirm: () => reset.mutateAsync(repository.id),
              })
            }
          >
            {t('repositories.hostKey.reset')}
          </Button>
        </div>
      )}
      <TestAccess repository={repository} />
      {dialog}
    </div>
  );
}
