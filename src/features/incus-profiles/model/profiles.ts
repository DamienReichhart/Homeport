import type { IncusProfile } from '@/shared/api';
import { t } from '@/shared/i18n';

/** Why a profile can't be deleted, or `null` when it can. */
export function undeletable(profile: IncusProfile): string | null {
  if (profile.name === 'default') return t('incus.profiles.defaultKept');
  if (profile.usedBy.length > 0)
    return t('incus.profiles.inUse', { names: profile.usedBy.join(', ') });
  return null;
}
