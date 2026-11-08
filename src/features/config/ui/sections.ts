import type { Section } from '@/core/cluster';
import { t } from '@/shared/i18n';

/** The configuration pages, as tabs between them. */
export const sections = (): Section[] => [
  { path: '/configmaps', label: t('config.configMaps') },
  { path: '/secrets', label: t('config.secrets') },
];
