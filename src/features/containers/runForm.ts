import type { RestartPolicy, RunSpec } from '@/shared/api';

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
});

const blankToNull = (s: string) => (s.trim() === '' ? null : s.trim());

/** The Run request for a filled-in form; rows left empty are dropped. */
export function toRunSpec(form: RunForm): RunSpec {
  return {
    image: form.image.trim(),
    name: blankToNull(form.name),
    command: blankToNull(form.command),
    env: form.env
      .split('\n')
      .map((l) => l.trim())
      .filter((l) => l !== '' && !l.startsWith('#')),
    ports: form.ports
      .filter((p) => p.containerPort.trim() !== '')
      .map((p) => ({
        hostIp: null,
        hostPort: p.hostPort.trim() === '' ? null : Number(p.hostPort),
        containerPort: Number(p.containerPort),
        protocol: p.protocol,
      })),
    mounts: form.mounts
      .filter((m) => m.source.trim() !== '' || m.target.trim() !== '')
      .map((m) => ({ source: m.source.trim(), target: m.target.trim(), readOnly: m.readOnly })),
    network: blankToNull(form.network),
    restart: form.restart,
    autoRemove: form.autoRemove,
  };
}
