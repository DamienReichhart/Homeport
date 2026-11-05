import type { Theme } from '@/shared/lib';
import type { MessageKey } from '@/shared/i18n';

export const THEMES: { value: Theme; label: MessageKey }[] = [
  { value: 'system', label: 'settings.theme.system' },
  { value: 'light', label: 'settings.theme.light' },
  { value: 'dark', label: 'settings.theme.dark' },
];
