import { hostView } from '@/test/fixtures';
import { managedSwarmId, swarmLabel, swarmOfHost } from './naming';

const swarm = { id: 'abcdef1234567890', managers: ['m2', 'm1'], workers: ['w'], reachable: true };

describe('Swarm naming', () => {
  it('names a Swarm after its first manager host Homeport knows', () => {
    expect(swarmLabel(swarm, [hostView('alpha', { id: 'm1' })])).toBe('Swarm of alpha');
    expect(swarmLabel(swarm, [])).toBe('Swarm abcdef123456');
  });

  it('finds the Swarm and role of a host', () => {
    expect(swarmOfHost([swarm], 'm1')).toEqual({ swarm, role: 'manager' });
    expect(swarmOfHost([swarm], 'w')).toEqual({ swarm, role: 'worker' });
    expect(swarmOfHost([swarm], 'other')).toBeNull();
  });

  it('reads the Swarm a manager host manages from its status', () => {
    const manager = hostView('m', {
      id: 'm1',
      status: {
        state: 'connected',
        engineVersion: '29',
        apiVersion: '1.52',
        os: 'linux',
        arch: 'amd64',
        podmanVersion: null,
        containerdVersion: null,
        swarm: {
          swarmId: 's1',
          nodeId: 'n1',
          role: 'manager',
          state: 'active',
          nodeAddr: null,
          managers: [],
          error: null,
        },
      },
    });
    expect(managedSwarmId([manager], 'm1')).toBe('s1');
    expect(managedSwarmId([hostView('x', { id: 'x' })], 'x')).toBeNull();
  });
});
