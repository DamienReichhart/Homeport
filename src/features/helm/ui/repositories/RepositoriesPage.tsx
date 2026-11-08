import { useQuery } from '@tanstack/react-query';
import { PlusIcon, RefreshCwIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { ClusterGate } from '@/core/cluster';
import type { ChartRepository } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  DataTable,
  ErrorMessage,
  Spinner,
  useConfirmedAction,
} from '@/shared/ui';
import { useRemoveRepository } from '../../api/mutations';
import { helmQueries } from '../../api/queries';
import { useUpdateRepositories } from '../../api/streams';
import { HelmGate } from '../HelmGate';
import { HelmPageHeader } from '../HelmPageHeader';
import { HelmRunDialog } from '../run/HelmRunDialog';
import { AddRepositoryDialog } from './AddRepositoryDialog';
import { ChartSearch } from './ChartSearch';
import { t } from '@/shared/i18n';

function Repositories() {
  const list = useQuery(helmQueries.repositories());
  const remove = useRemoveRepository();
  const update = useUpdateRepositories();
  const [adding, setAdding] = useState(false);
  const [updating, setUpdating] = useState<string[] | null>(null);
  const [confirmDialog, confirm] = useConfirmedAction();
  return (
    <div className="grid gap-4">
      <HelmPageHeader
        current="/helm/repositories"
        toolbar={
          <>
            <Button size="sm" onClick={() => setAdding(true)}>
              <PlusIcon /> {t('helm.repos.add')}
            </Button>
            <Button
              size="sm"
              variant="outline"
              disabled={!list.data?.length}
              onClick={() => setUpdating([])}
            >
              <RefreshCwIcon /> {t('helm.repos.updateAll')}
            </Button>
          </>
        }
      />
      <p className="text-muted-foreground text-sm">{t('helm.repos.description')}</p>
      {list.isPending ? (
        <Spinner />
      ) : list.error ? (
        <ErrorMessage error={list.error} onRetry={() => void list.refetch()} />
      ) : (
        <DataTable<ChartRepository>
          columns={[
            {
              id: 'name',
              header: t('helm.repos.name'),
              accessorFn: (r) => r.name,
              cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
            },
            {
              id: 'url',
              header: t('helm.repos.url'),
              accessorFn: (r) => r.url,
              cell: ({ row }) => <span className="font-mono text-xs">{row.original.url}</span>,
            },
            {
              id: 'actions',
              header: '',
              enableSorting: false,
              cell: ({ row }) => (
                <div className="flex justify-end gap-1">
                  <Button
                    size="sm"
                    variant="ghost"
                    onClick={() => setUpdating([row.original.name])}
                  >
                    <RefreshCwIcon /> {t('helm.repos.update')}
                  </Button>
                  <Button
                    size="sm"
                    variant="ghost"
                    aria-label={t('helm.repos.removeNamed', { name: row.original.name })}
                    onClick={() =>
                      confirm({
                        title: t('helm.repos.removeTitle', { name: row.original.name }),
                        description: t('helm.repos.removeDescription', {
                          name: row.original.name,
                          url: row.original.url,
                        }),
                        confirmLabel: t('helm.repos.remove'),
                        destructive: true,
                        action: () => remove.mutateAsync(row.original.name),
                      })
                    }
                  >
                    <Trash2Icon />
                  </Button>
                </div>
              ),
            },
          ]}
          data={list.data}
          getRowId={(r) => r.name}
          empty={t('helm.repos.empty')}
        />
      )}
      <Card>
        <CardHeader>
          <CardTitle>{t('helm.charts.title')}</CardTitle>
          <CardDescription>{t('helm.charts.description')}</CardDescription>
        </CardHeader>
        <CardContent>
          <ChartSearch />
        </CardContent>
      </Card>
      {adding && <AddRepositoryDialog onClose={() => setAdding(false)} />}
      {updating && (
        <HelmRunDialog
          title={
            updating.length === 0
              ? t('helm.repos.updatingAll')
              : t('helm.repos.updating', { name: updating.join(', ') })
          }
          run={update(updating)}
          onClose={() => setUpdating(null)}
        />
      )}
      {confirmDialog}
    </div>
  );
}

/** helm's chart repositories on this machine, shared by every Cluster. */
export function RepositoriesPage() {
  return (
    <ClusterGate>
      {() => (
        <HelmGate>
          <Repositories />
        </HelmGate>
      )}
    </ClusterGate>
  );
}
