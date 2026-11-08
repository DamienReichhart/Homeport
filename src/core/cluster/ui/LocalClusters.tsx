import { useQuery } from '@tanstack/react-query';
import { PlusIcon, ShipWheelIcon } from 'lucide-react';
import type { ClusterView, LocalCluster, LocalTool } from '@/shared/api';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  CopyButton,
} from '@/shared/ui';
import { useAddLocalCluster } from '../api/mutations';
import { clusterQueries } from '../api/queries';
import { t, type MessageKey } from '@/shared/i18n';

const TOOLS: Record<LocalTool, MessageKey> = {
  dockerDesktop: 'core.localClusters.tool.dockerDesktop',
  orbstack: 'core.localClusters.tool.orbstack',
  rancherDesktop: 'core.localClusters.tool.rancherDesktop',
  kind: 'core.localClusters.tool.kind',
  k3d: 'core.localClusters.tool.k3d',
  minikube: 'core.localClusters.tool.minikube',
};

function Found({
  local,
  onAdd,
  pending,
}: {
  local: LocalCluster;
  onAdd: () => void;
  pending: boolean;
}) {
  const running = local.serverVersion !== null;
  return (
    <div className="flex items-center justify-between gap-3">
      <div className="flex min-w-0 items-center gap-2">
        <ShipWheelIcon className="text-muted-foreground size-4" />
        <div className="min-w-0">
          <p className="text-sm font-medium">
            {local.suggestedName}{' '}
            <span className="text-muted-foreground">
              {local.suggestedName !== t(TOOLS[local.tool]) && ` · ${t(TOOLS[local.tool])}`}
              {running && ` · ${t('core.cluster.serverVersion', { version: local.serverVersion })}`}
            </span>
          </p>
          <p className="text-muted-foreground truncate font-mono text-xs">{local.server}</p>
        </div>
      </div>
      {local.added ? (
        <Badge variant="secondary">{t('core.localClusters.added')}</Badge>
      ) : running ? (
        <Button size="sm" disabled={pending} onClick={onAdd}>
          <PlusIcon /> {t('core.localClusters.add')}
        </Button>
      ) : (
        <Badge variant="outline">{t('core.localClusters.notRunning')}</Badge>
      )}
    </div>
  );
}

/**
 * Kubernetes clusters running on this machine that its kubeconfig reaches,
 * one click to add; and kind or k3d ones it doesn't reach yet, with the
 * command that adds them to it.
 */
export function LocalClusters({
  onAdded,
  hideAdded = false,
}: {
  onAdded?: (view: ClusterView) => void;
  /** Leave out those already added, as the welcome page does. */
  hideAdded?: boolean;
}) {
  const detected = useQuery(clusterQueries.localClusters());
  const add = useAddLocalCluster(onAdded);
  const found = (detected.data?.found ?? []).filter((c) => !(hideAdded && c.added));
  const missing = detected.data?.missing ?? [];
  if (found.length === 0 && missing.length === 0) return null;

  return (
    <Card className="w-full text-left">
      <CardHeader>
        <CardTitle>{t('core.localClusters.title')}</CardTitle>
        <CardDescription>{t('core.localClusters.description')}</CardDescription>
      </CardHeader>
      <CardContent className="grid gap-2">
        {found.map((local) => (
          <Found
            key={local.context}
            local={local}
            pending={add.isPending}
            onAdd={() => add.mutate(local)}
          />
        ))}
        {missing.map((m) => (
          <div key={`${m.tool}:${m.name}`} className="grid gap-1 text-sm">
            <p>{t('core.localClusters.missing', { tool: t(TOOLS[m.tool]), name: m.name })}</p>
            <div className="bg-muted flex items-center justify-between gap-2 rounded px-2 py-1">
              <code className="truncate text-xs">{m.command}</code>
              <CopyButton text={m.command} />
            </div>
          </div>
        ))}
      </CardContent>
    </Card>
  );
}
