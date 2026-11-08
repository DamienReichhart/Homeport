import { useState, type ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import type { ClusterView, ReleaseRevision } from '@/shared/api';
import { useRollback, useUninstall, useUpgrade, type HelmRun } from '../../api/streams';
import { HelmRunDialog } from '../run/HelmRunDialog';
import { RollbackDialog } from './RollbackDialog';
import { UninstallDialog } from './UninstallDialog';
import { UpgradeDialog } from './UpgradeDialog';
import { t } from '@/shared/i18n';

type Asking =
  { kind: 'upgrade' } | { kind: 'uninstall' } | { kind: 'rollback'; target: ReleaseRevision };

interface Running {
  title: string;
  run: HelmRun;
  /** Where to go once it succeeded. */
  after?: string;
}

/**
 * Upgrading, rolling back and uninstalling the release `name`: each asks
 * first in its dialog, then streams helm's output. Returns what opens
 * them, and the dialogs to render.
 */
export function useReleaseChanges(
  cluster: ClusterView,
  namespace: string,
  name: string,
  current: ReleaseRevision | undefined,
): {
  upgrade: () => void;
  uninstall: () => void;
  rollback: (target: ReleaseRevision) => void;
  dialogs: ReactNode;
} {
  const id = cluster.cluster.id;
  const clusterName = cluster.cluster.name;
  const navigate = useNavigate();
  const upgrade = useUpgrade(id);
  const rollback = useRollback(id);
  const uninstall = useUninstall(id);
  const [asking, setAsking] = useState<Asking | null>(null);
  const [running, setRunning] = useState<Running | null>(null);
  const start = (next: Running) => {
    setAsking(null);
    setRunning(next);
  };
  const dialogs = (
    <>
      {current && asking?.kind === 'upgrade' && (
        <UpgradeDialog
          clusterId={id}
          clusterName={clusterName}
          namespace={namespace}
          name={name}
          current={current}
          onUpgrade={(request) =>
            start({
              title: t('helm.upgrade.running', { name, cluster: clusterName }),
              run: upgrade(request),
            })
          }
          onClose={() => setAsking(null)}
        />
      )}
      {current && asking?.kind === 'rollback' && (
        <RollbackDialog
          clusterName={clusterName}
          namespace={namespace}
          name={name}
          current={current}
          target={asking.target}
          onConfirm={(request) =>
            start({
              title: t('helm.rollback.running', { name, revision: request.revision }),
              run: rollback(request),
            })
          }
          onClose={() => setAsking(null)}
        />
      )}
      {current && asking?.kind === 'uninstall' && (
        <UninstallDialog
          clusterId={id}
          clusterName={clusterName}
          namespace={namespace}
          name={name}
          revision={current.revision}
          onConfirm={(request) =>
            start({
              title: t('helm.uninstall.running', { name, cluster: clusterName }),
              run: uninstall(request),
              after: '/helm',
            })
          }
          onClose={() => setAsking(null)}
        />
      )}
      {running && (
        <HelmRunDialog
          title={running.title}
          run={running.run}
          onClose={() => {
            setRunning(null);
          }}
          doneAction={
            running.after
              ? {
                  label: t('helm.uninstall.backToReleases'),
                  onClick: () => void navigate(running.after ?? '/helm', { replace: true }),
                }
              : undefined
          }
        />
      )}
    </>
  );
  return {
    upgrade: () => setAsking({ kind: 'upgrade' }),
    uninstall: () => setAsking({ kind: 'uninstall' }),
    rollback: (target) => setAsking({ kind: 'rollback', target }),
    dialogs,
  };
}
