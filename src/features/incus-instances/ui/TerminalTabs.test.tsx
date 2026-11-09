import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { resetTerminals, terminals } from '@/test/terminal';
import { ConsoleTab, ExecTab } from './TerminalTabs';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

describe('an instance’s terminals', () => {
  beforeEach(resetTerminals);

  function sessions() {
    const started: { cmd: string; args: Record<string, unknown> }[] = [];
    const input: string[] = [];
    const closed: number[] = [];
    const start = (cmd: string) => (args: Record<string, unknown>) => {
      started.push({ cmd, args });
      return started.length;
    };
    mockCommands({
      ...baseCommands,
      incus_exec_start: start('exec'),
      incus_console_start: start('console'),
      incus_console_log: () => 'Welcome to Alpine Linux 3.21\n',
      incus_terminal_resize: () => null,
      incus_terminal_input: ({ data }) => {
        input.push(data as string);
        return null;
      },
      incus_terminal_close: ({ session }) => {
        closed.push(session as number);
        return null;
      },
    });
    return { started, input, closed };
  }

  it('open a shell, then the command and user asked for', async () => {
    const { started, input, closed } = sessions();
    const user = userEvent.setup();
    renderWithProviders(<ExecTab hostId="i1" project="default" name="web" running />);
    await waitFor(() => expect(started).toHaveLength(1));
    expect(started[0]?.args).toMatchObject({
      project: 'default',
      name: 'web',
      options: { command: null, user: null, cwd: null, cols: 80, rows: 24 },
    });
    sendToChannel(started[0]?.args.onEvent, { type: 'output', data: 'web:~# ' });
    expect(await screen.findByText(/web:~#/)).toBeInTheDocument();
    terminals[0]?.type('ls\r');
    expect(input).toEqual(['ls\r']);

    await user.type(screen.getByLabelText('Command'), 'top');
    await user.type(screen.getByLabelText('User ID'), '1000');
    await user.click(screen.getByRole('button', { name: 'Open' }));
    await waitFor(() => expect(started).toHaveLength(2));
    expect(started[1]?.args).toMatchObject({ options: { command: 'top', user: 1000 } });
    await waitFor(() => expect(closed).toEqual([1]));
  });

  it('show the console’s log, then attach to it', async () => {
    const { started } = sessions();
    renderWithProviders(<ConsoleTab hostId="i1" project="default" name="web" running />);
    expect(await screen.findByText(/Welcome to Alpine/)).toBeInTheDocument();
    await waitFor(() => expect(started.map((s) => s.cmd)).toEqual(['console']));
    sendToChannel(started[0]?.args.onEvent, { type: 'output', data: 'web login: ' });
    expect(await screen.findByText(/web login:/)).toBeInTheDocument();
  });

  it('ask for a running instance first', () => {
    sessions();
    renderWithProviders(<ExecTab hostId="i1" project="default" name="web" running={false} />);
    expect(screen.getByText('Start the instance to run commands in it.')).toBeInTheDocument();
  });
});
