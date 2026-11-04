import { useQuery } from '@tanstack/react-query';
import { resourceQueries } from '@/core/host';
import { formatBytes } from '@/shared/lib';
import { Card, CardContent, CardHeader, CardTitle, KeyValue } from '@/shared/ui';

/** What the Engine of a Host is and runs on. */
export function EngineCard({ hostId }: { hostId: string }) {
  const info = useQuery(resourceQueries.systemInfo(hostId));
  return (
    <Card>
      <CardHeader>
        <CardTitle>Engine</CardTitle>
      </CardHeader>
      <CardContent>
        {info.data && (
          <KeyValue
            items={[
              ['Name', info.data.name],
              ['Version', info.data.serverVersion],
              ['OS', info.data.operatingSystem],
              ['Kernel', info.data.kernelVersion],
              ['Architecture', info.data.architecture],
              ['Resources', `${info.data.cpus} CPUs · ${formatBytes(info.data.memory)}`],
              ['Storage driver', info.data.storageDriver],
              ['Root directory', <span className="font-mono text-xs">{info.data.rootDir}</span>],
            ]}
          />
        )}
      </CardContent>
    </Card>
  );
}
