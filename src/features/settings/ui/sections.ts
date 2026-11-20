import type { MessageKey } from '@/shared/i18n';

export interface SettingsSection {
  id: string;
  label: MessageKey;
}

/** The DOM id of a section of the Settings page. */
export const sectionId = (id: string) => `settings-${id}`;

/** Brings a section to the top of the page and moves the focus to it. */
export function goToSection(id: string) {
  const section = document.getElementById(sectionId(id));
  section?.scrollIntoView?.({ block: 'start' });
  section?.focus({ preventScroll: true });
}
