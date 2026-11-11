import { DatabaseIcon, FolderOpenIcon } from 'lucide-react';
import { commands, type StartupProblem } from '@/shared/api';
import { Button, CopyButton, EmptyState, ErrorMessage, toastError } from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * Shown instead of the app when its database couldn't be opened: what went
 * wrong, where the file is, and a way to its folder. Nothing else runs, so
 * nothing here needs the database.
 */
export function StartupProblemPage({ problem }: { problem: StartupProblem }) {
  return (
    <main className="flex h-screen items-center justify-center overflow-auto p-6">
      <EmptyState
        icon={DatabaseIcon}
        title={t('app.startup.title')}
        description={
          <div className="grid max-w-xl gap-3 text-start">
            <ErrorMessage error={problem.error} />
            <p>{t('app.startup.description')}</p>
            <div className="flex items-center gap-2">
              <code className="bg-muted min-w-0 rounded px-1.5 py-0.5 font-mono text-xs break-all">
                {problem.databasePath}
              </code>
              <CopyButton text={problem.databasePath} />
            </div>
          </div>
        }
        action={
          <Button onClick={() => commands.openDataFolder().catch(toastError)}>
            <FolderOpenIcon /> {t('app.startup.openFolder')}
          </Button>
        }
      />
    </main>
  );
}
