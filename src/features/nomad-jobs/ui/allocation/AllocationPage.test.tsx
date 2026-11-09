import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { nomadAllocation, nomadCommands } from '@/test/nomad';
import { renderApp } from '@/test/render';
import { resetTerminals, terminals } from '@/test/terminal';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

const ALLOC = 'a1b2c3d4-0000-0000-0000-000000000009';

function setup() {
  const calls: { command: string; args: Record<string, unknown> }[] = [];
  const record =
    (command: string, answer: unknown = null) =>
    (args: Record<string, unknown>) => {
      calls.push({ command, args });
      return answer;
    };
  mockCommands({
    ...baseCommands,
    ...nomadCommands,
    clusters_nomad_allocation: () => ({
      allocation: nomadAllocation('web.api[0]', {
        id: ALLOC,
        tasks: [
          { name: 'server', state: 'running', failed: false, restarts: 1, started: null },
          { name: 'sidecar', state: 'dead', failed: false, restarts: 0, started: null },
        ],
      }),
      clientDescription: 'Tasks are running',
      events: [{ task: 'server', kind: 'Started', message: 'Task started by client', time: null }],
    }),
    clusters_nomad_logs_subscribe: record('logs', 1),
    clusters_nomad_exec_start: record('exec', 7),
    clusters_nomad_exec_resize: record('resize'),
    clusters_nomad_exec_input: record('input'),
    clusters_nomad_exec_close: record('close'),
  });
  return calls;
}

describe('a Nomad allocation', () => {
  beforeEach(resetTerminals);

  it('shows its tasks and their events', async () => {
    setup();
    renderApp(`/nomad/allocations/${ALLOC}`);
    expect(await screen.findByText('Task started by client')).toBeInTheDocument();
    expect(screen.getByText('Tasks are running')).toBeInTheDocument();
    expect(screen.getByText('1 restart')).toBeInTheDocument();
  });

  it('streams the first task’s logs', async () => {
    const calls = setup();
    renderApp(`/nomad/allocations/${ALLOC}/logs`);
    await waitFor(() => expect(calls.some((c) => c.command === 'logs')).toBe(true));
    expect(calls.find((c) => c.command === 'logs')?.args).toMatchObject({
      id: 'n1',
      allocId: ALLOC,
      task: 'server',
    });
  });

  it('opens /bin/sh in a running task, and reports how it exited', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp(`/nomad/allocations/${ALLOC}/shell`);
    await waitFor(() => expect(calls.some((c) => c.command === 'exec')).toBe(true));
    const exec = calls.find((c) => c.command === 'exec');
    expect(exec?.args).toMatchObject({
      allocId: ALLOC,
      options: { task: 'server', command: '/bin/sh' },
    });
    await waitFor(() => expect(terminals).toHaveLength(1));
    sendToChannel(exec?.args.onEvent, { type: 'output', data: 'hello from nomad' });
    sendToChannel(exec?.args.onEvent, { type: 'exit', code: 0, error: null });
    await waitFor(() => expect(terminals[0]?.text()).toContain('exited with code 0'));
    expect(terminals[0]?.text()).toContain('hello from nomad');
    expect(screen.getByRole('button', { name: 'Reconnect' })).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Overview' }));
    await waitFor(() => expect(calls.some((c) => c.command === 'close')).toBe(true));
  });
});
