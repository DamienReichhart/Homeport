import { useQuery } from '@tanstack/react-query';
import { useDeferredValue, useState } from 'react';
import type { ChartVersion } from '@/shared/api';
import { Button, DataTable, ErrorMessage, SearchInput, Spinner } from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/**
 * Searches the charts of helm's repositories by name or description, at
 * their latest version; with `onPick`, each row offers to use that chart.
 */
export function ChartSearch({ onPick }: { onPick?: (chart: ChartVersion) => void }) {
  const [query, setQuery] = useState('');
  const deferred = useDeferredValue(query.trim());
  const charts = useQuery({ ...helmQueries.charts(deferred, false), enabled: deferred !== '' });
  return (
    <div className="grid gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={t('helm.charts.search')} />
      {deferred === '' ? (
        <p className="text-muted-foreground text-sm">{t('helm.charts.hint')}</p>
      ) : charts.isPending ? (
        <Spinner />
      ) : charts.error ? (
        <ErrorMessage error={charts.error} onRetry={() => void charts.refetch()} />
      ) : (
        <DataTable<ChartVersion>
          columns={[
            {
              id: 'name',
              header: t('helm.charts.name'),
              accessorFn: (c) => c.name,
              cell: ({ row }) => <span className="font-medium">{row.original.name}</span>,
            },
            { id: 'version', header: t('helm.charts.version'), accessorFn: (c) => c.version },
            {
              id: 'app',
              header: t('helm.column.appVersion'),
              accessorFn: (c) => c.appVersion || '—',
            },
            {
              id: 'description',
              header: t('helm.column.description'),
              accessorFn: (c) => c.description,
              cell: ({ row }) => (
                <span className="line-clamp-2 max-w-md text-xs">{row.original.description}</span>
              ),
            },
            ...(onPick
              ? [
                  {
                    id: 'pick',
                    header: '',
                    enableSorting: false,
                    cell: ({ row }: { row: { original: ChartVersion } }) => (
                      <Button size="sm" variant="outline" onClick={() => onPick(row.original)}>
                        {t('helm.charts.pick')}
                      </Button>
                    ),
                  },
                ]
              : []),
          ]}
          data={charts.data}
          getRowId={(c) => `${c.name}@${c.version}`}
          pageSize={50}
          empty={t('helm.charts.none')}
        />
      )}
    </div>
  );
}
