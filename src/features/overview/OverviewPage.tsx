import { useQuery } from '@tanstack/react-query';
import { BrushCleaningIcon, CpuIcon, MemoryStickIcon, PlusIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate } from 'react-router-dom';
import { useUsage } from '@/features/containers';
import {
  connectionLabel,
  describeAddress,
  HostGate,
  HostStatusBadge,
  useHosts,
  useHostSelection,
} from '@/core/host';
import { commands, keys, type HostView } from '@/shared/api';
import { cn, formatBytes, formatPercent, formatRelative } from '@/shared/lib';
import {
  Badge,
  Button,
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  KeyValue,
  PageHeader,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import { PruneDialog } from './PruneDialog';

/** Live CPU and memory use of a Host's running containers against its capacity. */
function HostUsage({ hostId, cpus, memory }: { hostId: string; cpus: number; memory: number }) {
  const usage = [...useUsage(hostId).values()];
  const cpu = usage.reduce((sum, u) => sum + (u.cpuPercent ?? 0), 0);
  const used = usage.reduce((sum, u) => sum + u.memoryUsage, 0);
  return (
    <>
      <span className="flex items-center gap-1" title="Container CPU use, 100% per CPU">
        <CpuIcon className="size-3.5" /> {formatPercent(cpu)} of {cpus} CPUs
      </span>
      <span className="flex items-center gap-1" title="Container memory use">
        <MemoryStickIcon className="size-3.5" /> {formatBytes(used)} / {formatBytes(memory)}
      </span>
    </>
  );
}

function HostCard({ view, selected }: { view: HostView; selected: boolean }) {
  const { select } = useHostSelection();
  const navigate = useNavigate();
  const connected = view.status.state === 'connected';
  const info = useQuery({
    queryKey: [...keys.system(view.host.id), 'info'],
    queryFn: () => commands.systemInfo(view.host.id),
    enabled: connected,
  });
  return (
    <Card
      className={cn(
        'cursor-pointer transition-colors hover:border-primary/60',
        selected && 'border-primary',
      )}
      onClick={() => {
        select(view.host.id);
        if (connected) navigate('/containers');
      }}
    >
      <CardHeader>
        <CardTitle className="truncate">{view.host.name}</CardTitle>
        <CardDescription className="truncate font-mono text-xs">
          {describeAddress(view.host.connection)}
        </CardDescription>
        <CardAction>
          <Badge variant="outline">{connectionLabel(view.host.connection.kind)}</Badge>
        </CardAction>
      </CardHeader>
      <CardContent className="grid gap-3">
        <HostStatusBadge status={view.status} />
        {connected && info.data && (
          <div className="text-muted-foreground grid grid-cols-2 gap-2 text-sm">
            <span>
              <span className="text-foreground font-medium">{info.data.running}</span> /{' '}
              {info.data.containers} running
            </span>
            <span>{info.data.images} images</span>
            <HostUsage hostId={view.host.id} cpus={info.data.cpus} memory={info.data.memory} />
          </div>
        )}
      </CardContent>
    </Card>
  );
}

function HostDashboard({ hostId }: { hostId: string }) {
  const [pruning, setPruning] = useState(false);
  const info = useQuery({
    queryKey: [...keys.system(hostId), 'info'],
    queryFn: () => commands.systemInfo(hostId),
  });
  const usage = useQuery({
    queryKey: [...keys.system(hostId), 'df'],
    queryFn: () => commands.systemDiskUsage(hostId),
  });
  const events = useQuery({
    queryKey: keys.events(hostId),
    queryFn: () => commands.hostsEvents(hostId),
  });
  const rows = usage.data
    ? ([
        ['Images', usage.data.images],
        ['Containers', usage.data.containers],
        ['Volumes', usage.data.volumes],
        ['Build cache', usage.data.buildCache],
      ] as const)
    : [];

  return (
    <div className="grid gap-4 lg:grid-cols-2">
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
      <Card>
        <CardHeader>
          <CardTitle>Disk usage</CardTitle>
          <CardAction>
            <Button variant="outline" size="sm" onClick={() => setPruning(true)}>
              <BrushCleaningIcon /> Clean up…
            </Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>Type</TableHead>
                <TableHead className="text-right">Total</TableHead>
                <TableHead className="text-right">In use</TableHead>
                <TableHead className="text-right">Size</TableHead>
                <TableHead className="text-right">Reclaimable</TableHead>
              </TableRow>
            </TableHeader>
            <TableBody>
              {rows.map(([label, line]) => (
                <TableRow key={label}>
                  <TableCell>{label}</TableCell>
                  <TableCell className="text-right">{line.total}</TableCell>
                  <TableCell className="text-right">{line.active}</TableCell>
                  <TableCell className="text-right">{formatBytes(line.size)}</TableCell>
                  <TableCell className="text-right">{formatBytes(line.reclaimable)}</TableCell>
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </CardContent>
      </Card>
      <Card className="lg:col-span-2">
        <CardHeader>
          <CardTitle>Recent events</CardTitle>
          <CardAction>
            <Button variant="ghost" size="sm" asChild>
              <Link to="/events">All events</Link>
            </Button>
          </CardAction>
        </CardHeader>
        <CardContent>
          {events.data?.length ? (
            <ul className="grid gap-1 text-sm">
              {events.data
                .slice(-8)
                .reverse()
                .map((e, i) => (
                  <li key={`${e.time}-${i}`} className="flex gap-3">
                    <span className="text-muted-foreground w-28 shrink-0">
                      {formatRelative(e.time)}
                    </span>
                    <span className="w-20 shrink-0 capitalize">{e.kind}</span>
                    <span className="font-medium">{e.action}</span>
                    <span className="text-muted-foreground truncate">
                      {e.attributes.name ?? e.actorId.slice(0, 12)}
                    </span>
                  </li>
                ))}
            </ul>
          ) : (
            <p className="text-muted-foreground text-sm">
              Nothing has happened since Homeport connected.
            </p>
          )}
        </CardContent>
      </Card>
      <PruneDialog hostId={hostId} open={pruning} onOpenChange={setPruning} />
    </div>
  );
}

export function OverviewPage() {
  const hosts = useHosts();
  const { hostId, host } = useHostSelection();
  return (
    <div className="grid gap-6">
      <PageHeader
        title="Overview"
        actions={
          <Button asChild variant="outline">
            <Link to="/hosts/new">
              <PlusIcon /> Add host
            </Link>
          </Button>
        }
      />
      {hosts.data && hosts.data.length > 0 && (
        <div className="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
          {hosts.data.map((view) => (
            <HostCard key={view.host.id} view={view} selected={view.host.id === hostId} />
          ))}
        </div>
      )}
      <section className="grid gap-3">
        {host && <h2 className="text-lg font-semibold">{host.host.name}</h2>}
        <HostGate>{(id) => <HostDashboard hostId={id} />}</HostGate>
      </section>
    </div>
  );
}
