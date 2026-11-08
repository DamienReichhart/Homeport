import type { ContainerdContainer, TaskAction, TaskState } from '@/shared/api';
import type { Tone } from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';

export const STATE_LABELS: Record<TaskState, MessageKey> = {
  none: 'containerd.state.none',
  created: 'containerd.state.created',
  running: 'containerd.state.running',
  paused: 'containerd.state.paused',
  pausing: 'containerd.state.pausing',
  stopped: 'containerd.state.stopped',
  unknown: 'containerd.state.unknown',
};

export function stateTone(state: TaskState): Tone {
  switch (state) {
    case 'running':
      return 'success';
    case 'paused':
    case 'pausing':
      return 'warning';
    case 'stopped':
    case 'unknown':
      return 'muted';
    case 'created':
    case 'none':
      return 'info';
  }
}

/** Whether a container's task runs, paused or not: what removing must kill. */
export function isUp(state: TaskState): boolean {
  return state === 'running' || state === 'paused' || state === 'pausing';
}

/** The task actions that make sense in `state`, the main one first. */
export function taskActions(state: TaskState): TaskAction[] {
  switch (state) {
    case 'running':
      return ['stop', 'pause'];
    case 'paused':
    case 'pausing':
      return ['resume', 'stop'];
    default:
      return ['start'];
  }
}

/** Containers the kubelet manages, whose stops and removals it undoes. */
export function managedByKubernetes(container: ContainerdContainer): boolean {
  return container.createdBy === 'kubernetes';
}

/** The containers a list shows: Pods' pause containers only when asked. */
export function visibleContainers(
  containers: ContainerdContainer[],
  showSandboxes: boolean,
): ContainerdContainer[] {
  return showSandboxes ? containers : containers.filter((c) => !c.sandbox);
}

/** Seconds since the epoch of an RFC 3339 time, for sorting and relative times. */
export function seconds(time: string | null): number {
  return time ? Math.floor(Date.parse(time) / 1000) : 0;
}
