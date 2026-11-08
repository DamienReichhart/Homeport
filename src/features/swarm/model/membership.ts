import type { HostConnection } from '@/shared/api';

const IPV4 = /^(\d{1,3})(\.\d{1,3}){3}$/;

/** Whether `host` is an IP address rather than a name. */
export const isIp = (host: string) =>
  IPV4.test(host) || (host.includes(':') && !host.includes('.'));

/**
 * The address to advertise when a Host starts a Swarm: the one Homeport
 * reaches it at when that's an IP, else empty, which lets the Engine pick.
 */
export function advertiseFromConnection(connection: HostConnection): string {
  switch (connection.kind) {
    case 'ssh':
      return isIp(connection.host) ? connection.host : '';
    case 'tcp':
    case 'tls': {
      const host = connection.address.replace(/:\d+$/, '').replace(/^\[|\]$/g, '');
      return isIp(host) ? host : '';
    }
    default:
      return '';
  }
}

/** What leaving the Swarm costs: nothing, a Force, or the whole Swarm. */
export type LeaveRisk = 'worker' | 'manager' | 'lastManager';

/** The risk of a Host leaving, from its role and how many managers the Swarm has. */
export function leaveRisk(role: 'manager' | 'worker', managers: number | undefined): LeaveRisk {
  if (role === 'worker') return 'worker';
  return managers !== undefined && managers <= 1 ? 'lastManager' : 'manager';
}

/** Whether the typed confirmation lets a last manager leave. */
export const confirmsLeave = (typed: string, hostName: string) => typed.trim() === hostName;
