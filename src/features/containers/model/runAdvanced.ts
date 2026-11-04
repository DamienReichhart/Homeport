import type { GpuSpec, HealthcheckSpec, LogSpec, RunSpec } from '@/shared/api';
import { parseDuration, parsePositive, parseSize } from './parse';

/** The Advanced area of the Run dialog, one object per section. */
export interface ResourcesForm {
  cpus: string;
  cpuShares: string;
  memory: string;
  memoryReservation: string;
  /** A size, or `-1` for unlimited swap. */
  memorySwap: string;
}

export interface DeviceRow {
  hostPath: string;
  containerPath: string;
  permissions: string;
}

export interface SecurityForm {
  capAdd: string[];
  capDrop: string[];
  privileged: boolean;
  devices: DeviceRow[];
  gpus: { mode: 'none' | 'all' | 'count' | 'ids'; count: string; ids: string };
}

export interface HealthcheckForm {
  /** Keep the image's healthcheck, define one, or turn it off. */
  mode: 'image' | 'custom' | 'disabled';
  command: string;
  interval: string;
  timeout: string;
  retries: string;
  startPeriod: string;
}

export interface IdentityForm {
  hostname: string;
  domainname: string;
  /** One entry per line in each list. */
  dns: string;
  dnsSearch: string;
  extraHosts: string;
}

export interface LoggingForm {
  /** Empty for the Engine's default driver. */
  driver: string;
  /** One `key=value` per line. */
  options: string;
}

export interface AdvancedForm {
  resources: ResourcesForm;
  /** One `key=value` per line. */
  labels: string;
  security: SecurityForm;
  healthcheck: HealthcheckForm;
  identity: IdentityForm;
  logging: LoggingForm;
}

export const emptyAdvanced = (): AdvancedForm => ({
  resources: { cpus: '', cpuShares: '', memory: '', memoryReservation: '', memorySwap: '' },
  labels: '',
  security: {
    capAdd: [],
    capDrop: [],
    privileged: false,
    devices: [],
    gpus: { mode: 'none', count: '1', ids: '' },
  },
  healthcheck: {
    mode: 'image',
    command: '',
    interval: '',
    timeout: '',
    retries: '',
    startPeriod: '',
  },
  identity: { hostname: '', domainname: '', dns: '', dnsSearch: '', extraHosts: '' },
  logging: { driver: '', options: '' },
});

/** Fields whose text the frontend parses, and so explains before sending. */
export type AdvancedField =
  | 'cpus'
  | 'cpuShares'
  | 'memory'
  | 'memoryReservation'
  | 'memorySwap'
  | 'gpuCount'
  | 'interval'
  | 'timeout'
  | 'retries'
  | 'startPeriod';

export type AdvancedErrors = Partial<Record<AdvancedField, string>>;

/** Non-empty lines, trimmed, without `#` comments. */
export const lines = (text: string) =>
  text
    .split('\n')
    .map((l) => l.trim())
    .filter((l) => l !== '' && !l.startsWith('#'));

const swap = (text: string) => (text.trim() === '-1' ? -1 : parseSize(text));

/** What can't be read in the Advanced area, by field. Empty means sendable. */
export function advancedErrors(form: AdvancedForm): AdvancedErrors {
  const errors: AdvancedErrors = {};
  const check = (field: AdvancedField, value: unknown, message: string) => {
    if (value === undefined) errors[field] = message;
  };
  const r = form.resources;
  const size = 'Write a size like 512m or 2g.';
  check('cpus', parsePositive(r.cpus), 'Write a number of CPUs like 0.5 or 2.');
  check('cpuShares', parsePositive(r.cpuShares, true), 'Write a whole number like 512.');
  check('memory', parseSize(r.memory), size);
  check('memoryReservation', parseSize(r.memoryReservation), size);
  check('memorySwap', swap(r.memorySwap), 'Write a size like 1g, or -1 for unlimited swap.');
  if (form.security.gpus.mode === 'count')
    check(
      'gpuCount',
      parsePositive(form.security.gpus.count, true),
      'Write how many GPUs, like 1.',
    );
  const h = form.healthcheck;
  if (h.mode === 'custom') {
    const duration = 'Write a duration like 30s or 1m30s.';
    check('interval', parseDuration(h.interval), duration);
    check('timeout', parseDuration(h.timeout), duration);
    check('startPeriod', parseDuration(h.startPeriod), duration);
    check('retries', parsePositive(h.retries, true), 'Write a whole number like 3.');
  }
  return errors;
}

const nullIfMissing = <T>(value: T | null | undefined): T | null => value ?? null;

function gpus(form: SecurityForm['gpus']): GpuSpec | null {
  switch (form.mode) {
    case 'none':
      return null;
    case 'all':
      return { kind: 'all' };
    case 'count':
      return { kind: 'count', count: parsePositive(form.count, true) ?? 1 };
    case 'ids':
      return { kind: 'ids', ids: form.ids.split(/[\s,]+/).filter(Boolean) };
  }
}

function healthcheck(form: HealthcheckForm): HealthcheckSpec | null {
  if (form.mode === 'image') return null;
  return {
    command: form.command.trim(),
    intervalS: nullIfMissing(parseDuration(form.interval)),
    timeoutS: nullIfMissing(parseDuration(form.timeout)),
    retries: nullIfMissing(parsePositive(form.retries, true)),
    startPeriodS: nullIfMissing(parseDuration(form.startPeriod)),
    disabled: form.mode === 'disabled',
  };
}

function log(form: LoggingForm): LogSpec | null {
  if (form.driver.trim() === '') return null;
  return { driver: form.driver.trim(), options: lines(form.options) };
}

type AdvancedSpec = Omit<
  RunSpec,
  'image' | 'name' | 'command' | 'env' | 'ports' | 'mounts' | 'network' | 'restart' | 'autoRemove'
>;

/** The Run request's advanced fields; call once `advancedErrors` is empty. */
export function advancedToSpec(form: AdvancedForm): AdvancedSpec {
  const r = form.resources;
  const blank = (s: string) => (s.trim() === '' ? null : s.trim());
  return {
    cpus: nullIfMissing(parsePositive(r.cpus)),
    cpuShares: nullIfMissing(parsePositive(r.cpuShares, true)),
    memory: nullIfMissing(parseSize(r.memory)),
    memoryReservation: nullIfMissing(parseSize(r.memoryReservation)),
    memorySwap: nullIfMissing(swap(r.memorySwap)),
    labels: lines(form.labels),
    capAdd: form.security.capAdd,
    capDrop: form.security.capDrop,
    privileged: form.security.privileged,
    devices: form.security.devices
      .filter((d) => d.hostPath.trim() !== '')
      .map((d) => ({
        hostPath: d.hostPath.trim(),
        containerPath: blank(d.containerPath),
        permissions: d.permissions.trim() || 'rwm',
      })),
    gpus: gpus(form.security.gpus),
    healthcheck: healthcheck(form.healthcheck),
    hostname: blank(form.identity.hostname),
    domainname: blank(form.identity.domainname),
    dns: lines(form.identity.dns),
    dnsSearch: lines(form.identity.dnsSearch),
    extraHosts: lines(form.identity.extraHosts),
    log: log(form.logging),
  };
}
