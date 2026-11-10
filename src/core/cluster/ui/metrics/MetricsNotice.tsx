import { openUrl } from '@tauri-apps/plugin-opener';
import { ExternalLinkIcon, GaugeIcon } from 'lucide-react';
import {
  Alert,
  AlertDescription,
  AlertTitle,
  ErrorMessage,
  Spinner,
  toastError,
} from '@/shared/ui';
import type { MetricsState } from '../../model/metrics';
import { t } from '@/shared/i18n';

/** metrics-server's own page, with how to install it. */
export const METRICS_SERVER_URL = 'https://github.com/kubernetes-sigs/metrics-server';

/**
 * Why a Metrics tab has no chart: metrics-server isn't installed, the
 * credentials may not read metrics, it doesn't answer yet, or the first
 * sample hasn't come. Nothing when there's something to chart.
 */
export function MetricsNotice({ metrics, waiting }: { metrics: MetricsState; waiting: boolean }) {
  const { unavailable, problem } = metrics;
  if (unavailable?.reason === 'notInstalled')
    return (
      <Alert>
        <GaugeIcon />
        <AlertTitle>{t('core.metrics.notInstalled.title')}</AlertTitle>
        <AlertDescription className="flex flex-col gap-2">
          <p>{t('core.metrics.notInstalled.body')}</p>
          <a
            href={METRICS_SERVER_URL}
            className="text-primary inline-flex items-center gap-1 underline-offset-4 hover:underline"
            onClick={(event) => {
              event.preventDefault();
              openUrl(METRICS_SERVER_URL).catch(toastError);
            }}
            // A middle click would open the page in a new webview window.
            onAuxClick={(event) => event.preventDefault()}
          >
            {t('core.metrics.notInstalled.link')}
            <ExternalLinkIcon className="size-3.5" aria-hidden />
          </a>
        </AlertDescription>
      </Alert>
    );
  if (unavailable?.reason === 'forbidden') return <ErrorMessage error={unavailable.error} />;
  if (problem?.notResponding)
    return <p className="text-muted-foreground text-sm">{t('core.metrics.notResponding')}</p>;
  if (problem) return <ErrorMessage error={problem.error} />;
  if (waiting) return <Spinner label={t('core.metrics.waiting')} />;
  return null;
}
