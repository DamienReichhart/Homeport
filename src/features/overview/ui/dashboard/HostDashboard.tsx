import { DiskUsageCard } from './DiskUsageCard';
import { EngineCard } from './EngineCard';
import { RecentEventsCard } from './RecentEventsCard';

/** The selected Host's Engine, disk usage and recent events. */
export function HostDashboard({ hostId }: { hostId: string }) {
  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <EngineCard hostId={hostId} />
      <DiskUsageCard hostId={hostId} />
      <RecentEventsCard hostId={hostId} />
    </div>
  );
}
