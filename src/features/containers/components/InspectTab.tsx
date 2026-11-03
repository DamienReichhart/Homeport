import type { ReactNode } from 'react';
import {
  Badge,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  JsonView,
  KeyValue,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import type { ContainerInspect } from '../types';

function Section({ title, children }: { title: string; children: ReactNode }) {
  return (
    <Card>
      <CardHeader>
        <CardTitle>{title}</CardTitle>
      </CardHeader>
      <CardContent>{children}</CardContent>
    </Card>
  );
}

function SimpleTable({ head, rows }: { head: string[]; rows: ReactNode[][] }) {
  if (rows.length === 0) return <p className="text-muted-foreground text-sm">None.</p>;
  return (
    <Table>
      <TableHeader>
        <TableRow>
          {head.map((h) => (
            <TableHead key={h}>{h}</TableHead>
          ))}
        </TableRow>
      </TableHeader>
      <TableBody>
        {rows.map((r, i) => (
          <TableRow key={i}>
            {r.map((c, j) => (
              <TableCell key={j} className="max-w-md break-all whitespace-normal">
                {c}
              </TableCell>
            ))}
          </TableRow>
        ))}
      </TableBody>
    </Table>
  );
}

const mono = (s: string) => <span className="font-mono text-xs">{s}</span>;

/** The inspect document as readable sections, with the raw JSON one tab away. */
export function InspectTab({ inspect }: { inspect: ContainerInspect }) {
  const env = (inspect.Config.Env ?? []).map((e) => {
    const i = e.indexOf('=');
    return i < 0 ? [e, ''] : [e.slice(0, i), e.slice(i + 1)];
  });
  const ports = Object.entries(inspect.NetworkSettings.Ports ?? {}).flatMap(([port, bindings]) =>
    bindings?.length
      ? bindings.map((b) => [mono(port), mono(`${b.HostIp || '*'}:${b.HostPort}`)])
      : [[mono(port), 'not published']],
  );
  const networks = Object.entries(inspect.NetworkSettings.Networks ?? {});
  const labels = Object.entries(inspect.Config.Labels ?? {}).sort(([a], [b]) => a.localeCompare(b));
  const health = inspect.State.Health;
  const restart = inspect.HostConfig.RestartPolicy;

  return (
    <Tabs defaultValue="details">
      <TabsList>
        <TabsTrigger value="details">Details</TabsTrigger>
        <TabsTrigger value="raw">Raw JSON</TabsTrigger>
      </TabsList>
      <TabsContent value="details" className="grid gap-4 lg:grid-cols-2">
        <Section title="Container">
          <KeyValue
            items={[
              ['ID', mono(inspect.Id)],
              ['Image', inspect.Config.Image],
              ['Created', new Date(inspect.Created).toLocaleString()],
              [
                'State',
                `${inspect.State.Status}${inspect.State.Running ? '' : ` (exit ${inspect.State.ExitCode})`}${inspect.State.OOMKilled ? ', out of memory' : ''}`,
              ],
              [
                'Started',
                inspect.State.StartedAt.startsWith('0001')
                  ? '—'
                  : new Date(inspect.State.StartedAt).toLocaleString(),
              ],
              [
                'Entrypoint',
                inspect.Config.Entrypoint ? mono(inspect.Config.Entrypoint.join(' ')) : '—',
              ],
              ['Command', inspect.Config.Cmd ? mono(inspect.Config.Cmd.join(' ')) : '—'],
              ['Working dir', inspect.Config.WorkingDir || '—'],
              ['User', inspect.Config.User || 'image default'],
              [
                'Restart policy',
                restart?.Name
                  ? `${restart.Name}${restart.MaximumRetryCount ? ` (max ${restart.MaximumRetryCount})` : ''} · restarted ${inspect.RestartCount}×`
                  : 'no',
              ],
              ['Network mode', inspect.HostConfig.NetworkMode],
            ]}
          />
        </Section>
        <Section title="Health">
          {health ? (
            <div className="grid gap-2">
              <span>
                <Badge variant={health.Status === 'unhealthy' ? 'destructive' : 'outline'}>
                  {health.Status}
                </Badge>{' '}
                <span className="text-muted-foreground text-sm">
                  {health.FailingStreak} failing in a row
                </span>
              </span>
              <SimpleTable
                head={['Started', 'Exit', 'Output']}
                rows={(health.Log ?? [])
                  .slice(-5)
                  .reverse()
                  .map((l) => [
                    new Date(l.Start).toLocaleTimeString(),
                    String(l.ExitCode),
                    mono(l.Output.trim()),
                  ])}
              />
            </div>
          ) : (
            <p className="text-muted-foreground text-sm">The image defines no health check.</p>
          )}
        </Section>
        <Section title="Environment">
          <SimpleTable
            head={['Variable', 'Value']}
            rows={env.map(([k, v]) => [mono(k ?? ''), mono(v ?? '')])}
          />
        </Section>
        <Section title="Mounts">
          <SimpleTable
            head={['Type', 'Source', 'Destination', 'Mode']}
            rows={inspect.Mounts.map((m) => [
              m.Type,
              mono(m.Name ?? m.Source),
              mono(m.Destination),
              m.RW ? 'read-write' : 'read-only',
            ])}
          />
        </Section>
        <Section title="Ports">
          <SimpleTable head={['Container', 'Published on']} rows={ports} />
        </Section>
        <Section title="Networks">
          <SimpleTable
            head={['Network', 'IP', 'Gateway', 'Aliases']}
            rows={networks.map(([name, n]) => [
              name,
              mono(n.IPAddress || '—'),
              mono(n.Gateway || '—'),
              (n.Aliases ?? []).join(', '),
            ])}
          />
        </Section>
        <div className="lg:col-span-2">
          <Section title="Labels">
            <SimpleTable
              head={['Label', 'Value']}
              rows={labels.map(([k, v]) => [mono(k), mono(v)])}
            />
          </Section>
        </div>
      </TabsContent>
      <TabsContent value="raw">
        <JsonView value={inspect} />
      </TabsContent>
    </Tabs>
  );
}
