import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { LocalClusters, useClusterSelection } from '@/core/cluster';
import { KeychainNeeded } from '@/features/hosts';
import { useCapabilities, type ClusterView, type KubeContexts } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  ErrorMessage,
  Label,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Textarea,
} from '@/shared/ui';
import { clustersQueries } from '../../api/queries';
import { useAddNomadCluster, useReadPastedKubeconfig } from '../../api/mutations';
import { NomadClusterForm } from '../nomad/NomadClusterForm';
import { ImportPanel } from './ImportPanel';
import { t } from '@/shared/i18n';

function FromThisMachine() {
  const contexts = useQuery(clustersQueries.localContexts());
  if (contexts.isPending) return <Spinner />;
  if (contexts.error)
    return <ErrorMessage error={contexts.error} onRetry={() => void contexts.refetch()} />;
  return <ImportPanel found={contexts.data} text={null} />;
}

function Pasted() {
  const [text, setText] = useState('');
  const [read, setRead] = useState<{ text: string; found: KubeContexts } | null>(null);
  const parse = useReadPastedKubeconfig();
  return (
    <div className="grid gap-3">
      <p className="text-muted-foreground text-sm">{t('clusters.paste.description')}</p>
      <Label htmlFor="kubeconfig">{t('clusters.paste.label')}</Label>
      <Textarea
        id="kubeconfig"
        value={text}
        onChange={(e) => {
          setText(e.target.value);
          setRead(null);
        }}
        className="min-h-48 font-mono text-xs"
        spellCheck={false}
        autoComplete="off"
      />
      {parse.error && <ErrorMessage error={parse.error} />}
      <div>
        <Button
          variant="outline"
          disabled={!text.trim() || parse.isPending}
          onClick={() => parse.mutate(text, { onSuccess: (found) => setRead({ text, found }) })}
        >
          {t('clusters.paste.read')}
        </Button>
      </div>
      {read && <ImportPanel key={read.text} found={read.found} text={read.text} />}
    </div>
  );
}

function Nomad({ onAdded }: { onAdded: (view: ClusterView) => void }) {
  const add = useAddNomadCluster(onAdded);
  return (
    <div className="grid gap-3">
      <p className="text-muted-foreground text-sm">{t('clusters.nomad.description')}</p>
      <NomadClusterForm
        submitLabel={t('clusters.nomad.add')}
        pending={add.isPending}
        error={add.error}
        onSubmit={(form) => add.mutate(form)}
      />
    </div>
  );
}

/**
 * Adds Kubernetes Clusters from contexts of this machine's kubeconfig or of
 * a pasted one, or a Nomad Cluster by its address.
 */
export function AddClusterPage() {
  const capabilities = useCapabilities();
  const navigate = useNavigate();
  const { select } = useClusterSelection();
  const opened = (view: ClusterView) => {
    select(view.cluster.id);
    void navigate(`/clusters/${view.cluster.id}`);
  };
  return (
    <div className="max-w-3xl">
      <PageHeader title={t('clusters.add')} description={t('clusters.addPage.description')} />
      {capabilities.data?.keychain === false ? (
        <KeychainNeeded />
      ) : (
        <div className="grid gap-4">
          <LocalClusters onAdded={opened} />
          <Card>
            <CardContent>
              <Tabs defaultValue="machine">
                <TabsList>
                  <TabsTrigger value="machine">{t('clusters.addPage.fromMachine')}</TabsTrigger>
                  <TabsTrigger value="paste">{t('clusters.addPage.paste')}</TabsTrigger>
                  <TabsTrigger value="nomad">{t('clusters.addPage.nomad')}</TabsTrigger>
                </TabsList>
                <TabsContent value="machine" className="pt-3">
                  <FromThisMachine />
                </TabsContent>
                <TabsContent value="paste" className="pt-3">
                  <Pasted />
                </TabsContent>
                <TabsContent value="nomad" className="pt-3">
                  <Nomad onAdded={opened} />
                </TabsContent>
              </Tabs>
            </CardContent>
          </Card>
        </div>
      )}
    </div>
  );
}
