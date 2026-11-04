import { useQuery } from '@tanstack/react-query';
import { MonitorIcon, PlusIcon } from 'lucide-react';
import type { HostInput } from '@/shared/api';
import { Button, Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/shared/ui';
import { hostQueries, useHosts } from '../api/queries';
import { t } from '@/shared/i18n';

/** Engines answering on this machine's sockets that aren't Hosts yet, one click to add. */
export function LocalEngines({
  onAdd,
  pending,
}: {
  onAdd: (input: HostInput) => void;
  pending: boolean;
}) {
  const hosts = useHosts();
  const detected = useQuery(hostQueries.localEngines());
  const added = new Set(
    hosts.data?.flatMap(({ host }) =>
      host.connection.kind === 'local' ? [host.connection.socketPath] : [],
    ),
  );
  const local = detected.data?.filter((e) => !added.has(e.socketPath)) ?? [];
  if (local.length === 0) return null;

  return (
    <Card className="w-full text-left">
      <CardHeader>
        <CardTitle>{t('core.localEngines.title')}</CardTitle>
        <CardDescription>{t('core.localEngines.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-2">
        {local.map((engine) => (
          <div key={engine.socketPath} className="flex items-center justify-between gap-3">
            <div className="flex min-w-0 items-center gap-2">
              <MonitorIcon className="text-muted-foreground size-4" />
              <div className="min-w-0">
                <p className="text-sm font-medium">
                  {engine.label}{' '}
                  <span className="text-muted-foreground">
                    · {t('core.host.engineVersion', { version: engine.engineVersion })}
                  </span>
                </p>
                <p className="text-muted-foreground truncate font-mono text-xs">
                  {engine.socketPath}
                </p>
              </div>
            </div>
            <Button
              size="sm"
              disabled={pending}
              onClick={() =>
                onAdd({
                  name: engine.label,
                  connection: { kind: 'local', socketPath: engine.socketPath },
                  credentials: null,
                })
              }
            >
              <PlusIcon /> {t('core.localEngines.add')}
            </Button>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
