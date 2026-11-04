import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { resetTerminals, terminals } from '@/test/terminal';
import { LogViewer } from './LogViewer';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

const source = [{ containerId: 'web-1', label: null }];

function streaming() {
  const channels: unknown[] = [];
  const cancelled: number[] = [];
  mockCommands({
    ...baseCommands,
    logs_subscribe: ({ onEvent }) => {
      channels.push(onEvent);
      return channels.length;
    },
    stream_cancel: ({ id }) => {
      cancelled.push(id as number);
      return null;
    },
  });
  return { channels, cancelled };
}

describe('log viewer', () => {
  beforeEach(resetTerminals);

  it('writes the lines pushed through the stream', async () => {
    const { channels } = streaming();
    renderWithProviders(<LogViewer hostId="h1" sources={source} />);
    await waitFor(() => expect(channels).toHaveLength(1));
    sendToChannel(channels[0], {
      type: 'lines',
      lines: [
        { source: null, stream: 'stdout', text: 'listening on :80' },
        { source: null, stream: 'stderr', text: 'warning: slow' },
      ],
    });
    expect(await screen.findByText(/listening on :80/)).toBeInTheDocument();
    expect(terminals[0]?.text()).toContain('warning: slow');
  });

  it('stops the stream and starts another when the options change', async () => {
    const { channels, cancelled } = streaming();
    const user = userEvent.setup();
    renderWithProviders(<LogViewer hostId="h1" sources={source} />);
    await waitFor(() => expect(channels).toHaveLength(1));
    await user.click(screen.getByRole('button', { name: 'Timestamps' }));
    await waitFor(() => expect(channels).toHaveLength(2));
    expect(cancelled).toEqual([1]);
  });

  it('stops the stream on unmount', async () => {
    const { channels, cancelled } = streaming();
    const view = renderWithProviders(<LogViewer hostId="h1" sources={source} />);
    await waitFor(() => expect(channels).toHaveLength(1));
    view.unmount();
    expect(cancelled).toEqual([1]);
  });

  it('searches the terminal from the find box', async () => {
    streaming();
    const user = userEvent.setup();
    renderWithProviders(<LogViewer hostId="h1" sources={source} />);
    await user.type(screen.getByRole('textbox', { name: 'Find in logs' }), 'error{Enter}');
    expect(terminals[0]?.findNext).toHaveBeenCalledWith('error', expect.anything());
    await user.click(screen.getByRole('button', { name: 'Previous match' }));
    expect(terminals[0]?.findPrevious).toHaveBeenCalledWith('error', expect.anything());
  });

  it('explains a stream that fails', async () => {
    mockCommands({
      ...baseCommands,
      logs_subscribe: () => {
        throw {
          code: 'container_not_running',
          message: 'web isn’t running.',
          hint: { text: 'Start it first.', commands: [] },
          detail: null,
        };
      },
    });
    renderWithProviders(<LogViewer hostId="h1" sources={source} />);
    expect(await screen.findByText(/web isn’t running\./)).toBeInTheDocument();
    expect(terminals[0]?.text()).toContain('Start it first.');
  });
});
