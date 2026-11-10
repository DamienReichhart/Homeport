import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ChartSeries } from '@/shared/ui';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands, podView } from '@/test/clusters';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

// uPlot draws on a canvas jsdom lacks: what each chart is given is checked.
const charts: { series: ChartSeries[]; data: unknown }[] = [];
vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  TimeChart: (props: { series: ChartSeries[]; data: unknown }) => {
    charts.push(props);
    return null;
  },
}));

/** Matches a card title by its whole text, which React splits into several nodes. */
const title = (text: string) => (_: string, el: Element | null) =>
  el?.getAttribute('data-slot') === 'card-title' && el.textContent === text;

const MB = 1024 ** 2;

interface Stream {
  target: { type: string };
  onEvent: unknown;
}

/** The commands of the Pods pages, the metrics streams they open kept in `streams`. */
function commands(streams: Stream[], cancelled: unknown[]) {
  return {
    ...baseCommands,
    ...clusterCommands,
    clusters_pods_list: () => [
      podView('web-1'),
      podView('web-2', {
        cpu: { request: 100, limit: null },
        memory: { request: null, limit: null },
      }),
      podView('job-1', { status: 'Completed', health: 'succeeded' }),
    ],
    clusters_metrics_subscribe: (args: Record<string, unknown>) => {
      streams.push(args as unknown as Stream);
      // Apart from the watches' ids.
      return 100 + streams.length;
    },
    stream_cancel: ({ id }: Record<string, unknown>) => {
      cancelled.push(id);
      return null;
    },
  };
}

const podUsage = (name: string, cpu: number, memory: number) => ({
  namespace: 'default',
  name,
  time: 1_794_304_800,
  usage: { cpu, memory },
  containers: [{ name: 'web', usage: { cpu, memory } }],
});

const header = (name: string) => screen.getByRole('button', { name });

describe('pod metrics', () => {
  beforeEach(() => {
    charts.length = 0;
  });

  it('adds CPU and memory to the list once metrics-server answers, sortable by use', async () => {
    const streams: Stream[] = [];
    mockCommands(commands(streams, []));
    const user = userEvent.setup();
    renderApp('/pods');
    expect(await screen.findByText('web-2')).toBeInTheDocument();
    await waitFor(() => expect(streams).toHaveLength(1));
    expect(streams[0]?.target).toEqual({ type: 'pods', namespace: 'default' });
    // Nothing is known yet: no columns.
    expect(screen.queryByRole('columnheader', { name: /CPU/ })).not.toBeInTheDocument();

    sendToChannel(streams[0]?.onEvent, {
      type: 'pods',
      pods: [podUsage('web-1', 250, 128 * MB), podUsage('web-2', 400, 64 * MB)],
    });
    expect(await screen.findByText('25.0% of limit')).toBeInTheDocument();
    expect(screen.getByText('250m')).toBeInTheDocument();
    expect(screen.getByText('50.0% of request')).toBeInTheDocument();
    expect(screen.getByText('400% of request')).toBeInTheDocument();
    expect(screen.getByText('128.0 MB')).toBeInTheDocument();
    // A Pod with no sample (it has ended) shows a dash.
    const done = screen.getByText('job-1').closest('tr');
    expect(within(done as HTMLElement).getAllByText('—')).toHaveLength(2);

    // Sorting by CPU orders the Pods by use, those without a sample at the end.
    const names = () =>
      screen
        .getAllByRole('row')
        .slice(1)
        .map((r) => /(web-\d|job-\d)/.exec(r.textContent ?? '')?.[1]);
    await user.click(header('CPU'));
    const first = names();
    expect([
      ['web-2', 'web-1', 'job-1'],
      ['job-1', 'web-1', 'web-2'],
    ]).toContainEqual(first);
    await user.click(header('CPU'));
    expect(names()).toEqual([...first].reverse());
  });

  it('shows no columns and says nothing in the list without metrics-server', async () => {
    const streams: Stream[] = [];
    mockCommands(commands(streams, []));
    renderApp('/pods');
    expect(await screen.findByText('web-2')).toBeInTheDocument();
    await waitFor(() => expect(streams).toHaveLength(1));
    sendToChannel(streams[0]?.onEvent, { type: 'notInstalled' });
    await waitFor(() => expect(screen.getAllByRole('row')).toHaveLength(4));
    expect(screen.queryByRole('button', { name: 'CPU' })).not.toBeInTheDocument();
    expect(screen.queryByText(/metrics-server/)).not.toBeInTheDocument();
    expect(screen.queryByRole('alert')).not.toBeInTheDocument();
  });

  it('charts each container against its request and limit, and stops reading when the page closes', async () => {
    const streams: Stream[] = [];
    const cancelled: unknown[] = [];
    mockCommands({
      ...commands(streams, cancelled),
      clusters_pod_get: () => ({
        view: podView('web-1'),
        labels: {},
        annotations: {},
        conditions: [],
        containers: ['web', 'proxy'].map((name) => ({
          name,
          image: 'nginx',
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
          cpu: name === 'web' ? { request: 250, limit: 1000 } : { request: null, limit: null },
          memory: { request: 256 * MB, limit: null },
        })),
        hostIp: null,
        qosClass: null,
        serviceAccount: null,
      }),
    });
    const user = userEvent.setup();
    const { unmount } = renderApp('/pods/default/web-1');
    expect(await screen.findByRole('tab', { name: 'Metrics' })).toBeInTheDocument();
    // The page reads metrics from when it opens, whichever tab shows.
    await waitFor(() => expect(streams).toHaveLength(1));
    expect(streams[0]?.target).toEqual({ type: 'pod', namespace: 'default', name: 'web-1' });

    await user.click(screen.getByRole('tab', { name: 'Metrics' }));
    expect(
      await screen.findByText('Waiting for the first sample from metrics-server…'),
    ).toBeInTheDocument();
    const sample = (time: number, cpu: number, memory: number) => ({
      time,
      usage: { cpu, memory },
    });
    sendToChannel(streams[0]?.onEvent, {
      type: 'pod',
      containers: [
        // metrics-server's order isn't the Pod's.
        { name: 'proxy', samples: [sample(0, 2, 10 * MB)] },
        { name: 'web', samples: [sample(0, 100, 100 * MB), sample(15, 120, 200 * MB)] },
      ],
    });
    expect(await screen.findByText(title('CPU · 120m'))).toBeInTheDocument();
    expect(screen.getByText(title('Memory · 200.0 MB'))).toBeInTheDocument();
    const sections = within(screen.getByRole('tabpanel')).getAllByRole('region');
    expect(sections.map((s) => s.getAttribute('aria-label'))).toEqual(['web', 'proxy']);

    const webCpu = [...charts].reverse().find((c) => c.series.some((s) => s.label === 'Limit'));
    expect(webCpu?.series.map((s) => s.label)).toEqual(['Used', 'Request', 'Limit']);
    expect(webCpu?.data).toEqual([
      [0, 15],
      [100, 120],
      [250, 250],
      [1000, 1000],
    ]);
    expect(screen.getByText(/kept while this page is open/)).toBeInTheDocument();

    unmount();
    await waitFor(() => expect(cancelled).toContain(101));
  });

  it('says once that metrics-server isn’t installed, with a link to it', async () => {
    const streams: Stream[] = [];
    mockCommands({
      ...commands(streams, []),
      clusters_pod_get: () => ({
        view: podView('web-1'),
        labels: {},
        annotations: {},
        conditions: [],
        containers: [],
        hostIp: null,
        qosClass: null,
        serviceAccount: null,
      }),
    });
    renderApp('/pods/default/web-1/metrics');
    await waitFor(() => expect(streams).toHaveLength(1));
    sendToChannel(streams[0]?.onEvent, { type: 'notInstalled' });
    expect(await screen.findByText('metrics-server isn’t installed')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'How to install metrics-server' })).toHaveAttribute(
      'href',
      'https://github.com/kubernetes-sigs/metrics-server',
    );
    expect(charts).toHaveLength(0);
  });

  it('says which permission is missing when the credentials can’t read metrics', async () => {
    const streams: Stream[] = [];
    mockCommands({
      ...commands(streams, []),
      clusters_pod_get: () => ({
        view: podView('web-1'),
        labels: {},
        annotations: {},
        conditions: [],
        containers: [],
        hostIp: null,
        qosClass: null,
        serviceAccount: null,
      }),
    });
    renderApp('/pods/default/web-1/metrics');
    await waitFor(() => expect(streams).toHaveLength(1));
    sendToChannel(streams[0]?.onEvent, {
      type: 'forbidden',
      error: {
        code: 'forbidden',
        key: 'errors.clusters.forbiddenIn',
        params: { verb: 'get', resource: 'pods.metrics.k8s.io', namespace: 'default' },
        message: 'Your credentials don’t allow “get” on pods.metrics.k8s.io in default.',
        hint: null,
        detail: null,
      },
    });
    expect(await screen.findByText(/pods\.metrics\.k8s\.io/)).toBeInTheDocument();
    expect(charts).toHaveLength(0);
  });
});
