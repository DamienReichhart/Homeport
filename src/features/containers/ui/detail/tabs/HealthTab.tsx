import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { HealthResult } from '@/shared/api';
import { cn, formatDateTime } from '@/shared/lib';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  Spinner,
} from '@/shared/ui';
import { containerQueries } from '../../../api/queries';
import { checkDuration, healthTiming, OUTPUT_LINES } from '../../../model/health';

function Output({ text }: { text: string }) {
  const [open, setOpen] = useState(false);
  const lines = text.trimEnd().split('\n');
  const long = lines.length > OUTPUT_LINES;
  if (text.trim() === '') return <p className="text-muted-foreground text-xs">No output.</p>;
  return (
    <div className="grid gap-1">
      <pre className="bg-muted overflow-x-auto rounded-md p-2 font-mono text-xs whitespace-pre-wrap">
        {long && !open ? lines.slice(0, OUTPUT_LINES).join('\n') : text.trimEnd()}
      </pre>
      {long && (
        <Button
          variant="ghost"
          size="sm"
          className="justify-self-start"
          onClick={() => setOpen(!open)}
        >
          {open ? 'Show less' : `Show ${lines.length - OUTPUT_LINES} more lines`}
        </Button>
      )}
    </div>
  );
}

function Result({ result }: { result: HealthResult }) {
  const passed = result.exitCode === 0;
  const took = checkDuration(result.start, result.end);
  return (
    <li className="grid gap-1.5 border-b pb-3 last:border-b-0 last:pb-0">
      <div className="flex flex-wrap items-center gap-2 text-sm">
        <Badge
          variant="outline"
          className={cn(passed ? 'text-success' : 'border-destructive text-destructive')}
        >
          exit {result.exitCode}
        </Badge>
        <span>{formatDateTime(Date.parse(result.start) / 1000)}</span>
        {took && <span className="text-muted-foreground">· took {took}</span>}
      </div>
      <Output text={result.output} />
    </li>
  );
}

/** A container's healthcheck: its status, configuration and last results. */
export function HealthTab({
  hostId,
  containerId,
  status,
}: {
  hostId: string;
  containerId: string;
  /** The status the list shows, so a change refreshes the tab. */
  status: string | null;
}) {
  const health = useQuery(containerQueries.health(hostId, containerId, status));
  if (health.isPending) return <Spinner />;
  if (health.error)
    return <ErrorMessage error={health.error} onRetry={() => void health.refetch()} />;
  if (!health.data)
    return <p className="text-muted-foreground text-sm">This container has no healthcheck.</p>;
  const { config, log } = health.data;
  const unhealthy = health.data.status === 'unhealthy';

  return (
    <div className="grid gap-4 lg:grid-cols-[minmax(0,1fr)_minmax(0,2fr)]">
      <Card>
        <CardHeader>
          <CardTitle className="flex items-center gap-2">
            Healthcheck
            <Badge
              variant={unhealthy ? 'destructive' : 'outline'}
              className={cn(health.data.status === 'healthy' && 'text-success')}
            >
              {health.data.status}
            </Badge>
          </CardTitle>
        </CardHeader>
        <CardContent>
          <KeyValue
            items={[
              ['Command', <span className="font-mono text-xs break-all">{config.command}</span>],
              ['Timing', healthTiming(config)],
              ['Failing in a row', String(health.data.failingStreak)],
            ]}
          />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>Last results</CardTitle>
        </CardHeader>
        <CardContent>
          {log.length === 0 ? (
            <p className="text-muted-foreground text-sm">No check has run yet.</p>
          ) : (
            <ol className="grid gap-3">
              {log.map((result) => (
                <Result key={result.start} result={result} />
              ))}
            </ol>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
