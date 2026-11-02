import { useEffect, useState } from 'react';
import type { AlignedData } from 'uplot';
import { commands, subscribe, type StatsSample } from '@/shared/api';
import { formatBytes, formatPercent } from '@/shared/lib';
import { Card, CardContent, CardHeader, CardTitle } from '@/shared/ui/card';
import { TimeChart } from '@/shared/ui';

/** Five minutes of samples at about one per second. */
const WINDOW = 300;

interface Point {
  time: number;
  sample: StatsSample;
}

/** Turns cumulative byte counters into per-second rates between samples. */
export function rates(points: Point[], pick: (s: StatsSample) => number): (number | null)[] {
  return points.map((p, i) => {
    const prev = points[i - 1];
    if (!prev) return null;
    const dt = p.time - prev.time;
    const delta = pick(p.sample) - pick(prev.sample);
    return dt > 0 && delta >= 0 ? delta / dt : null;
  });
}

export function StatsTab({
  hostId,
  containerId,
  running,
}: {
  hostId: string;
  containerId: string;
  running: boolean;
}) {
  const [points, setPoints] = useState<Point[]>([]);

  useEffect(() => {
    if (!running) return;
    return subscribe<StatsSample>(
      (ch) => commands.statsSubscribe(hostId, containerId, ch),
      (sample) =>
        setPoints((list) => [...list.slice(-(WINDOW - 1)), { time: Date.now() / 1000, sample }]),
    );
  }, [hostId, containerId, running]);

  if (!running)
    return (
      <p className="text-muted-foreground text-sm">
        Start the container to see its resource usage.
      </p>
    );
  const last = points.at(-1)?.sample;
  const times = points.map((p) => p.time);
  const cpu: AlignedData = [times, points.map((p) => p.sample.cpuPercent ?? 0)];
  const memory: AlignedData = [times, points.map((p) => p.sample.memoryUsage)];
  const network: AlignedData = [
    times,
    rates(points, (s) => s.netRx),
    rates(points, (s) => s.netTx),
  ];
  const disk: AlignedData = [
    times,
    rates(points, (s) => s.blockRead),
    rates(points, (s) => s.blockWrite),
  ];

  return (
    <div className="grid gap-4 lg:grid-cols-2">
      <Card>
        <CardHeader>
          <CardTitle>
            CPU{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                · {formatPercent(last.cpuPercent ?? 0)}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={cpu}
            series={[{ label: 'CPU', colour: 'var(--chart-1)' }]}
            format={formatPercent}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            Memory{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                · {formatBytes(last.memoryUsage)} of {formatBytes(last.memoryLimit)}
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={memory}
            series={[{ label: 'Used', colour: 'var(--chart-2)' }]}
            format={(v) => formatBytes(v)}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            Network{' '}
            {last && (
              <span className="text-muted-foreground font-normal">
                · {formatBytes(last.netRx)} in, {formatBytes(last.netTx)} out
              </span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={network}
            series={[
              { label: 'Received', colour: 'var(--chart-3)' },
              { label: 'Sent', colour: 'var(--chart-4)' },
            ]}
            format={(v) => `${formatBytes(v)}/s`}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>
            Disk I/O{' '}
            {last && (
              <span className="text-muted-foreground font-normal">· {last.pids} processes</span>
            )}
          </CardTitle>
        </CardHeader>
        <CardContent>
          <TimeChart
            data={disk}
            series={[
              { label: 'Read', colour: 'var(--chart-5)' },
              { label: 'Written', colour: 'var(--chart-1)' },
            ]}
            format={(v) => `${formatBytes(v)}/s`}
          />
        </CardContent>
      </Card>
    </div>
  );
}
