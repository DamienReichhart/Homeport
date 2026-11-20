import { DiskUsageCard } from './DiskUsageCard';
import { EngineCard } from './EngineCard';
import { HostVitals } from './HostVitals';
import { RecentEventsCard } from './RecentEventsCard';

/** The selected Host's vitals, Engine, disk usage and recent events. */
export function HostDashboard({ hostId }: { hostId: string }) {
  return (
    <div className="grid gap-4">
      <HostVitals hostId={hostId} />
      <div className="grid gap-4 lg:grid-cols-2">
        <EngineCard hostId={hostId} />
        <DiskUsageCard hostId={hostId} />
        <RecentEventsCard hostId={hostId} />
      </div>
    </div>
  );
}
