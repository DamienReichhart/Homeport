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

function sessions() {
  const started: { options: Record<string, unknown>; channel: unknown }[] = [];
  const input: string[] = [];
  const closed: number[] = [];
  mockCommands({
    ...baseCommands,
    exec_start: ({ options, onEvent }) => {
      started.push({ options: options as Record<string, unknown>, channel: onEvent });
      return started.length;
    },
    exec_resize: () => null,
    exec_input: ({ data }) => {
      input.push(data as string);
      return null;
    },
    exec_close: ({ session }) => {
      closed.push(session as number);
      return null;
    },
  });
  return { started, input, closed };
}

describe('exec tab', () => {
  beforeEach(resetTerminals);

  it('opens a shell and connects it to the terminal', async () => {
    const { started, input } = sessions();
    renderWithProviders(<ExecTab hostId="h1" containerId="web-1" running />);
    await waitFor(() => expect(started).toHaveLength(1));
    expect(started[0]?.options).toMatchObject({ command: null, user: null, cols: 80, rows: 24 });

    sendToChannel(started[0]?.channel, { type: 'output', data: 'root@web:/# ' });
    expect(await screen.findByText(/root@web:\/#/)).toBeInTheDocument();
    await waitFor(() => expect(terminals[0]?.focus).toHaveBeenCalled());
    terminals[0]?.type('ls\r');
    expect(input).toEqual(['ls\r']);

    sendToChannel(started[0]?.channel, { type: 'exit', code: 0, error: null });
    expect(await screen.findByText(/process exited with code 0/)).toBeInTheDocument();
  });

  it('opens another terminal with the chosen command and user', async () => {
    const { started, closed } = sessions();
    const user = userEvent.setup();
    renderWithProviders(<ExecTab hostId="h1" containerId="web-1" running />);
    await waitFor(() => expect(started).toHaveLength(1));

    await user.click(screen.getByRole('button', { name: 'New terminal' }));
    await user.type(screen.getByLabelText('Command'), 'psql');
    await user.type(screen.getByLabelText('User'), 'postgres');
    await user.click(screen.getByRole('button', { name: 'Open' }));
    await waitFor(() => expect(started).toHaveLength(2));
    expect(started[1]?.options).toMatchObject({ command: 'psql', user: 'postgres' });

    await user.click(screen.getByRole('button', { name: 'Close psql' }));
    await waitFor(() => expect(closed).toEqual([2]));
  });

  it('says when the container has no shell', async () => {
    const { started } = sessions();
    renderWithProviders(<ExecTab hostId="h1" containerId="web-1" running />);
    await waitFor(() => expect(started).toHaveLength(1));
    sendToChannel(started[0]?.channel, {
      type: 'exit',
      code: 127,
      error: {
        code: 'no_shell',
        message: 'web has no shell to open.',
        hint: { text: 'Choose a command the image has, such as its main program.', commands: [] },
        detail: null,
      },
    });
    expect(await screen.findByText(/web has no shell to open\./)).toBeInTheDocument();
    expect(terminals[0]?.text()).toContain('Choose a command the image has');
  });

  it('asks to start a stopped container', () => {
    sessions();
    renderWithProviders(<ExecTab hostId="h1" containerId="web-1" running={false} />);
    expect(screen.getByText('Start the container to open a terminal.')).toBeInTheDocument();
    expect(terminals).toHaveLength(0);
  });
});
