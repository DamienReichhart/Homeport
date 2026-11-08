import type { RunSpec } from '@/shared/api';
import { formatDuration, formatSize } from './parse';
import { emptyAdvanced, type AdvancedForm } from './runAdvanced';
import type { RunForm } from './runForm';

const text = (value: number | null, format: (n: number) => string = String) =>
  value === null ? '' : format(value);

function advanced(spec: RunSpec): AdvancedForm {
  const empty = emptyAdvanced();
  const health = spec.healthcheck;
  const gpus = spec.gpus;
  return {
    resources: {
      cpus: text(spec.cpus),
      cpuShares: text(spec.cpuShares),
      memory: text(spec.memory, formatSize),
      memoryReservation: text(spec.memoryReservation, formatSize),
      memorySwap: spec.memorySwap === -1 ? '-1' : text(spec.memorySwap, formatSize),
    },
    labels: spec.labels.join('\n'),
    security: {
      capAdd: spec.capAdd,
      capDrop: spec.capDrop,
      privileged: spec.privileged,
      devices: spec.devices.map((d) => ({
        hostPath: d.hostPath,
        containerPath: d.containerPath ?? '',
        permissions: d.permissions,
      })),
      gpus: !gpus
        ? empty.security.gpus
        : gpus.kind === 'all'
          ? { ...empty.security.gpus, mode: 'all' }
          : gpus.kind === 'count'
            ? { ...empty.security.gpus, mode: 'count', count: String(gpus.count) }
            : { ...empty.security.gpus, mode: 'ids', ids: gpus.ids.join(', ') },
    },
    healthcheck: !health
      ? empty.healthcheck
      : {
          mode: health.disabled ? 'disabled' : 'custom',
          command: health.command,
          interval: text(health.intervalS, formatDuration),
          timeout: text(health.timeoutS, formatDuration),
          retries: text(health.retries),
          startPeriod: text(health.startPeriodS, formatDuration),
        },
    identity: {
      hostname: spec.hostname ?? '',
      domainname: spec.domainname ?? '',
      dns: spec.dns.join('\n'),
      dnsSearch: spec.dnsSearch.join('\n'),
      extraHosts: spec.extraHosts.join('\n'),
    },
    logging: { driver: spec.log?.driver ?? '', options: (spec.log?.options ?? []).join('\n') },
  };
}

/** The Run form showing `spec`, as read back from a container. */
export function fromRunSpec(spec: RunSpec): RunForm {
  return {
    image: spec.image,
    name: spec.name ?? '',
    command: spec.command ?? '',
    env: spec.env.join('\n'),
    ports: spec.ports.map((p) => ({
      hostPort: p.hostPort === null ? '' : String(p.hostPort),
      containerPort: String(p.containerPort),
      protocol: p.protocol,
    })),
    mounts: spec.mounts.map((m) => ({ ...m })),
    network: spec.network ?? '',
    restart: spec.restart,
    autoRemove: spec.autoRemove,
    advanced: advanced(spec),
    basedOn: spec.basedOn,
    // Duplicate and Edit never run into a pod.
    pod: '',
  };
}

/** Whether the Advanced area has anything set, so it opens on its own. */
export const hasAdvanced = (form: RunForm) =>
  JSON.stringify(form.advanced) !== JSON.stringify(emptyAdvanced());
