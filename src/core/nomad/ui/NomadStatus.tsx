import { StatusDot, type Tone } from '@/shared/ui';
import { statusLabel } from '../model/status';
import { t } from '@/shared/i18n';

/** A Nomad status with its colour, in Homeport's words when it knows it. */
export function NomadStatus({ status, tone }: { status: string; tone: Tone }) {
  const label = statusLabel(status);
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={tone} />
      {label ? t(label) : status}
    </span>
  );
}
