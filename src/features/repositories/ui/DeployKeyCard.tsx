import type { ReactNode } from 'react';
import type { Repository } from '@/shared/api';
import { Button, CopyButton, useConfirm } from '@/shared/ui';
import { useRotateKey } from '../api/mutations';
import { t, tRich } from '@/shared/i18n';

const mono = (chunks: ReactNode[]) => <span className="font-mono">{chunks}</span>;
const strong = (chunks: ReactNode[]) => <span className="font-medium">{chunks}</span>;

/** A repository's deploy key, and where to add it on GitHub, GitLab, Gitea and Forgejo. */
export function DeployKeyCard({ repository }: { repository: Repository }) {
  const rotate = useRotateKey();
  const [dialog, confirm] = useConfirm();
  if (!repository.publicKey) return null;
  return (
    <div className="grid gap-3 rounded-md border p-3">
      <div className="grid gap-1.5">
        <div className="flex items-center justify-between">
          <span className="text-sm font-medium">{t('repositories.deployKey.title')}</span>
          <CopyButton text={repository.publicKey} />
        </div>
        <pre className="bg-muted/50 overflow-x-auto rounded-md border p-2 font-mono text-xs break-all whitespace-pre-wrap">
          {repository.publicKey}
        </pre>
      </div>
      <ul className="text-muted-foreground grid list-inside list-disc gap-1 text-xs">
        <li>{tRich('repositories.deployKey.github', { strong })}</li>
        <li>{tRich('repositories.deployKey.gitlab', { strong })}</li>
        <li>{tRich('repositories.deployKey.gitea', { strong })}</li>
      </ul>
      <p className="text-muted-foreground text-xs">{t('repositories.deployKey.readOnly')}</p>
      <div className="text-muted-foreground flex flex-wrap items-center justify-between gap-2 text-xs">
        <span>
          {tRich('repositories.deployKey.fingerprint', {
            fingerprint: repository.keyFingerprint ?? '',
            mono,
          })}
        </span>
        <Button
          variant="outline"
          size="sm"
          onClick={() =>
            confirm({
              title: t('repositories.rotate.title'),
              description: t('repositories.rotate.description'),
              confirmLabel: t('repositories.rotate.confirm'),
              onConfirm: () => rotate.mutateAsync(repository.id),
            })
          }
        >
          {t('repositories.rotate.button')}
        </Button>
      </div>
      {dialog}
    </div>
  );
}
