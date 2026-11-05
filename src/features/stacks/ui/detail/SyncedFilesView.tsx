import { useQuery } from '@tanstack/react-query';
import { openUrl } from '@tauri-apps/plugin-opener';
import { ExternalLinkIcon, FileIcon } from 'lucide-react';
import { forgeFileUrl } from '@/features/repositories';
import type { SyncedStack } from '@/shared/api';
import { Button, ErrorMessage, Spinner, toastError } from '@/shared/ui';
import { syncedQueries } from '../../api/synced';
import { YamlEditor } from '../YamlEditor';
import { t } from '@/shared/i18n';

/**
 * A Synced Stack's compose file at its fetched commit, read-only (it can
 * still be selected and copied), and the other files of its folder.
 */
export function SyncedFilesView({ synced, url }: { synced: SyncedStack; url: string | null }) {
  const files = useQuery(syncedQueries.files(synced.id, synced.fetchedCommit));
  if (files.isPending) return <Spinner />;
  if (files.error) return <ErrorMessage error={files.error} onRetry={() => void files.refetch()} />;
  const [compose, ...others] = files.data.files;
  const link = (path: string) => (url ? forgeFileUrl(url, files.data.commit.hash, path) : null);
  const composeLink = compose ? link(compose.path) : null;
  return (
    <div className="grid gap-3">
      <div className="flex flex-wrap items-center justify-between gap-2">
        <p className="text-sm">
          {t('stacks.syncedFiles.editInGit', {
            path: synced.composePath,
            commit: files.data.commit.shortHash,
          })}
        </p>
        {composeLink && (
          <Button
            variant="outline"
            size="sm"
            onClick={() => openUrl(composeLink).catch(toastError)}
          >
            <ExternalLinkIcon /> {t('stacks.syncedFiles.openOnForge')}
          </Button>
        )}
      </div>
      {compose?.content !== null && compose !== undefined && (
        <YamlEditor
          value={compose.content ?? ''}
          readOnly
          label={t('stacks.syncedFiles.composeLabel', { path: compose.path })}
        />
      )}
      {others.length > 0 && (
        <section className="grid gap-1">
          <h3 className="text-sm font-medium">{t('stacks.syncedFiles.others')}</h3>
          <ul className="grid gap-0.5 text-sm">
            {others.map((f) => {
              const href = link(f.path);
              return (
                <li key={f.path} className="flex items-center gap-1.5">
                  <FileIcon className="text-muted-foreground size-3.5" />
                  {href ? (
                    <button
                      type="button"
                      className="font-mono text-xs hover:underline"
                      onClick={() => openUrl(href).catch(toastError)}
                    >
                      {f.path}
                    </button>
                  ) : (
                    <span className="font-mono text-xs">{f.path}</span>
                  )}
                </li>
              );
            })}
          </ul>
        </section>
      )}
    </div>
  );
}
