import { useQuery } from '@tanstack/react-query';
import { KeyRoundIcon } from 'lucide-react';
import { useDeferredValue } from 'react';
import { Link } from 'react-router-dom';
import { Alert, AlertDescription, AlertTitle } from '@/shared/ui';
import { registryQueries } from '../api/queries';
import { registryOf } from '../model/login';
import { t, tRich } from '@/shared/i18n';

/**
 * Warns before a push that no login is stored for the target's registry,
 * with a way to add one. Renders nothing when there is one.
 */
export function MissingLogin({ reference }: { reference: string }) {
  const deferred = useDeferredValue(reference.trim());
  const login = useQuery(registryQueries.loginFor(deferred));
  if (!deferred || login.isPending || login.error || login.data) return null;
  const registry = registryOf(deferred);
  return (
    <Alert>
      <KeyRoundIcon />
      <AlertTitle>{t('registries.noLogin.title', { registry })}</AlertTitle>
      <AlertDescription>
        <p>
          {tRich('registries.noLogin.description', {
            link: (chunks) => (
              <Link to="/settings" className="underline underline-offset-2">
                {chunks}
              </Link>
            ),
          })}
        </p>
      </AlertDescription>
    </Alert>
  );
}
