import type { HostStatus } from '@/shared/api';
import { toAppError } from '@/shared/api';
import { HintView, showErrorDetails } from '@/shared/ui';
import { Button } from '@/shared/ui/button';
import { statusError } from '../status';

/** What to do about a Host that can't be used, and the raw cause on demand. */
export function HostProblem({ status }: { status: HostStatus }) {
  const error = statusError(status);
  if (!error) return null;
  return (
    <div className="grid justify-items-start gap-2 text-left">
      {error.hint && <HintView hint={error.hint} />}
      <Button variant="outline" size="sm" onClick={() => showErrorDetails(toAppError(error))}>
        Details
      </Button>
    </div>
  );
}
