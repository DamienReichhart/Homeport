import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ChartSeries } from '@/shared/ui';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

const charts: { series: ChartSeries[]; data: unknown }[] = [];
vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  TimeChart: (props: { series: ChartSeries[]; data: unknown }) => {
    charts.push(props);
    return null;
  },
}));

const GB = 1024 ** 3;

const node = (name: string) => ({
  name,
  ready: true,
  unschedulable: false,
  roles: [],
  version: 'v1.33.1',
  os: 'linux',
  arch: 'amd64',
  internalIp: '10.0.0.4',
  cpu: '4',
  memory: '16Gi',
  cpuMillicores: 4000,
  memoryBytes: 16 * GB,
  taints: 0,
  created: null,
});

interface Stream {
  target: unknown;
  onEvent: unknown;
}

function commands(streams: Stream[]) {
  return {
    ...baseCommands,
    ...clusterCommands,
    clusters_nodes_list: () => [node('node-a'), node('node-b')],
    clusters_node_get: () => ({
      view: node('node-a'),
      labels: {},
      annotations: {},
      conditions: [],
      capacity: {},
      allocatable: {},
      taints: [],
      osImage: null,
      kernel: null,
      containerRuntime: null,
      addresses: [],
    }),
    clusters_pods_list: () => [],
    clusters_metrics_subscribe: (args: Record<string, unknown>) => {
      streams.push(args as unknown as Stream);
      return streams.length;
    },
  };
}

describe('node metrics', () => {
  beforeEach(() => {
    charts.length = 0;
  });

  it('adds what each Node uses against its allocatable to the list', async () => {
    const streams: Stream[] = [];
    mockCommands(commands(streams));
    renderApp('/nodes');
    expect(await screen.findByText('node-b')).toBeInTheDocument();
    await waitFor(() => expect(streams).toHaveLength(1));
    expect(streams[0]?.target).toEqual({ type: 'nodes' });
    expect(screen.queryByRole('button', { name: 'CPU used' })).not.toBeInTheDocument();
    sendToChannel(streams[0]?.onEvent, {
      type: 'nodes',
      nodes: [
        { name: 'node-a', time: 0, usage: { cpu: 3800, memory: 4 * GB } },
        { name: 'node-b', time: 0, usage: { cpu: 400, memory: 8 * GB } },
      ],
    });
    expect(await screen.findByRole('button', { name: 'CPU used' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Memory used' })).toBeInTheDocument();
    expect(screen.getByText('3,800m')).toBeInTheDocument();
    // Nearly all of it: a warning.
    expect(screen.getByText('95.0% of allocatable')).toHaveClass('text-warning');
    expect(screen.getByText('25.0% of allocatable')).toBeInTheDocument();
    expect(screen.getByText('50.0% of allocatable')).toBeInTheDocument();
  });

  it('charts a Node against its allocatable, and says when metrics-server doesn’t answer', async () => {
    const streams: Stream[] = [];
    mockCommands(commands(streams));
    const user = userEvent.setup();
    renderApp('/nodes/node-a');
    await user.click(await screen.findByRole('tab', { name: 'Metrics' }));
    await waitFor(() => expect(streams).toHaveLength(1));
    expect(streams[0]?.target).toEqual({ type: 'node', name: 'node-a' });

    sendToChannel(streams[0]?.onEvent, {
      type: 'notResponding',
      error: { code: 'timeout', key: 'x', params: {}, message: 'kind-dev didn’t answer in time.' },
    });
    expect(
      await screen.findByText(/The Metrics API doesn’t answer: metrics-server may be starting/),
    ).toBeInTheDocument();

    sendToChannel(streams[0]?.onEvent, {
      type: 'node',
      samples: [{ time: 0, usage: { cpu: 500, memory: 2 * GB } }],
    });
    await waitFor(() => expect(charts.length).toBeGreaterThan(0));
    expect(screen.queryByText(/doesn’t answer/)).not.toBeInTheDocument();
    const cpu = [...charts].reverse().find((c) => (c.data as number[][])[2]?.[0] === 4000);
    expect(cpu?.series.map((s) => s.label)).toEqual(['Used', 'Allocatable']);
    const memory = [...charts].reverse().find((c) => (c.data as number[][])[2]?.[0] === 16 * GB);
    expect(memory?.data).toEqual([[0], [2 * GB], [16 * GB]]);
  });
});
