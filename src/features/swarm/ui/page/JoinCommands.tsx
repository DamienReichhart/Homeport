import { useQuery } from '@tanstack/react-query';
import { EyeIcon } from 'lucide-react';
import { useState } from 'react';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  CopyButton,
  ErrorMessage,
  Spinner,
} from '@/shared/ui';
import { swarmFeatureQueries } from '../../api/queries';
import { t } from '@/shared/i18n';

/**
 * The `docker swarm join` commands for machines Homeport doesn't manage,
 * fetched only once revealed and dropped from the cache when hidden.
 */
export function JoinCommands({ swarmId }: { swarmId: string }) {
  const [shown, setShown] = useState(false);
  const info = useQuery({ ...swarmFeatureQueries.joinInfo(swarmId), enabled: shown });
  const addr = info.data?.managerAddrs[0] ?? '<manager>:2377';
  const rows = info.data
    ? [
        {
          label: t('swarm.join.worker'),
          command: `docker swarm join --token ${info.data.workerToken} ${addr}`,
        },
        {
          label: t('swarm.join.manager'),
          command: `docker swarm join --token ${info.data.managerToken} ${addr}`,
        },
      ]
    : [];
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('swarm.join.title')}</CardTitle>
        <CardDescription>{t('swarm.join.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-3">
        {!shown ? (
          <Button variant="outline" className="justify-self-start" onClick={() => setShown(true)}>
            <EyeIcon /> {t('swarm.join.reveal')}
          </Button>
        ) : info.error ? (
          <ErrorMessage error={info.error} onRetry={() => void info.refetch()} />
        ) : !info.data ? (
          <Spinner />
        ) : (
          <>
            {rows.map((row) => (
              <div key={row.label} className="grid gap-1">
                <span className="text-sm font-medium">{row.label}</span>
                <div className="bg-muted flex items-start gap-2 rounded-md p-2">
                  <code className="min-w-0 flex-1 font-mono text-xs break-all">{row.command}</code>
                  <CopyButton text={row.command} />
                </div>
              </div>
            ))}
            <Button variant="ghost" className="justify-self-start" onClick={() => setShown(false)}>
              {t('swarm.join.hide')}
            </Button>
          </>
        )}
      </CardContent>
    </Card>
  );
}
