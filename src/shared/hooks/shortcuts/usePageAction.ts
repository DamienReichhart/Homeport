import { useShortcut } from './useShortcut';
import { t } from '@/shared/i18n';

/** The key that starts a page's main action. */
export const PAGE_ACTION_KEYS = 'n';

/**
 * Makes `n` start the page's main action, such as Run container or New
 * Stack, listed as `label` under "This page".
 */
export function usePageAction(label: string, handler: () => void, enabled = true) {
  useShortcut({
    keys: PAGE_ACTION_KEYS,
    label,
    group: t('ui.shortcuts.thisPage'),
    handler,
    enabled,
  });
}
