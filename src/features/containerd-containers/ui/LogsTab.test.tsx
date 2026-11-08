import { screen, waitFor } from '@testing-library/react';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { resetTerminals } from '@/test/terminal';
import { LogsTab } from './LogsTab';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

describe('a containerd container’s logs', () => {
  beforeEach(resetTerminals);

  it('explain that ctr containers keep no output', async () => {
    mockCommands({ ...baseCommands, containerd_logs_source: () => ({ kind: 'none' }) });
    renderWithProviders(<LogsTab hostId="c1" namespace="default" id="t1" />);
    expect(await screen.findByText('This container’s output isn’t recorded')).toBeInTheDocument();
    expect(screen.getByText(/containerd keeps no logs/)).toBeInTheDocument();
  });

  it('name a log driver Homeport can’t read', async () => {
    mockCommands({
      ...baseCommands,
      containerd_logs_source: () => ({ kind: 'driver', driver: 'journald' }),
    });
    renderWithProviders(<LogsTab hostId="c1" namespace="default" id="t1" />);
    expect(
      await screen.findByText('nerdctl sends this container’s output to journald'),
    ).toBeInTheDocument();
  });

  it('stream the file nerdctl or the kubelet wrote', async () => {
    const subscribed: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      containerd_logs_source: () => ({
        kind: 'file',
        path: '/var/lib/nerdctl/x/containers/default/web/web-json.log',
        format: 'json',
      }),
      containerd_logs_subscribe: (args) => {
        subscribed.push(args);
        return 1;
      },
    });
    renderWithProviders(<LogsTab hostId="c1" namespace="default" id="web" />);
    expect(await screen.findByText(/web-json\.log/)).toBeInTheDocument();
    await waitFor(() => expect(subscribed).toHaveLength(1));
    expect(subscribed[0]).toMatchObject({ hostId: 'c1', namespace: 'default', id: 'web' });
  });
});
