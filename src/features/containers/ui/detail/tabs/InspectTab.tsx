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
import type { ContainerInspect } from '../../../model/types';
import { t } from '@/shared/i18n';

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
  if (rows.length === 0) return <p className="text-muted-foreground text-sm">{t('common.none')}</p>;
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
      : [[mono(port), t('containers.inspect.notPublished')]],
  );
  const networks = Object.entries(inspect.NetworkSettings.Networks ?? {});
  const labels = Object.entries(inspect.Config.Labels ?? {}).sort(([a], [b]) => a.localeCompare(b));
  const health = inspect.State.Health;
  const restart = inspect.HostConfig.RestartPolicy;
  const { Status: status, ExitCode: code } = inspect.State;
  const state = inspect.State.Running ? status : t('containers.inspect.exited', { status, code });
  const restarted = (policy: string, max: number) =>
    max
      ? t('containers.inspect.restartedMax', { policy, max, count: inspect.RestartCount })
      : t('containers.inspect.restarted', { policy, count: inspect.RestartCount });

  return (
    <Tabs defaultValue="details">
      <TabsList>
        <TabsTrigger value="details">{t('common.details')}</TabsTrigger>
        <TabsTrigger value="raw">{t('containers.inspect.raw')}</TabsTrigger>
      </TabsList>
      <TabsContent value="details" className="grid gap-4 lg:grid-cols-2">
        <Section title={t('containers.inspect.container')}>
          <KeyValue
            items={[
              [t('containers.inspect.id'), mono(inspect.Id)],
              [t('containers.inspect.image'), inspect.Config.Image],
              [t('containers.inspect.created'), new Date(inspect.Created).toLocaleString()],
              [
                t('containers.inspect.state'),
                inspect.State.OOMKilled ? t('containers.inspect.outOfMemory', { state }) : state,
              ],
              [
                t('containers.inspect.started'),
                inspect.State.StartedAt.startsWith('0001')
                  ? '—'
                  : new Date(inspect.State.StartedAt).toLocaleString(),
              ],
              [
                t('containers.inspect.entrypoint'),
                inspect.Config.Entrypoint ? mono(inspect.Config.Entrypoint.join(' ')) : '—',
              ],
              [
                t('containers.inspect.command'),
                inspect.Config.Cmd ? mono(inspect.Config.Cmd.join(' ')) : '—',
              ],
              [t('containers.inspect.workingDir'), inspect.Config.WorkingDir || '—'],
              [
                t('containers.inspect.user'),
                inspect.Config.User || t('containers.inspect.imageDefault'),
              ],
              [
                t('containers.inspect.restartPolicy'),
                // `no` is the Engine's name for the policy, not a word.
                restart?.Name ? restarted(restart.Name, restart.MaximumRetryCount) : 'no',
              ],
              [t('containers.inspect.networkMode'), inspect.HostConfig.NetworkMode],
            ]}
          />
        </Section>
        <Section title={t('containers.inspect.health')}>
          {health ? (
            <div className="grid gap-2">
              <span>
                <Badge variant={health.Status === 'unhealthy' ? 'destructive' : 'outline'}>
                  {health.Status}
                </Badge>{' '}
                <span className="text-muted-foreground text-sm">
                  {t('containers.inspect.failingStreak', { count: health.FailingStreak })}
                </span>
              </span>
              <SimpleTable
                head={[
                  t('containers.inspect.column.started'),
                  t('containers.inspect.column.exit'),
                  t('containers.inspect.column.output'),
                ]}
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
            <p className="text-muted-foreground text-sm">{t('containers.inspect.noHealthcheck')}</p>
          )}
        </Section>
        <Section title={t('containers.inspect.environment')}>
          <SimpleTable
            head={[t('containers.inspect.column.variable'), t('containers.inspect.column.value')]}
            rows={env.map(([k, v]) => [mono(k ?? ''), mono(v ?? '')])}
          />
        </Section>
        <Section title={t('containers.inspect.mounts')}>
          <SimpleTable
            head={[
              t('containers.inspect.column.type'),
              t('containers.inspect.column.source'),
              t('containers.inspect.column.destination'),
              t('containers.inspect.column.mode'),
            ]}
            rows={inspect.Mounts.map((m) => [
              m.Type,
              mono(m.Name ?? m.Source),
              mono(m.Destination),
              m.RW ? t('containers.inspect.readWrite') : t('containers.inspect.readOnly'),
            ])}
          />
        </Section>
        <Section title={t('containers.inspect.ports')}>
          <SimpleTable
            head={[
              t('containers.inspect.column.container'),
              t('containers.inspect.column.publishedOn'),
            ]}
            rows={ports}
          />
        </Section>
        <Section title={t('containers.inspect.networks')}>
          <SimpleTable
            head={[
              t('containers.inspect.column.network'),
              t('containers.inspect.column.ip'),
              t('containers.inspect.column.gateway'),
              t('containers.inspect.column.aliases'),
            ]}
            rows={networks.map(([name, n]) => [
              name,
              mono(n.IPAddress || '—'),
              mono(n.Gateway || '—'),
              (n.Aliases ?? []).join(', '),
            ])}
          />
        </Section>
        <div className="lg:col-span-2">
          <Section title={t('containers.inspect.labels')}>
            <SimpleTable
              head={[t('containers.inspect.column.label'), t('containers.inspect.column.value')]}
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
