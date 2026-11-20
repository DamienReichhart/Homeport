import { FileUpIcon } from 'lucide-react';
import { useState } from 'react';
import { ApplyFlow, ClusterGate, NamespacePicker, useNamespace } from '@/core/cluster';
import type { ClusterView } from '@/shared/api';
import { Button, PageHeader, toastError, YamlEditor } from '@/shared/ui';
import { openManifest } from '../api/files';
import { t } from '@/shared/i18n';

function ApplyYaml({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const [namespace, setNamespace] = useNamespace(cluster);
  const [yaml, setYaml] = useState('');
  return (
    <div className="grid gap-3">
      <PageHeader
        title={t('apply.title')}
        description={t('apply.description', { cluster: cluster.cluster.name })}
        actions={
          <>
            <NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />
            <Button
              variant="outline"
              onClick={() =>
                void openManifest()
                  .then((text) => text !== null && setYaml(text))
                  .catch((e: unknown) => toastError(e, t('apply.openFailed')))
              }
            >
              <FileUpIcon /> {t('apply.open')}
            </Button>
          </>
        }
      />
      <YamlEditor value={yaml} onChange={setYaml} label={t('apply.editor')} height="24rem" />
      <p className="text-muted-foreground text-sm">
        {namespace
          ? t('apply.defaultNamespace', { namespace })
          : t('apply.defaultNamespaceCluster')}
      </p>
      <ApplyFlow
        clusterId={id}
        clusterName={cluster.cluster.name}
        yaml={yaml}
        namespace={namespace}
      />
    </div>
  );
}

export function ApplyPage() {
  return (
    <ClusterGate>
      {(cluster) => <ApplyYaml key={cluster.cluster.id} cluster={cluster} />}
    </ClusterGate>
  );
}
