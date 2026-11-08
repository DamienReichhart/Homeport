import type { Section } from '@/core/cluster';
import { t } from '@/shared/i18n';

/** The Helm pages, as tabs. */
export const sections = (): Section[] => [{ path: '/helm', label: t('helm.section.releases') }];
