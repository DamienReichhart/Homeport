import type { RestartPolicy, RunSpec } from '@/shared/api';
import {
  advancedErrors,
  advancedToSpec,
  emptyAdvanced,
  lines,
  type AdvancedErrors,
  type AdvancedForm,
} from './runAdvanced';

export interface PortRow {
  hostPort: string;
  containerPort: string;
  protocol: string;
}

export interface MountRow {
  source: string;
  target: string;
  readOnly: boolean;
}

export interface RunForm {
  image: string;
  name: string;
  command: string;
  /** One `KEY=value` per line. */
  env: string;
  ports: PortRow[];
  mounts: MountRow[];
  network: string;
  restart: RestartPolicy;
  autoRemove: boolean;
  advanced: AdvancedForm;
  /** The container this form was read from (Duplicate, Edit), whose other settings are kept. */
  basedOn: string | null;
  /** The Podman pod to run it in, by name; empty for none. Its ports and network are the pod's. */
  pod: string;
}

export const emptyRunForm = (image = ''): RunForm => ({
  image,
  name: '',
  command: '',
  env: '',
  ports: [],
  mounts: [],
  network: '',
  restart: 'no',
  autoRemove: false,
  advanced: emptyAdvanced(),
  basedOn: null,
  pod: '',
});

/** What can't be read in the form, by field; empty when it can be sent. */
export type RunFormErrors = AdvancedErrors;

export const runFormErrors = (form: RunForm): RunFormErrors => advancedErrors(form.advanced);

const blankToNull = (s: string) => (s.trim() === '' ? null : s.trim());

/**
 * The Run request for a filled-in form; rows left empty are dropped, and so
 * are the ports and network of a container run into a pod, which are the
 * pod's. Call it once `runFormErrors` is empty.
 */
export function toRunSpec(form: RunForm): RunSpec {
  const inPod = form.pod.trim() !== '';
  return {
    image: form.image.trim(),
    name: blankToNull(form.name),
    command: blankToNull(form.command),
    env: lines(form.env),
    ports: form.ports
      .filter((p) => !inPod && p.containerPort.trim() !== '')
      .map((p) => ({
        hostIp: null,
        hostPort: p.hostPort.trim() === '' ? null : Number(p.hostPort),
        containerPort: Number(p.containerPort),
        protocol: p.protocol,
      })),
    mounts: form.mounts
      .filter((m) => m.source.trim() !== '' || m.target.trim() !== '')
      .map((m) => ({ source: m.source.trim(), target: m.target.trim(), readOnly: m.readOnly })),
    network: inPod ? null : blankToNull(form.network),
    restart: form.restart,
    autoRemove: form.autoRemove,
    ...advancedToSpec(form.advanced),
    basedOn: form.basedOn,
  };
}

/** The pod `form` runs the container in, for `containers_run`; `null` for none. */
export const podOf = (form: RunForm): string | null => blankToNull(form.pod);
