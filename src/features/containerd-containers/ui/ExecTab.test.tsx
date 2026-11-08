import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { resetTerminals, terminals } from '@/test/terminal';
import { ExecTab } from './ExecTab';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

describe('a containerd container’s terminal', () => {
  beforeEach(resetTerminals);

  function sessions() {
    const started: { args: Record<string, unknown>; channel: unknown }[] = [];
    const input: string[] = [];
    const closed: number[] = [];
    mockCommands({
      ...baseCommands,
      containerd_exec_start: (args) => {
        started.push({ args, channel: args.onEvent });
        return started.length;
      },
      containerd_exec_resize: () => null,
      containerd_exec_input: ({ data }) => {
        input.push(data as string);
        return null;
      },
      containerd_exec_close: ({ session }) => {
        closed.push(session as number);
        return null;
      },
    });
    return { started, input, closed };
  }

  it('opens a shell, then the command and user asked for', async () => {
    const { started, input, closed } = sessions();
    const user = userEvent.setup();
    renderWithProviders(<ExecTab hostId="c1" namespace="default" id="web" running />);
    await waitFor(() => expect(started).toHaveLength(1));
    expect(started[0]?.args).toMatchObject({
      namespace: 'default',
      id: 'web',
      options: { command: null, user: null, cols: 80, rows: 24 },
    });
    sendToChannel(started[0]?.channel, { type: 'output', data: '/ # ' });
    expect(await screen.findByText(/\/ #/)).toBeInTheDocument();
    terminals[0]?.type('ls\r');
    expect(input).toEqual(['ls\r']);

    await user.type(screen.getByLabelText('Command'), 'top');
    await user.type(screen.getByLabelText('User ID'), '0');
    await user.click(screen.getByRole('button', { name: 'Open again' }));
    await waitFor(() => expect(started).toHaveLength(2));
    expect(started[1]?.args).toMatchObject({ options: { command: 'top', user: '0' } });
    await waitFor(() => expect(closed).toEqual([1]));
  });

  it('asks for a running task first', () => {
    sessions();
    renderWithProviders(<ExecTab hostId="c1" namespace="default" id="web" running={false} />);
    expect(screen.getByText('Start the container to open a terminal in it.')).toBeInTheDocument();
  });
});
