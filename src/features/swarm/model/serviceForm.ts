import type { MountRow, PortRow } from '@/features/containers';
import type { ServiceChange, ServiceSpecInput, ServiceView } from '@/shared/api';
import { parseReplicas } from './services';

/** A secret or config picked for a service, and where it's mounted. */
export interface ObjectPick {
  name: string;
  target: string;
}

/** The Create service dialog's fields, as typed. */
export interface ServiceForm {
  name: string;
  image: string;
  mode: 'replicated' | 'global';
  replicas: string;
  /** One `KEY=value` per line. */
  env: string;
  command: string;
  ports: PortRow[];
  /** Publish on the node running each task instead of the routing mesh. */
  hostMode: boolean;
  networks: string[];
  mounts: MountRow[];
  secrets: ObjectPick[];
  configs: ObjectPick[];
  restart: 'default' | 'none' | 'on-failure' | 'any';
}

export const emptyServiceForm = (): ServiceForm => ({
  name: '',
  image: '',
  mode: 'replicated',
  replicas: '1',
  env: '',
  command: '',
  ports: [],
  hostMode: false,
  networks: [],
  mounts: [],
  secrets: [],
  configs: [],
  restart: 'default',
});

/** The non-blank lines of a textarea, trimmed. */
export const envLines = (text: string) =>
  text
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l !== '');

const port = (text: string) => {
  const n = Number(text.trim());
  return Number.isInteger(n) && n > 0 && n < 65536 ? n : null;
};

/**
 * The input the backend checks and turns into a spec; `null` while the
 * replicas aren't a number. Blank port and mount rows are dropped.
 */
export function toServiceInput(form: ServiceForm): ServiceSpecInput | null {
  const replicas = parseReplicas(form.replicas);
  if (form.mode === 'replicated' && replicas === null) return null;
  return {
    name: form.name.trim(),
    image: form.image.trim(),
    mode:
      form.mode === 'global' ? { kind: 'global' } : { kind: 'replicated', replicas: replicas ?? 1 },
    env: envLines(form.env),
    command: form.command.trim() || null,
    ports: form.ports
      .filter((p) => p.containerPort.trim() !== '' || p.hostPort.trim() !== '')
      .map((p) => ({
        target: port(p.containerPort) ?? 0,
        published: port(p.hostPort),
        protocol: p.protocol,
        hostMode: form.hostMode,
      })),
    networks: form.networks,
    mounts: form.mounts
      .filter((m) => m.source.trim() !== '' || m.target.trim() !== '')
      .map((m) => ({ source: m.source.trim(), target: m.target.trim(), readOnly: m.readOnly })),
    secrets: form.secrets.map((s) => ({ name: s.name, target: s.target.trim() || null })),
    configs: form.configs.map((c) => ({ name: c.name, target: c.target.trim() || null })),
    restart: form.restart === 'default' ? null : form.restart,
  };
}

/** What the Edit dialog shows: the three fields it changes. */
export interface EditForm {
  image: string;
  env: string;
  replicas: string;
  redeploy: boolean;
}

/** The service's image and environment (from its inspect document) and replicas. */
export function editForm(service: ServiceView, inspect: unknown): EditForm {
  const env = (inspect as { Spec?: { TaskTemplate?: { ContainerSpec?: { Env?: string[] } } } })
    ?.Spec?.TaskTemplate?.ContainerSpec?.Env;
  return {
    image: service.image,
    env: (env ?? []).join('\n'),
    replicas: String(service.replicas ?? ''),
    redeploy: false,
  };
}

/** Only what changed between `initial` and `form`; `null` while the replicas aren't a number. */
export function toServiceChange(
  service: ServiceView,
  initial: EditForm,
  form: EditForm,
): ServiceChange | null {
  const replicated = service.mode === 'replicated';
  const replicas = replicated ? parseReplicas(form.replicas) : null;
  if (replicated && replicas === null) return null;
  const env = envLines(form.env);
  return {
    image: form.image.trim() !== initial.image ? form.image.trim() : null,
    env: env.join('\n') !== envLines(initial.env).join('\n') ? env : null,
    replicas: replicated && replicas !== service.replicas ? replicas : null,
    redeploy: form.redeploy,
  };
}

/** Whether a change would do anything. */
export const changesSomething = (c: ServiceChange) =>
  c.image !== null || c.env !== null || c.replicas !== null || c.redeploy;
