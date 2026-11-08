import type { PodHealth, WorkloadHealth } from '@/shared/api';
import { StatusDot } from '@/shared/ui';
import { podTone, workloadHealth } from '../model/health';

/** A workload's health, or a Pod's status text with its colour. */
export function HealthBadge(
  props: { workload: WorkloadHealth } | { pod: PodHealth; status: string },
) {
  const { tone, label } =
    'workload' in props
      ? workloadHealth(props.workload)
      : { tone: podTone(props.pod), label: props.status };
  return (
    <span className="inline-flex items-center gap-1.5 text-sm">
      <StatusDot tone={tone} />
      {label}
    </span>
  );
}
