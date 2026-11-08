import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { PodContainer, PodDetail } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { podView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { resetTerminals } from '@/test/terminal';
import { PodLogs } from './PodLogs';

vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  XTerm: (await import('@/test/terminal')).FakeXTerm,
}));

const container = (name: string, extra: Partial<PodContainer> = {}): PodContainer => ({
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
  ...extra,
});

const pod = {
  view: podView('api-1', { namespace: 'shop' }),
  labels: {},
  annotations: {},
  conditions: [],
  containers: [
    container('migrate', { init: true, state: 'terminated' }),
    container('api', { restarts: 4, state: 'waiting', reason: 'CrashLoopBackOff' }),
    container('envoy'),
  ],
  hostIp: null,
  qosClass: null,
  serviceAccount: null,
} as unknown as PodDetail;

describe('pod logs', () => {
  beforeEach(resetTerminals);

  it('opens the first app container, then the run before its last crash', async () => {
    const asked: Record<string, unknown>[] = [];
    mockCommands({
      ...baseCommands,
      clusters_pod_logs_subscribe: (args) => {
        asked.push(args);
        return asked.length;
      },
      stream_cancel: () => null,
    });
    const user = userEvent.setup();
    renderWithProviders(<PodLogs clusterId="c1" pod={pod} />);
    await waitFor(() => expect(asked).toHaveLength(1));
    expect(asked[0]).toMatchObject({
      id: 'c1',
      namespace: 'shop',
      sources: [{ pod: 'api-1', label: null }],
      target: { container: 'api', previous: false },
    });
    await user.click(screen.getByRole('button', { name: 'Previous instance' }));
    await waitFor(() => expect(asked).toHaveLength(2));
    expect(asked[1]).toMatchObject({ target: { container: 'api', previous: true } });
  });

  it('offers no previous instance for a container that never restarted', async () => {
    mockCommands({
      ...baseCommands,
      clusters_pod_logs_subscribe: () => 1,
      stream_cancel: () => null,
    });
    renderWithProviders(
      <PodLogs
        clusterId="c1"
        pod={{ ...pod, containers: [container('web')] } as unknown as PodDetail}
      />,
    );
    expect(await screen.findByRole('button', { name: 'Previous instance' })).toBeDisabled();
    expect(screen.queryByRole('combobox', { name: 'Container' })).not.toBeInTheDocument();
  });
});
