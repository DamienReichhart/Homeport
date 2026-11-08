import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { ReleasePreview, ReleaseRevision, UpgradeRequest } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Spinner,
  ToggleGroup,
  ToggleGroupItem,
  YamlDiff,
} from '@/shared/ui';
import { usePreviewUpgrade } from '../../api/mutations';
import { helmQueries } from '../../api/queries';
import { isOci, timeoutSeconds } from '../../model/install';
import { ChartField } from '../form/ChartField';
import { PreviewView } from '../form/PreviewView';
import { ValuesField } from '../form/ValuesField';
import { VersionField } from '../form/VersionField';
import { WaitFields } from '../form/WaitFields';
import { t } from '@/shared/i18n';

/**
 * Upgrades a release: the chart to upgrade from and its version, values
 * starting from the release's user-supplied ones with their diff, a
 * preview of the manifest against the current one, then `onUpgrade`.
 */
export function UpgradeDialog({
  clusterId,
  clusterName,
  namespace,
  name,
  current,
  onUpgrade,
  onClose,
}: {
  clusterId: string;
  clusterName: string;
  namespace: string;
  name: string;
  /** The release's current revision. */
  current: ReleaseRevision;
  onUpgrade: (request: UpgradeRequest) => void;
  onClose: () => void;
}) {
  const details = useQuery(helmQueries.details(clusterId, namespace, name, current.revision));
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-4xl">
        <DialogHeader>
          <DialogTitle>{t('helm.upgrade.title', { name, cluster: clusterName })}</DialogTitle>
        </DialogHeader>
        {details.isPending ? (
          <Spinner />
        ) : details.error ? (
          <ErrorMessage error={details.error} onRetry={() => void details.refetch()} />
        ) : (
          <Form
            clusterId={clusterId}
            namespace={namespace}
            name={name}
            current={current}
            values={details.data.userValues}
            manifest={details.data.manifest}
            onUpgrade={onUpgrade}
            onClose={onClose}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

function Form({
  clusterId,
  namespace,
  name,
  current,
  values: currentValues,
  manifest,
  onUpgrade,
  onClose,
}: {
  clusterId: string;
  namespace: string;
  name: string;
  current: ReleaseRevision;
  values: string;
  manifest: string;
  onUpgrade: (request: UpgradeRequest) => void;
  onClose: () => void;
}) {
  // `null` until the user picks or types one: the likeliest repository chart.
  const [chart, setChart] = useState<string | null>(null);
  const candidates = useQuery(helmQueries.chartCandidates(current.chartName, current.chartVersion));
  const effectiveChart = chart ?? candidates.data?.[0] ?? '';
  const [version, setVersion] = useState('');
  const [values, setValues] = useState(currentValues);
  const [view, setView] = useState<'edit' | 'diff'>('edit');
  const [wait, setWait] = useState(false);
  const [minutes, setMinutes] = useState('5');
  const [preview, setPreview] = useState<ReleasePreview | null>(null);
  const render = usePreviewUpgrade(clusterId);
  const ready = effectiveChart !== '' && (isOci(effectiveChart) || effectiveChart.includes('/'));
  const request: UpgradeRequest = {
    name,
    namespace,
    chart: effectiveChart,
    version: version || null,
    values,
    wait,
    timeoutSeconds: wait ? timeoutSeconds(minutes) : null,
  };

  if (preview)
    return (
      <>
        <PreviewView preview={preview} />
        <p className="text-sm font-medium">{t('helm.upgrade.manifestDiff')}</p>
        <YamlDiff
          before={manifest}
          after={preview.manifest}
          label={t('helm.upgrade.manifestDiff')}
        />
        <DialogFooter>
          <Button variant="outline" onClick={() => setPreview(null)}>
            {t('helm.install.back')}
          </Button>
          <Button onClick={() => onUpgrade(request)}>{t('helm.upgrade.upgrade')}</Button>
        </DialogFooter>
      </>
    );

  return (
    <div className="grid gap-4">
      <p className="text-muted-foreground text-sm">
        {t('helm.upgrade.current', { chart: current.chart, app: current.appVersion || '—' })}
      </p>
      <div className="grid gap-4 sm:grid-cols-2">
        <ChartField
          chartName={current.chartName}
          chartVersion={current.chartVersion}
          value={effectiveChart}
          onChange={(c) => {
            setChart(c);
            setVersion('');
          }}
        />
        {ready && (
          <VersionField
            key={effectiveChart}
            chart={effectiveChart}
            value={version}
            onChange={setVersion}
          />
        )}
      </div>
      <WaitFields wait={wait} minutes={minutes} onWait={setWait} onMinutes={setMinutes} />
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        value={view}
        onValueChange={(v) => v && setView(v as 'edit' | 'diff')}
        className="justify-self-start"
      >
        <ToggleGroupItem value="edit">{t('helm.upgrade.editValues')}</ToggleGroupItem>
        <ToggleGroupItem value="diff">{t('helm.upgrade.valuesDiff')}</ToggleGroupItem>
      </ToggleGroup>
      {view === 'edit' ? (
        <ValuesField
          value={values}
          onChange={setValues}
          reset={currentValues}
          resetLabel={t('helm.upgrade.resetValues')}
        />
      ) : (
        <YamlDiff before={currentValues} after={values} label={t('helm.upgrade.valuesDiff')} />
      )}
      <p className="text-muted-foreground text-xs">{t('helm.upgrade.valuesNote')}</p>
      {render.error && <ErrorMessage error={render.error} />}
      <DialogFooter>
        <Button variant="outline" onClick={onClose}>
          {t('common.cancel')}
        </Button>
        <Button
          variant="outline"
          disabled={!ready || render.isPending}
          onClick={() => render.mutate(request, { onSuccess: setPreview })}
        >
          {render.isPending ? t('helm.install.previewing') : t('helm.install.preview')}
        </Button>
        <Button disabled={!ready} onClick={() => onUpgrade(request)}>
          {t('helm.upgrade.upgrade')}
        </Button>
      </DialogFooter>
    </div>
  );
}
