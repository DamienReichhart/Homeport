import { useQuery } from '@tanstack/react-query';
import { PlusIcon, SlidersHorizontalIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { usePageQuery } from '@/core/feature';
import { IncusGate } from '@/core/host';
import type { IncusProfile } from '@/shared/api';
import {
  Button,
  DataTable,
  EmptyState,
  ErrorMessage,
  PageHeader,
  SearchInput,
  Spinner,
  Tooltip,
  TooltipContent,
  TooltipTrigger,
  useConfirmedAction,
  WithShortcut,
  type ColumnDef,
} from '@/shared/ui';
import { useDeleteProfile } from '../api/mutations';
import { incusProfileQueries } from '../api/queries';
import { undeletable } from '../model/profiles';
import { NewProfileDialog } from './NewProfileDialog';
import { PAGE_ACTION_KEYS, usePageAction } from '@/shared/hooks';
import { t } from '@/shared/i18n';

function ProfilesList({ hostId, project }: { hostId: string; project: string }) {
  const profiles = useQuery(incusProfileQueries.list(hostId, project));
  const remove = useDeleteProfile(hostId, project);
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirmedAction();
  const [query, setQuery] = usePageQuery();
  const [creating, setCreating] = useState(false);
  usePageAction(t('incus.profiles.new'), () => setCreating(true));

  if (profiles.isPending) return <Spinner />;
  if (profiles.error)
    return <ErrorMessage error={profiles.error} onRetry={() => void profiles.refetch()} />;

  const columns: ColumnDef<IncusProfile, unknown>[] = [
    {
      id: 'name',
      header: t('incus.profiles.column.name'),
      accessorFn: (p) => p.name,
      cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
    },
    {
      id: 'description',
      header: t('incus.profiles.column.description'),
      accessorFn: (p) => p.description,
      cell: ({ row }) => (
        <span
          className="text-muted-foreground block max-w-80 truncate"
          title={row.original.description || undefined}
        >
          {row.original.description || '—'}
        </span>
      ),
    },
    {
      id: 'settings',
      header: t('incus.profiles.column.settings'),
      accessorFn: (p) => Object.keys(p.config).length + Object.keys(p.devices).length,
      cell: ({ row }) => (
        <span className="text-muted-foreground whitespace-nowrap">
          {t('incus.profiles.settings', {
            config: Object.keys(row.original.config).length,
            devices: Object.keys(row.original.devices).length,
          })}
        </span>
      ),
    },
    {
      id: 'usedBy',
      header: t('incus.profiles.column.usedBy'),
      accessorFn: (p) => p.usedBy.join(' '),
      cell: ({ row }) => (
        <span
          className="text-muted-foreground block max-w-64 truncate text-xs"
          title={row.original.usedBy.join('\n') || undefined}
        >
          {row.original.usedBy.join(', ') || '—'}
        </span>
      ),
    },
    {
      id: 'actions',
      header: '',
      enableSorting: false,
      cell: ({ row }) => {
        const profile = row.original;
        const why = undeletable(profile);
        return (
          <div className="flex justify-end" onClick={(e) => e.stopPropagation()}>
            <Tooltip>
              <TooltipTrigger asChild>
                {/* A disabled button shows no tooltip; its wrapper does. */}
                <span tabIndex={why ? 0 : -1}>
                  <Button
                    variant="ghost"
                    size="icon-sm"
                    aria-label={t('incus.profiles.deleteOne', { name: profile.name })}
                    disabled={why !== null || remove.isPending}
                    onClick={() =>
                      confirm({
                        title: t('incus.profiles.delete.title', { name: profile.name }),
                        description: t('incus.profiles.delete.description', { project }),
                        confirmLabel: t('incus.delete'),
                        destructive: true,
                        action: () => remove.mutateAsync(profile.name),
                      })
                    }
                  >
                    <Trash2Icon />
                  </Button>
                </span>
              </TooltipTrigger>
              <TooltipContent>
                {why ?? t('incus.profiles.deleteOne', { name: profile.name })}
              </TooltipContent>
            </Tooltip>
          </div>
        );
      },
    },
  ];

  return (
    <div>
      <PageHeader
        title={t('incus.profiles.title')}
        description={t('incus.profiles.summary', { count: profiles.data.length, project })}
        actions={
          <WithShortcut keys={PAGE_ACTION_KEYS} label={t('incus.profiles.new')}>
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> {t('incus.profiles.new')}
            </Button>
          </WithShortcut>
        }
      />
      {profiles.data.length > 0 && (
        <div className="mb-3">
          <SearchInput value={query} onChange={setQuery} placeholder={t('incus.profiles.search')} />
        </div>
      )}
      {profiles.data.length === 0 ? (
        <EmptyState
          icon={SlidersHorizontalIcon}
          title={t('incus.profiles.empty', { project })}
          action={
            <Button onClick={() => setCreating(true)}>
              <PlusIcon /> {t('incus.profiles.new')}
            </Button>
          }
        />
      ) : (
        <DataTable
          columns={columns}
          data={profiles.data}
          getRowId={(p) => p.name}
          filter={query}
          onRowClick={(p) => void navigate(`/incus/profiles/${encodeURIComponent(p.name)}`)}
        />
      )}
      <NewProfileDialog
        hostId={hostId}
        project={project}
        open={creating}
        onClose={() => setCreating(false)}
      />
      {dialog}
    </div>
  );
}

export function ProfilesPage() {
  return (
    <IncusGate>
      {(hostId, project) => (
        <ProfilesList key={`${hostId}/${project}`} hostId={hostId} project={project} />
      )}
    </IncusGate>
  );
}
