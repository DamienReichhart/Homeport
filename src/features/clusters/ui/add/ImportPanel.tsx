import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useClusters, useClusterSelection } from '@/core/cluster';
import type { KubeContexts } from '@/shared/api';
import { Button, EmptyState, ErrorMessage } from '@/shared/ui';
import { useImportClusters, useRefreshFromImport } from '../../api/mutations';
import { choices, initialRows, rowsProblem, type ContextRow } from '../../model/importRows';
import { ContextList } from './ContextList';
import { t } from '@/shared/i18n';

/** Picks contexts of a kubeconfig (`text`, or this machine's) and imports them. */
export function ImportPanel({ found, text }: { found: KubeContexts; text: string | null }) {
  const navigate = useNavigate();
  const clusters = useClusters();
  const { select } = useClusterSelection();
  const [rows, setRows] = useState<ContextRow[]>(() => initialRows(found.contexts));
  const refresh = useRefreshFromImport();
  const create = useImportClusters((added) => {
    const first = added[0];
    if (!first) return;
    select(first.cluster.id);
    void navigate(added.length === 1 ? `/clusters/${first.cluster.id}` : '/clusters');
  });

  if (found.contexts.length === 0) return <EmptyState title={t('clusters.import.noContexts')} />;

  const taken = (clusters.data ?? []).map((c) => c.cluster.name);
  const problem = rowsProblem(rows, taken);
  const picked = choices(rows);
  return (
    <div className="grid gap-3">
      {found.paths.length > 0 && (
        <p className="text-muted-foreground text-sm">
          {t('clusters.import.readFrom', { paths: found.paths.join(', ') })}
        </p>
      )}
      <ContextList
        rows={rows}
        onChange={setRows}
        refreshing={refresh.isPending}
        onRefresh={(clusterId) => refresh.mutate({ clusterId, text })}
      />
      {problem && problem.kind !== 'nothing' && (
        <p className="text-destructive text-sm" role="alert">
          {problem.kind === 'empty'
            ? t('clusters.import.problem.empty')
            : problem.kind === 'duplicate'
              ? t('clusters.import.problem.duplicate', { name: problem.name })
              : t('clusters.import.problem.taken', { name: problem.name })}
        </p>
      )}
      {create.error && <ErrorMessage error={create.error} />}
      <div>
        <Button
          disabled={problem !== null || create.isPending}
          onClick={() => create.mutate({ contexts: picked, text })}
        >
          {t('clusters.import.submit', { count: picked.length })}
        </Button>
      </div>
    </div>
  );
}
