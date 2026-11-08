import { StatusDot } from '@/shared/ui';
import { statusTone } from '../model/status';

/** A release's or revision's status as helm names it, with its colour. */
export function ReleaseStatus({ status }: { status: string }) {
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={statusTone(status)} />
      {status}
    </span>
  );
}
