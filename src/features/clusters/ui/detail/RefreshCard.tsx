import { useState } from 'react';
import { kubernetesConnection } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  ErrorMessage,
  Label,
  Textarea,
} from '@/shared/ui';
import { useRefreshCluster } from '../../api/mutations';
import { t } from '@/shared/i18n';

/**
 * Reads the Cluster's context again: from this machine's kubeconfig, or from
 * a pasted one when it was pasted (kind and k3d recreate Clusters with new
 * certificates).
 */
export function RefreshCard({ view }: { view: ClusterView }) {
  const source = kubernetesConnection(view.cluster)?.source;
  const refresh = useRefreshCluster(view.cluster.id);
  const [text, setText] = useState('');
  if (!source) return null;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('clusters.refresh.title')}</CardTitle>
        <CardDescription>
          {t(source.pasted ? 'clusters.refresh.pasted' : 'clusters.refresh.machine', {
            context: source.context,
          })}
        </CardDescription>
      </CardHeader>
      <CardContent className="grid gap-3">
        {source.pasted && (
          <>
            <Label htmlFor="refresh-kubeconfig">{t('clusters.paste.label')}</Label>
            <Textarea
              id="refresh-kubeconfig"
              value={text}
              onChange={(e) => setText(e.target.value)}
              className="min-h-32 font-mono text-xs"
              spellCheck={false}
              autoComplete="off"
            />
          </>
        )}
        {refresh.error && <ErrorMessage error={refresh.error} />}
        <div>
          <Button
            variant="outline"
            disabled={refresh.isPending || (source.pasted && !text.trim())}
            onClick={() =>
              refresh.mutate(source.pasted ? text : null, { onSuccess: () => setText('') })
            }
          >
            {t('clusters.refresh.submit')}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
