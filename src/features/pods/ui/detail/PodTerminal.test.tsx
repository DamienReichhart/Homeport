import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { PodDetail } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { podView } from '@/test/clusters';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { resetTerminals, terminals } from '@/test/terminal';
import { PodTerminal } from './PodTerminal';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

const container = (name: string) => ({
  name,
  image: 'img',
  init: false,
  state: 'running',
  reason: null,
  message: null,
  started: null,
  exitCode: null,
  ready: true,
  restarts: 0,
  lastReason: null,
  ports: [],
  cpu: { request: null, limit: null },
  memory: { request: null, limit: null },
});

const pod = {
  view: podView('api-1', { namespace: 'shop' }),
  labels: {},
  annotations: {},
  conditions: [],
  containers: [container('api'), container('envoy')],
  hostIp: null,
  qosClass: null,
  serviceAccount: null,
} as unknown as PodDetail;

function terminalsOpened() {
  const started: { args: Record<string, unknown> }[] = [];
  const input: string[] = [];
  const closed: number[] = [];
  mockCommands({
    ...baseCommands,
    clusters_exec_start: (args) => {
      started.push({ args });
      return started.length;
    },
    clusters_exec_resize: () => null,
    clusters_exec_input: ({ data }) => {
      input.push(data as string);
      return null;
    },
    clusters_exec_close: ({ terminal }) => {
      closed.push(terminal as number);
      return null;
    },
  });
  return { started, input, closed };
}

describe('pod terminal', () => {
  beforeEach(resetTerminals);

  it('opens a shell in the default container and sends what is typed', async () => {
    const { started, input } = terminalsOpened();
    renderWithProviders(<PodTerminal clusterId="c1" pod={pod} />);
    await waitFor(() => expect(started).toHaveLength(1));
    expect(started[0]?.args).toMatchObject({
      id: 'c1',
      namespace: 'shop',
      pod: 'api-1',
      options: { container: 'api', command: null },
    });
    await waitFor(() => expect(terminals[0]?.focus).toHaveBeenCalled());
    terminals[0]?.type('top\r');
    expect(input).toEqual(['top\r']);
  });

  it('reconnects with the typed command once the process exits, closing the old one', async () => {
    const { started, closed } = terminalsOpened();
    const user = userEvent.setup();
    renderWithProviders(<PodTerminal clusterId="c1" pod={pod} />);
    await waitFor(() => expect(started).toHaveLength(1));
    sendToChannel(started[0]?.args.onEvent, { type: 'exit', code: 0, error: null });
    expect(await screen.findByText(/process exited with code 0/)).toBeInTheDocument();
    await user.type(screen.getByLabelText('Command'), 'psql -U app');
    await user.click(screen.getByRole('button', { name: 'Reconnect' }));
    await waitFor(() => expect(started).toHaveLength(2));
    expect(started[1]?.args).toMatchObject({ options: { command: 'psql -U app' } });
    expect(closed).toEqual([1]);
  });
});
