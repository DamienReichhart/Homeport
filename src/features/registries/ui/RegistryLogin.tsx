import { useQuery } from '@tanstack/react-query';
import { KeyRoundIcon } from 'lucide-react';
import { useDeferredValue } from 'react';
import { registryQueries } from '../api/queries';
import { describeLogin } from '../model/login';

/**
 * Which stored login an image reference will use, as a line under a form
 * field: "Pulled as alice on ghcr.io" or "Pulled anonymously".
 */
export function RegistryLogin({ reference, verb }: { reference: string; verb: string }) {
  const deferred = useDeferredValue(reference.trim());
  const login = useQuery(registryQueries.loginFor(deferred));
  if (!deferred || login.isPending || login.error) return null;
  return (
    <p className="text-muted-foreground inline-flex items-center gap-1.5 text-xs">
      <KeyRoundIcon className="size-3.5" />
      {verb} {describeLogin(login.data)}
    </p>
  );
}
