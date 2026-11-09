import type { IncusInstance, InstanceAction, InstanceStatus, InstanceType } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';

export const STATUS_LABELS: Record<InstanceStatus, MessageKey> = {
  running: 'incus.status.running',
  stopped: 'incus.status.stopped',
  frozen: 'incus.status.frozen',
  starting: 'incus.status.starting',
  stopping: 'incus.status.stopping',
  freezing: 'incus.status.freezing',
  error: 'incus.status.error',
  unknown: 'incus.status.unknown',
};

export const TYPE_LABELS: Record<InstanceType, MessageKey> = {
  container: 'incus.type.container',
  'virtual-machine': 'incus.type.virtualMachine',
};

export function statusTone(status: InstanceStatus): Tone {
  switch (status) {
    case 'running':
      return 'success';
    case 'frozen':
    case 'freezing':
      return 'info';
    case 'starting':
    case 'stopping':
      return 'warning';
    case 'error':
      return 'danger';
    case 'stopped':
    case 'unknown':
      return 'muted';
  }
}

/** The actions that make sense in `status`, the main one first. */
export function instanceActions(status: InstanceStatus): InstanceAction[] {
  switch (status) {
    case 'running':
      return ['stop', 'restart', 'freeze'];
    case 'frozen':
      return ['unfreeze', 'stop'];
    case 'stopped':
    case 'error':
      return ['start'];
    default:
      return [];
  }
}

/** Whether an Instance runs, frozen or not: what deleting it must stop first. */
export function isUp(status: InstanceStatus): boolean {
  return status !== 'stopped' && status !== 'error';
}

export type TypeFilter = 'all' | InstanceType;

/** The Instances a list shows for `filter`. */
export function filterByType(instances: IncusInstance[], filter: TypeFilter): IncusInstance[] {
  return filter === 'all' ? instances : instances.filter((i) => i.type === filter);
}

/** Seconds since the epoch of an RFC 3339 time, for sorting and relative times. */
export function seconds(time: string | null): number {
  return time ? Math.floor(Date.parse(time) / 1000) : 0;
}

/** CPU time in nanoseconds, as seconds with one decimal. */
export function cpuSeconds(nanoseconds: number): string {
  return (nanoseconds / 1e9).toFixed(1);
}
