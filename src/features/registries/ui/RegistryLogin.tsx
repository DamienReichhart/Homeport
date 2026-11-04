import { useQuery } from '@tanstack/react-query';
import { KeyRoundIcon } from 'lucide-react';
import { useDeferredValue } from 'react';
import type { Registry } from '@/shared/api';
import { t } from '@/shared/i18n';
import { registryQueries } from '../api/queries';

/**
 * What the image is about to be done to: `pull` or `push`. `Pulled` and
 * `Pushed` are accepted too, for callers written before the catalogue.
 */
export type RegistryVerb = 'pull' | 'push';

/** The sentence for a verb and a login, from literal keys. */
function sentence(verb: RegistryVerb, login: Registry | null): string {
  const push = verb === 'push';
  if (!login)
    return push ? t('registries.login.pushedAnonymously') : t('registries.login.pulledAnonymously');
  const values = { username: login.username, registry: login.address };
  return push ? t('registries.login.pushedAs', values) : t('registries.login.pulledAs', values);
}

/**
 * Which stored login an image reference will use, as a line under a form
 * field: "Pulled as alice on ghcr.io" or "Pulled anonymously".
 */
export function RegistryLogin({ reference, verb }: { reference: string; verb: RegistryVerb }) {
  const deferred = useDeferredValue(reference.trim());
  const login = useQuery(registryQueries.loginFor(deferred));
  if (!deferred || login.isPending || login.error) return null;
  return (
    <p className="text-muted-foreground inline-flex items-center gap-1.5 text-xs">
      <KeyRoundIcon className="size-3.5" />
      {sentence(verb, login.data)}
    </p>
  );
}
