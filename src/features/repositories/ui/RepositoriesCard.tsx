import { useQuery } from '@tanstack/react-query';
import { PlusIcon, SettingsIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import {
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Spinner,
  useConfirm,
} from '@/shared/ui';
import { useRemoveRepository } from '../api/mutations';
import { repositoryQueries } from '../api/queries';
import { describeAuth, repositoryLabel } from '../model/repository';
import { RepositoryDialog } from './RepositoryDialog';
import { t } from '@/shared/i18n';

/** The Repositories card in Settings: Synced Stacks' git repositories. */
export function RepositoriesCard() {
  const repositories = useQuery(repositoryQueries.list());
  const remove = useRemoveRepository();
  const [dialog, confirm] = useConfirm();
  const [open, setOpen] = useState<string | 'new' | null>(null);

  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('repositories.title')}</CardTitle>
        <CardDescription>{t('repositories.description')}</CardDescription>
        <CardAction>
          <Button variant="outline" size="sm" onClick={() => setOpen('new')}>
            <PlusIcon /> {t('repositories.add')}
          </Button>
        </CardAction>
      </CardHeader>
      <CardContent className="grid gap-3">
        {repositories.isPending && <Spinner />}
        {repositories.error && (
          <ErrorMessage error={repositories.error} onRetry={() => void repositories.refetch()} />
        )}
        {repositories.data?.length === 0 && (
          <p className="text-muted-foreground text-sm">{t('repositories.empty')}</p>
        )}
        {repositories.data && repositories.data.length > 0 && (
          <ul className="divide-border divide-y">
            {repositories.data.map((r) => (
              <li key={r.id} className="flex items-center gap-2 py-2">
                <div className="min-w-0 flex-1">
                  <p className="truncate font-medium">{repositoryLabel(r)}</p>
                  <p className="text-muted-foreground truncate text-xs">{describeAuth(r)}</p>
                </div>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('repositories.manageOne', { repository: repositoryLabel(r) })}
                  onClick={() => setOpen(r.id)}
                >
                  <SettingsIcon />
                </Button>
                <Button
                  variant="ghost"
                  size="icon-sm"
                  aria-label={t('repositories.removeOne', { repository: repositoryLabel(r) })}
                  onClick={() =>
                    confirm({
                      title: t('repositories.remove.title', { repository: repositoryLabel(r) }),
                      description: t('repositories.remove.description'),
                      confirmLabel: t('common.remove'),
                      destructive: true,
                      onConfirm: () => remove.mutateAsync(r),
                    })
                  }
                >
                  <Trash2Icon />
                </Button>
              </li>
            ))}
          </ul>
        )}
      </CardContent>
      {open && (
        <RepositoryDialog
          repositoryId={open === 'new' ? null : open}
          onClose={() => setOpen(null)}
        />
      )}
      {dialog}
    </Card>
  );
}
