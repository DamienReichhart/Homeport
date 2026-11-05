import { toast } from 'sonner';
import { toAppError } from '@/shared/api';
import { showErrorDetails } from './ErrorDetails';
import { t } from '@/shared/i18n';

/** How long an error toast stays up; hovering it pauses the countdown. */
const ERROR_TOAST_MS = 10_000;

/**
 * Reports a failure as a toast: the message, the hint's advice and a Details
 * action. `title` names what failed when the message alone wouldn't say.
 */
export function toastError(error: unknown, title?: string) {
  const failure = toAppError(error);
  toast.error(title ?? failure.message, {
    description: title ? failure.message : failure.hint?.text,
    duration: ERROR_TOAST_MS,
    action: { label: t('common.details'), onClick: () => showErrorDetails(failure) },
  });
}
