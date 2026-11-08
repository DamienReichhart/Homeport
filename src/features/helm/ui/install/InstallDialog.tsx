import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { InstallRequest, ReleasePreview } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { usePreviewInstall } from '../../api/mutations';
import { helmQueries } from '../../api/queries';
import {
  namespaceProblem,
  releaseNameProblem,
  suggestedName,
  timeoutSeconds,
} from '../../model/install';
import { PreviewView } from '../form/PreviewView';
import { ValuesField } from '../form/ValuesField';
import { VersionField } from '../form/VersionField';
import { WaitFields } from '../form/WaitFields';
import { ChartStep } from './ChartStep';
import { t } from '@/shared/i18n';

/**
 * Installs a chart on a Cluster: pick it, then its version, the release's
 * name and Namespace and its values (the chart's defaults to start with),
 * preview what it renders, and hand the request to `onInstall`.
 */
export function InstallDialog({
  clusterId,
  clusterName,
  namespace,
  onInstall,
  onClose,
}: {
  clusterId: string;
  clusterName: string;
  /** The Namespace to suggest. */
  namespace: string;
  onInstall: (request: InstallRequest) => void;
  onClose: () => void;
}) {
  const [chart, setChart] = useState<string | null>(null);
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-4xl">
        <DialogHeader>
          <DialogTitle>
            {chart
              ? t('helm.install.titleChart', { chart, cluster: clusterName })
              : t('helm.install.title', { cluster: clusterName })}
          </DialogTitle>
          {!chart && <DialogDescription>{t('helm.install.description')}</DialogDescription>}
        </DialogHeader>
        {chart ? (
          <Configure
            key={chart}
            clusterId={clusterId}
            chart={chart}
            namespace={namespace}
            onBack={() => setChart(null)}
            onInstall={onInstall}
          />
        ) : (
          <ChartStep onPick={setChart} />
        )}
      </DialogContent>
    </Dialog>
  );
}

function Configure({
  clusterId,
  chart,
  namespace: suggested,
  onBack,
  onInstall,
}: {
  clusterId: string;
  chart: string;
  namespace: string;
  onBack: () => void;
  onInstall: (request: InstallRequest) => void;
}) {
  const [version, setVersion] = useState('');
  const [name, setName] = useState(suggestedName(chart));
  const [namespace, setNamespace] = useState(suggested);
  const [createNamespace, setCreateNamespace] = useState(true);
  const [wait, setWait] = useState(false);
  const [minutes, setMinutes] = useState('5');
  const [edited, setEdited] = useState<string | null>(null);
  const [touched, setTouched] = useState(false);
  const [preview, setPreview] = useState<ReleasePreview | null>(null);
  const defaults = useQuery(helmQueries.chartValues(chart, version || null));
  const render = usePreviewInstall(clusterId);
  const values = edited ?? defaults.data ?? '';
  const problem = releaseNameProblem(name) ?? namespaceProblem(namespace);
  const request: InstallRequest = {
    name,
    namespace,
    chart,
    version: version || null,
    values,
    createNamespace,
    wait,
    timeoutSeconds: wait ? timeoutSeconds(minutes) : null,
  };
  const go = (next: () => void) => {
    setTouched(true);
    if (!problem) next();
  };

  if (preview)
    return (
      <>
        <PreviewView preview={preview} />
        <DialogFooter>
          <Button variant="outline" onClick={() => setPreview(null)}>
            {t('helm.install.back')}
          </Button>
          <Button onClick={() => onInstall(request)}>{t('helm.install.install')}</Button>
        </DialogFooter>
      </>
    );

  return (
    <div className="grid gap-4">
      <div className="grid gap-4 sm:grid-cols-3">
        <VersionField
          chart={chart}
          value={version}
          onChange={(v) => {
            setVersion(v);
            setPreview(null);
          }}
        />
        <div className="grid gap-2">
          <Label htmlFor="helm-name">{t('helm.install.name')}</Label>
          <Input id="helm-name" value={name} onChange={(e) => setName(e.target.value.trim())} />
        </div>
        <div className="grid gap-2">
          <Label htmlFor="helm-namespace">{t('helm.install.namespace')}</Label>
          <Input
            id="helm-namespace"
            value={namespace}
            onChange={(e) => setNamespace(e.target.value.trim())}
          />
        </div>
      </div>
      <div className="flex flex-wrap items-center gap-6">
        <div className="flex items-center gap-2">
          <Checkbox
            id="helm-create-namespace"
            checked={createNamespace}
            onCheckedChange={(v) => setCreateNamespace(v === true)}
          />
          <Label htmlFor="helm-create-namespace" className="font-normal">
            {t('helm.install.createNamespace')}
          </Label>
        </div>
        <WaitFields wait={wait} minutes={minutes} onWait={setWait} onMinutes={setMinutes} />
      </div>
      {touched && problem && (
        <p role="alert" className="text-destructive text-sm">
          {t(problem)}
        </p>
      )}
      {defaults.error ? (
        <ErrorMessage error={defaults.error} onRetry={() => void defaults.refetch()} />
      ) : (
        <ValuesField
          value={values}
          onChange={setEdited}
          reset={defaults.data ?? null}
          resetLabel={t('helm.install.resetDefaults')}
        />
      )}
      {render.error && <ErrorMessage error={render.error} />}
      <DialogFooter>
        <Button variant="outline" onClick={onBack}>
          {t('helm.install.changeChart')}
        </Button>
        <Button
          variant="outline"
          disabled={render.isPending || defaults.isPending}
          onClick={() => go(() => render.mutate(request, { onSuccess: setPreview }))}
        >
          {render.isPending ? t('helm.install.previewing') : t('helm.install.preview')}
        </Button>
        <Button disabled={defaults.isPending} onClick={() => go(() => onInstall(request))}>
          {t('helm.install.install')}
        </Button>
      </DialogFooter>
    </div>
  );
}
