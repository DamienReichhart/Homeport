import { useMemo, useState } from 'react';
import { LogViewer } from '@/features/containers';
import type { PodDetail } from '@/shared/api';
import { Toggle } from '@/shared/ui';
import { podLogs } from '../../api/streams';
import { defaultContainer, hasPrevious } from '../../model/containers';
import { ContainerSelect } from './ContainerSelect';
import { t } from '@/shared/i18n';

/** A Pod's logs: one container at a time, or the instance before its last restart. */
export function PodLogs({ clusterId, pod }: { clusterId: string; pod: PodDetail }) {
  const { namespace, name } = pod.view;
  const [container, setContainer] = useState(() => defaultContainer(pod));
  const [previous, setPrevious] = useState(false);
  const restarted = hasPrevious(pod.containers.find((c) => c.name === container));
  const showPrevious = previous && restarted;
  const stream = useMemo(
    () => podLogs(clusterId, namespace, [{ pod: name, label: null }], container, showPrevious),
    [clusterId, namespace, name, container, showPrevious],
  );
  return (
    <div className="h-[60vh]">
      <LogViewer
        stream={stream}
        streamKey={`${clusterId}/${namespace}/${name}/${container}/${String(showPrevious)}`}
        streamToggles={false}
        toolbar={
          <>
            {pod.containers.length > 1 && (
              <ContainerSelect
                containers={pod.containers}
                value={container}
                onChange={setContainer}
              />
            )}
            <Toggle
              size="sm"
              variant="outline"
              pressed={showPrevious}
              onPressedChange={setPrevious}
              disabled={!restarted}
              title={restarted ? undefined : t('pods.logs.noRestart')}
            >
              {t('pods.logs.previous')}
            </Toggle>
          </>
        }
      />
    </div>
  );
}
