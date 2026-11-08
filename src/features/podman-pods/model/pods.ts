import type { PodmanPod, PodmanPodAction, PodmanPodSpec } from '@/shared/api';
import type { PortRow } from '@/features/containers';
import type { Tone } from '@/shared/ui';
import type { MessageKey } from '@/shared/i18n';

/** A pod's status as a colour: running, partly running, paused or stopped. */
export function podTone(status: string): Tone {
  switch (status) {
    case 'running':
      return 'success';
    case 'degraded':
      return 'warning';
    case 'paused':
      return 'info';
    case 'dead':
    case 'error':
      return 'danger';
    default:
      return 'muted';
  }
}

/** The catalogue key of a pod's status. */
export function podStatusLabel(status: string): MessageKey {
  switch (status) {
    case 'running':
      return 'podmanpods.status.running';
    case 'degraded':
      return 'podmanpods.status.degraded';
    case 'paused':
      return 'podmanpods.status.paused';
    case 'created':
      return 'podmanpods.status.created';
    case 'dead':
    case 'error':
      return 'podmanpods.status.failed';
    default:
      return 'podmanpods.status.stopped';
  }
}

/** The pod's own containers, without its infra container. */
export const members = (pod: PodmanPod) => pod.containers.filter((c) => !c.infra);

/** How many of the pod's own containers are running. */
export const runningMembers = (pod: PodmanPod) =>
  members(pod).filter((c) => c.state === 'running').length;

/** Whether any container of the pod runs, so stopping, restarting and pausing apply. */
export const isUp = (status: string) => status === 'running' || status === 'degraded';

/** The actions that apply to a pod in `status`, in the order they're shown. */
export function podActions(status: string): PodmanPodAction[] {
  if (status === 'paused') return ['unpause', 'stop'];
  if (isUp(status)) return ['stop', 'restart', 'pause'];
  return ['start'];
}

/** The **New pod** dialog's fields, as typed. */
export interface PodForm {
  name: string;
  ports: PortRow[];
  /** One `key=value` per line. */
  labels: string;
}

export const emptyPodForm = (): PodForm => ({ name: '', ports: [], labels: '' });

/** What's wrong with the form, by field; empty when it can be sent. */
export interface PodFormErrors {
  name?: MessageKey;
  ports?: MessageKey;
  labels?: MessageKey;
}

const NAME = /^[A-Za-z0-9][A-Za-z0-9_.-]*$/;

const portNumber = (text: string) => {
  const n = Number(text.trim());
  return Number.isInteger(n) && n >= 1 && n <= 65535 ? n : null;
};

const labelLines = (text: string) =>
  text
    .split('\n')
    .map((l) => l.trim())
    .filter(Boolean);

/** Rows with nothing typed in them, which are ignored. */
const filled = (ports: PortRow[]) =>
  ports.filter((p) => p.containerPort.trim() !== '' || p.hostPort.trim() !== '');

export function podFormErrors(form: PodForm): PodFormErrors {
  const errors: PodFormErrors = {};
  if (form.name.trim() !== '' && !NAME.test(form.name.trim()))
    errors.name = 'podmanpods.form.nameInvalid';
  const badPort = filled(form.ports).some(
    (p) =>
      portNumber(p.containerPort) === null ||
      (p.hostPort.trim() !== '' && portNumber(p.hostPort) === null),
  );
  if (badPort) errors.ports = 'podmanpods.form.portInvalid';
  if (labelLines(form.labels).some((l) => !/^[^=\s][^=]*=/.test(l)))
    errors.labels = 'podmanpods.form.labelInvalid';
  return errors;
}

/** Whether the form can be sent: a name, and nothing wrong. */
export const canCreatePod = (form: PodForm) =>
  form.name.trim() !== '' && Object.keys(podFormErrors(form)).length === 0;

/** The request for a filled-in form; empty port rows are dropped. */
export function toPodSpec(form: PodForm): PodmanPodSpec {
  return {
    name: form.name.trim(),
    ports: filled(form.ports).map((p) => ({
      hostIp: null,
      hostPort: p.hostPort.trim() === '' ? null : Number(p.hostPort),
      containerPort: Number(p.containerPort),
      protocol: p.protocol,
    })),
    labels: labelLines(form.labels),
  };
}

/** A pod's creation time as Unix seconds, for relative times. */
export const createdSeconds = (pod: PodmanPod) => Math.floor(Date.parse(pod.created) / 1000) || 0;
