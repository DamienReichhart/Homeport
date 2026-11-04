import { CircleAlertIcon, RotateCwIcon } from 'lucide-react';
import { toAppError } from '@/shared/api';
import { cn } from 'cn';
import { Alert, AlertDescription, AlertTitle } from '../primitives/alert';
import { Button } from '../primitives/button';
import { showErrorDetails } from './ErrorDetails';
import { HintView } from './HintView';
import { t } from '@/shared/i18n';

/**
 * A failure shown in place: what happened, what to do, and buttons for the
 * raw details and (when the caller can redo the work) another try.
 */
export function ErrorMessage({
  error,
  onRetry,
  className,
}: {
  error: unknown;
  onRetry?: () => void;
  className?: string;
}) {
  const failure = toAppError(error);
  return (
    <Alert variant="destructive" className={cn('min-w-0', className)}>
      <CircleAlertIcon />
      <AlertTitle className="text-pretty">{failure.message}</AlertTitle>
      <AlertDescription className="flex min-w-0 flex-col gap-2">
        {failure.hint && <HintView hint={failure.hint} />}
        <div className="flex gap-2">
          <Button
            type="button"
            variant="outline"
            size="sm"
            onClick={() => showErrorDetails(failure)}
          >
            {t('common.details')}
          </Button>
          {onRetry && (
            <Button type="button" variant="outline" size="sm" onClick={onRetry}>
              <RotateCwIcon />
              {t('common.retry')}
            </Button>
          )}
        </div>
      </AlertDescription>
    </Alert>
  );
}
