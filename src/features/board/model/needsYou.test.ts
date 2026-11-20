import type { Container } from '@/shared/api';
import type { BoardRow } from './board';
import { exitCode, needsYou } from './needsYou';

const row = (
  name: string,
  state: string,
  status: string,
  health: string | null = null,
): BoardRow => ({
  key: `h/${name}`,
  hostId: 'h',
  hostName: 'prod',
  container: { id: name, name, state, status, health } as Container,
});

describe('needs you', () => {
  it('reads the exit code from the Engine status', () => {
    expect(exitCode('Exited (137) 3 hours ago')).toBe(137);
    expect(exitCode('Up 2 days')).toBeNull();
  });

  it('lists what needs attention, red first, and nothing healthy', () => {
    const needs = needsYou({
      hosts: [
        { id: 'a', name: 'a', unreachable: null },
        { id: 'b', name: 'b', unreachable: 'timed out' },
      ],
      clusters: [],
      rows: [
        row('ok', 'running', 'Up 2 days (healthy)', 'healthy'),
        row('crashed', 'exited', 'Exited (1) 2 hours ago'),
        row('stopped', 'exited', 'Exited (0) 2 hours ago'),
        row('sigterm', 'exited', 'Exited (143) 2 hours ago'),
        row('sick', 'running', 'Up 1 hour (unhealthy)', 'unhealthy'),
        row('looping', 'restarting', 'Restarting (1) 5 seconds ago'),
      ],
    });
    expect(needs.map((n) => [n.key, n.tone])).toEqual([
      ['host/b', 'danger'],
      ['h/sick', 'danger'],
      ['h/crashed', 'warning'],
      ['h/looping', 'warning'],
    ]);
  });
});
