import { useQuery } from '@tanstack/react-query';
import { resourceQueries } from '@/core/host';
import { formatBytes } from '@/shared/lib';
import { Card, CardContent, CardHeader, CardTitle, KeyValue } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** What the Engine of a Host is and runs on. */
export function EngineCard({ hostId }: { hostId: string }) {
  const info = useQuery(resourceQueries.systemInfo(hostId));
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('overview.engine.title')}</CardTitle>
      </CardHeader>
      <CardContent>
        {info.data && (
          <KeyValue
            items={[
              [t('overview.engine.name'), info.data.name],
              [t('overview.engine.version'), info.data.serverVersion],
              [t('overview.engine.os'), info.data.operatingSystem],
              [t('overview.engine.kernel'), info.data.kernelVersion],
              [t('overview.engine.architecture'), info.data.architecture],
              [
                t('overview.engine.resources'),
                t('overview.engine.resourcesValue', {
                  cpus: info.data.cpus,
                  memory: formatBytes(info.data.memory),
                }),
              ],
              [t('overview.engine.storageDriver'), info.data.storageDriver],
              [
                t('overview.engine.rootDir'),
                <span className="font-mono text-xs">{info.data.rootDir}</span>,
              ],
            ]}
          />
        )}
      </CardContent>
    </Card>
  );
}
