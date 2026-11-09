import type { InstanceSpec, InstanceType } from '@/shared/api';

/** Where the dialog takes the image from. */
export type ImageOrigin = 'remote' | 'local';

/** What the New Instance dialog edits, as plain strings. */
export interface NewInstanceForm {
  name: string;
  type: InstanceType;
  origin: ImageOrigin;
  server: string;
  alias: string;
  /** A local image's fingerprint. */
  fingerprint: string;
  profiles: string[];
  cpu: string;
  memory: string;
  disk: string;
  start: boolean;
}

export const emptyNewInstance = (server: string): NewInstanceForm => ({
  name: '',
  type: 'container',
  origin: 'remote',
  server,
  alias: '',
  fingerprint: '',
  profiles: ['default'],
  cpu: '',
  memory: '',
  disk: '',
  start: true,
});

/** Incus's rule for Instance names: a hostname label. */
export function validName(name: string): boolean {
  return /^[a-zA-Z](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?$/.test(name);
}

/** A size Incus reads: a number and an optional unit such as MiB, GiB or GB. */
export function validSize(size: string): boolean {
  return size.trim() === '' || /^\d+(\.\d+)?\s*([KMGTPE]i?B|B)?$/i.test(size.trim());
}

/** What's still missing or wrong, the first problem only; `null` when it can be made. */
export type FormProblem = 'name' | 'image' | 'cpu' | 'memory' | 'disk';

export function problem(form: NewInstanceForm): FormProblem | null {
  if (!validName(form.name.trim())) return 'name';
  if (form.origin === 'remote' ? !form.alias.trim() || !form.server.trim() : !form.fingerprint)
    return 'image';
  if (form.cpu.trim() !== '' && !/^[1-9]\d*$/.test(form.cpu.trim())) return 'cpu';
  if (!validSize(form.memory)) return 'memory';
  if (!validSize(form.disk)) return 'disk';
  return null;
}

/** Sizes as Incus writes them: "2GiB", without spaces. */
const size = (text: string) => text.replace(/\s+/g, '') || null;

/** The spec the backend makes the Instance from. */
export function toSpec(form: NewInstanceForm): InstanceSpec {
  return {
    name: form.name.trim(),
    type: form.type,
    source:
      form.origin === 'remote'
        ? { kind: 'remote', server: form.server.trim(), alias: form.alias.trim() }
        : { kind: 'local', fingerprint: form.fingerprint },
    profiles: form.profiles,
    cpu: form.cpu.trim() ? Number.parseInt(form.cpu, 10) : null,
    memory: size(form.memory),
    disk: size(form.disk),
    start: form.start,
  };
}
