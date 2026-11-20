import type { Container } from '@/shared/api';
import type { ConfirmOptions } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Confirmation for removing containers from `host`, naming what goes with them. */
export function removeConfirmation(
  containers: Container[],
  host: string,
  onConfirm: () => Promise<unknown>,
): ConfirmOptions {
  const running = containers.filter((c) => c.state === 'running').length;
  const count = containers.length;
  return {
    title:
      count === 1
        ? t('containers.remove.title', { name: containers[0]?.name, host })
        : t('containers.remove.titleMany', { count, host }),
    description: (
      <>
        {t('containers.remove.deleted', { count })}{' '}
        {running > 0 && <strong>{t('containers.remove.running', { count: running })} </strong>}
        {t('containers.remove.namedKept')}
      </>
    ),
    confirmLabel: t('common.remove'),
    destructive: true,
    onConfirm,
  };
}
