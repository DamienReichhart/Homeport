import { useQuery } from '@tanstack/react-query';
import { useMemo, useState } from 'react';
import { LogViewer } from '@/features/containers';
import {
  ErrorMessage,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
} from '@/shared/ui';
import { podQueries } from '../api/queries';
import { podLogs } from '../api/streams';
import { ContainerSelect } from './detail/ContainerSelect';
import { t } from '@/shared/i18n';

const ALL = '*';

/**
 * The logs of a workload's Pods: all of them interleaved, each line
 * labelled with its Pod, or one Pod; one container of their template.
 */
export function WorkloadLogs({
  clusterId,
  namespace,
  selector,
  containers,
}: {
  clusterId: string;
  namespace: string;
  selector: string;
  containers: string[];
}) {
  const pods = useQuery(podQueries.list(clusterId, namespace, { selector }));
  const [pod, setPod] = useState(ALL);
  const [container, setContainer] = useState(containers[0] ?? '');
  const names = useMemo(() => (pods.data ?? []).map((p) => p.name).sort(), [pods.data]);
  const chosen = pod === ALL || !names.includes(pod) ? names : [pod];
  const key = chosen.join(',');
  const stream = useMemo(
    () =>
      podLogs(
        clusterId,
        namespace,
        key
          .split(',')
          .filter(Boolean)
          .map((p) => ({ pod: p, label: chosen.length > 1 ? p : null })),
        container || null,
        false,
      ),
    // `chosen` is read through `key`.
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [clusterId, namespace, key, container],
  );
  if (pods.isPending) return <Spinner />;
  if (pods.error) return <ErrorMessage error={pods.error} onRetry={() => void pods.refetch()} />;
  if (names.length === 0)
    return <p className="text-muted-foreground text-sm">{t('pods.logs.noPods')}</p>;
  return (
    <div className="h-[60vh]">
      <LogViewer
        stream={stream}
        streamKey={`${clusterId}/${namespace}/${key}/${container}`}
        streamToggles={false}
        toolbar={
          <>
            <Select value={names.includes(pod) ? pod : ALL} onValueChange={setPod}>
              <SelectTrigger className="w-56" size="sm" aria-label={t('pods.logs.pod')}>
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value={ALL}>
                  {t('pods.logs.allPods', { count: names.length })}
                </SelectItem>
                {names.map((n) => (
                  <SelectItem key={n} value={n}>
                    {n}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
            {containers.length > 1 && (
              <ContainerSelect
                containers={containers.map((name) => ({ name }))}
                value={container}
                onChange={setContainer}
              />
            )}
          </>
        }
      />
    </div>
  );
}
