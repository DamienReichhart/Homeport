import { RotateCwIcon, TriangleAlertIcon } from 'lucide-react';
import { useEffect, useMemo } from 'react';
import { useNavigate, useRouteError } from 'react-router-dom';
import { logFailure, toAppError } from '@/shared/api';
import { Button, EmptyState, ErrorMessage } from '@/shared/ui';

/**
 * Shown instead of a page that threw while rendering. Logs the crash and
 * offers a way out; the Details carry the stack for a bug report.
 */
export function CrashPage() {
  const routeError = useRouteError();
  const navigate = useNavigate();
  const error = useMemo(() => toAppError(routeError), [routeError]);
  useEffect(() => logFailure(error, 'error'), [error]);

  return (
    <EmptyState
      icon={TriangleAlertIcon}
      title="Something went wrong"
      description={
        <div className="grid gap-3 text-left">
          <p className="text-center">
            Homeport hit an unexpected problem on this page. Reloading usually helps; if it keeps
            happening, copy the report from Details into an issue.
          </p>
          <ErrorMessage error={error} />
        </div>
      }
      action={
        <div className="flex gap-2">
          <Button variant="outline" onClick={() => window.location.reload()}>
            <RotateCwIcon /> Reload
          </Button>
          <Button onClick={() => navigate('/')}>Back to overview</Button>
        </div>
      }
    />
  );
}
