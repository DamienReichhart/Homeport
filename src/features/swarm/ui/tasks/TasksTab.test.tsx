import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const swarm = { id: 's1', managers: ['h1'], workers: [], reachable: true };
const base = {
  serviceId: 'svc',
  serviceName: 'web',
  nodeId: 'n1',
  nodeHostname: 'alpha',
  message: null,
  image: 'nginx:1.27',
  exitCode: null,
  hostId: 'h1',
};

describe('the Tasks tab', () => {
  it('shows why a task failed, and its earlier attempts on demand', async () => {
    mockCommands({
      ...baseCommands,
      swarms_list: () => [swarm],
      swarm_inspect: () => ({
        id: 's1',
        created: null,
        updated: null,
        nodes: 1,
        managers: 1,
        services: 1,
      }),
      swarm_tasks_list: () => [
        {
          ...base,
          id: 't2',
          slot: 1,
          desiredState: 'running',
          state: 'rejected',
          since: '2026-11-08T10:01:00Z',
          error: 'No such image: nope:1',
          containerId: null,
        },
        {
          ...base,
          id: 't1',
          slot: 1,
          desiredState: 'shutdown',
          state: 'rejected',
          since: '2026-11-08T10:00:00Z',
          error: 'earlier attempt',
          containerId: null,
        },
      ],
    });
    const user = userEvent.setup();
    renderApp('/swarms/s1/tasks');
    expect(await screen.findByText('No such image: nope:1')).toBeInTheDocument();
    expect(screen.queryByText('earlier attempt')).not.toBeInTheDocument();
    await user.click(screen.getByLabelText('Show history'));
    expect(screen.getByText('earlier attempt')).toBeInTheDocument();
  });

  it('opens the container of a task on a node that is a host', async () => {
    const web = container('web-1', null);
    mockCommands({
      ...baseCommands,
      swarms_list: () => [swarm],
      swarm_inspect: () => ({
        id: 's1',
        created: null,
        updated: null,
        nodes: 1,
        managers: 1,
        services: 1,
      }),
      swarm_tasks_list: () => [
        {
          ...base,
          id: 't1',
          slot: 1,
          desiredState: 'running',
          state: 'running',
          since: null,
          error: null,
          containerId: web.id,
        },
      ],
      containers_list: () => [web],
      containers_inspect: () => ({}),
      logs_subscribe: () => null,
      update_settings: ({ settings }: Record<string, unknown>) => settings,
    });
    const user = userEvent.setup();
    renderApp('/swarms/s1/tasks');
    await user.click(await screen.findByRole('button', { name: 'Open container' }));
    expect(await screen.findByRole('heading', { name: /web-1/ })).toBeInTheDocument();
  });
});
