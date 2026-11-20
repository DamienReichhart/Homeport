import { StatusDot } from '@/shared/ui';
import { cn } from '@/shared/lib';
import { statusTone } from '../model/status';

/** A release's or revision's status as helm names it, with its colour; a failed one in red. */
export function ReleaseStatus({ status }: { status: string }) {
  const tone = statusTone(status);
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 text-sm',
        tone === 'danger' && 'text-destructive font-medium',
      )}
    >
      <StatusDot tone={tone} />
      {status}
    </span>
  );
}
