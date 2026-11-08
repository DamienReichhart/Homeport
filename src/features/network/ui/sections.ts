import type { Section } from '@/core/cluster';
import { t } from '@/shared/i18n';

/** The network pages, as tabs between them. */
export const sections = (): Section[] => [
  { path: '/services', label: t('network.services') },
  { path: '/ingresses', label: t('network.ingresses') },
];
