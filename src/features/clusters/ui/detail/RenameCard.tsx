import { useState } from 'react';
import type { ClusterView } from '@/shared/api';
import { Button, Card, CardContent, CardHeader, CardTitle, ErrorMessage, Input } from '@/shared/ui';
import { useRenameCluster } from '../../api/mutations';
import { t } from '@/shared/i18n';

/** Renames a Cluster; its kubeconfig context keeps its own name. */
export function RenameCard({ view }: { view: ClusterView }) {
  const [name, setName] = useState(view.cluster.name);
  const rename = useRenameCluster(view.cluster.id);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('clusters.detail.name')}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-3">
        <div className="flex gap-2">
          <Input
            aria-label={t('clusters.detail.name')}
            value={name}
            onChange={(e) => setName(e.target.value)}
            className="max-w-sm"
          />
          <Button
            variant="outline"
            disabled={rename.isPending || name.trim() === view.cluster.name}
            onClick={() => rename.mutate(name)}
          >
            {t('clusters.detail.rename')}
          </Button>
        </div>
        {rename.error && <ErrorMessage error={rename.error} />}
      </CardContent>
    </Card>
  );
}
