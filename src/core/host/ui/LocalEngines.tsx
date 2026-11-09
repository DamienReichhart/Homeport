import { useQuery } from '@tanstack/react-query';
import { MonitorIcon, PlusIcon } from 'lucide-react';
import type { EngineKind, HostInput } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
} from '@/shared/ui';
import { hostQueries, useHosts } from '../api/queries';
import { t, type MessageKey } from '@/shared/i18n';

const VERSION_LABELS: Record<EngineKind, MessageKey> = {
  docker: 'core.host.engineVersion',
  containerd: 'core.host.containerdVersion',
  incus: 'core.host.incusVersion',
};

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
    <Card className="w-full text-start">
      <CardHeader>
        <CardTitle>{t('core.localEngines.title')}</CardTitle>
        <CardDescription>{t('core.localEngines.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-2">
        {local.map((engine) =>
          engine.problem ? (
            <div key={engine.socketPath} className="grid gap-1">
              <p className="text-sm font-medium">
                {engine.label}{' '}
                <span className="text-muted-foreground font-mono text-xs">{engine.socketPath}</span>
              </p>
              <ErrorMessage error={engine.problem} />
            </div>
          ) : (
            <div key={engine.socketPath} className="flex items-center justify-between gap-3">
              <div className="flex min-w-0 items-center gap-2">
                <MonitorIcon className="text-muted-foreground size-4" />
                <div className="min-w-0">
                  <p className="text-sm font-medium">
                    {engine.label}{' '}
                    <span className="text-muted-foreground">
                      · {t(VERSION_LABELS[engine.engine], { version: engine.engineVersion })}
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
                    engine: engine.engine,
                    credentials: null,
                  })
                }
              >
                <PlusIcon /> {t('core.localEngines.add')}
              </Button>
            </div>
          ),
        )}
      </CardContent>
    </Card>
  );
}
