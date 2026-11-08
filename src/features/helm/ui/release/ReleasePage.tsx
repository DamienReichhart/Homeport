import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useParams } from 'react-router-dom';
import { ClusterGate, ResourceTabs } from '@/core/cluster';
import { ArrowUpCircleIcon, Trash2Icon, Undo2Icon } from 'lucide-react';
import type { ClusterView, ReleaseRevision } from '@/shared/api';
import { Badge, Button } from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { releasePath } from '../../model/paths';
import { HelmGate } from '../HelmGate';
import { ReleaseStatus } from '../ReleaseStatus';
import { ReleaseHistory } from './ReleaseHistory';
import { RevisionDetails, type DetailPart } from './RevisionDetails';
import { RevisionPicker } from './RevisionPicker';
import { useReleaseChanges } from './useReleaseChanges';
import { t } from '@/shared/i18n';

const latest = (history: ReleaseRevision[]) => history[history.length - 1];

function Release({ cluster }: { cluster: ClusterView }) {
  const { namespace = '', name = '' } = useParams();
  const id = cluster.cluster.id;
  const history = useQuery(helmQueries.history(id, namespace, name));
  const [picked, setPicked] = useState<number | null>(null);
  const current = history.data ? latest(history.data) : undefined;
  const changes = useReleaseChanges(cluster, namespace, name, current);
  const part = (h: ReleaseRevision[], value: DetailPart) => {
    const current = latest(h)?.revision ?? 1;
    const revision = picked !== null && h.some((r) => r.revision === picked) ? picked : current;
    return (
      <RevisionDetails
        clusterId={id}
        namespace={namespace}
        name={name}
        revision={revision}
        part={value}
        picker={<RevisionPicker history={h} value={revision} onChange={setPicked} />}
      />
    );
  };
  return (
    <>
      <ResourceTabs
        detail={history}
        base={releasePath(namespace, name)}
        title={
          <span className="flex items-center gap-2">
            {name} <Badge variant="outline">{t('helm.release')}</Badge>
          </span>
        }
        actions={() => (
          <>
            <Button variant="outline" onClick={changes.upgrade}>
              <ArrowUpCircleIcon /> {t('helm.upgrade.open')}
            </Button>
            <Button variant="outline" onClick={changes.uninstall}>
              <Trash2Icon /> {t('helm.uninstall.open')}
            </Button>
          </>
        )}
        description={(h) => {
          const last = latest(h);
          return (
            last && (
              <span className="inline-flex flex-wrap items-center gap-3">
                <ReleaseStatus status={last.status} />
                <span>{t('helm.detail.chart', { chart: last.chart })}</span>
                <span>{t('helm.detail.namespace', { namespace })}</span>
                <span>{t('helm.detail.revision', { revision: last.revision })}</span>
              </span>
            )
          );
        }}
        tabs={(h) => [
          {
            value: 'history',
            label: t('helm.tab.history'),
            content: (
              <ReleaseHistory
                history={h}
                action={(r) =>
                  r.revision !== latest(h)?.revision && (
                    <Button size="sm" variant="ghost" onClick={() => changes.rollback(r)}>
                      <Undo2Icon /> {t('helm.rollback.open', { revision: r.revision })}
                    </Button>
                  )
                }
              />
            ),
          },
          { value: 'values', label: t('helm.tab.values'), content: part(h, 'values') },
          { value: 'notes', label: t('helm.tab.notes'), content: part(h, 'notes') },
          { value: 'manifest', label: t('helm.tab.manifest'), content: part(h, 'manifest') },
          { value: 'resources', label: t('helm.tab.resources'), content: part(h, 'resources') },
        ]}
      />
      {changes.dialogs}
    </>
  );
}

export function ReleasePage() {
  return (
    <ClusterGate>
      {(cluster) => (
        <HelmGate>
          <Release key={cluster.cluster.id} cluster={cluster} />
        </HelmGate>
      )}
    </ClusterGate>
  );
}
