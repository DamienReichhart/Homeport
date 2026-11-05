import type { MessageKey } from '@/shared/i18n';

/** A starting point for a new Managed Stack, picked before the editor opens. */
export interface StackTemplate {
  id: string;
  /** Catalogue keys of its title and its one-line description. */
  title: MessageKey;
  description: MessageKey;
  /** The compose file: pinned image tags, named volumes, no secrets. */
  compose: string;
  /**
   * The environment: values to change, and secrets left empty with a
   * comment saying what goes there.
   */
  env: string;
  /** The Stack name it suggests; empty to let the user choose. */
  name: string;
}
