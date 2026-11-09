import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { PodmanPod } from '@/shared/api';
import { baseCommands, connectedHost, container, podmanHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const web: PodmanPod = {
  id: 'p1',
  name: 'web',
  status: 'running',
  created: '2026-11-08T10:00:00Z',
  infraId: 'web-infra-id',
  labels: { app: 'web' },
  networks: ['podman'],
  containers: [
    { id: 'web-infra-id', name: 'a1b2-infra', state: 'running', infra: true },
    { id: 'web-app-0123456789abcdef', name: 'web-app', state: 'running', infra: false },
    { id: 'web-db-0123456789abcdef', name: 'web-db', state: 'exited', infra: false },
  ],
};

function setup(calls: { cmd: string; args: Record<string, unknown> }[] = [], host = podmanHost) {
  const record = (cmd: string, result: unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [host],
    pods_list: () => [web],
    pods_inspect: () => ({
      ...web,
      ports: [{ containerPort: 80, protocol: 'tcp', hostIp: null, hostPort: 8080 }],
      sharedNamespaces: ['ipc', 'net', 'uts'],
      hostname: 'web',
      createCommand: [],
    }),
    pods_act: record('pods_act', null),
    pods_remove: record('pods_remove', 2),
    pods_create: record('pods_create', 'p2'),
    containers_list: () => [container('web-app', null), container('redis', null)],
    usage_subscribe: () => 1,
    images_list: () => [],
    networks_list: () => [],
  });
  return userEvent.setup();
}

const nav = () => screen.getByRole('navigation', { name: 'Main' });

describe('Podman pods', () => {
  it('aren’t offered on a Docker Engine, and a link to them says why', async () => {
    setup([], connectedHost);
    renderApp('/podman-pods');
    expect(await screen.findByText('local can’t show this page')).toBeInTheDocument();
    expect(within(nav()).queryByRole('link', { name: 'Pods' })).toBeNull();
    expect(within(nav()).getByRole('link', { name: 'Containers' })).toBeInTheDocument();
  });

  it('are listed on a Podman Host with their containers', async () => {
    setup();
    renderApp('/podman-pods');
    expect(await screen.findByText('web')).toBeInTheDocument();
    expect(within(nav()).getByRole('link', { name: 'Pods' })).toBeInTheDocument();
    const row = screen.getByRole('row', { name: /web/ });
    expect(within(row).getByText('1 of 2 running')).toBeInTheDocument();
    expect(within(row).getByText('Running')).toBeInTheDocument();
  });

  it('stop from their row', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/podman-pods');
    await user.click(await screen.findByRole('button', { name: 'Stop web' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'pods_act',
        args: { hostId: 'h1', id: 'p1', action: 'stop' },
      }),
    );
  });

  it('are removed with their containers once confirmed, stopped first when running', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/podman-pods');
    await user.click(await screen.findByRole('button', { name: 'Remove web' }));
    const dialog = await screen.findByRole('alertdialog');
    expect(within(dialog).getByText(/its 2 containers are stopped/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole('button', { name: 'Stop and remove' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'pods_remove',
        args: { hostId: 'h1', id: 'p1', force: true },
      }),
    );
  });

  it('are created with their published ports', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/podman-pods');
    await user.click(await screen.findByRole('button', { name: 'New pod' }));
    const dialog = await screen.findByRole('dialog', { name: 'New pod' });
    await user.type(within(dialog).getByLabelText('Name'), 'api');
    await user.click(within(dialog).getByRole('button', { name: 'Add port' }));
    await user.type(within(dialog).getByLabelText('Host port'), '9090');
    await user.type(within(dialog).getByLabelText('Container port'), '80');
    await user.click(within(dialog).getByRole('button', { name: 'Create pod' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'pods_create',
        args: {
          hostId: 'h1',
          spec: {
            name: 'api',
            ports: [{ hostIp: null, hostPort: 9090, containerPort: 80, protocol: 'tcp' }],
            labels: [],
          },
        },
      }),
    );
  });

  it('show their containers and published ports on their page', async () => {
    setup();
    renderApp('/podman-pods/p1');
    expect(await screen.findByText('2 containers')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: /web-db/ })).toHaveAttribute(
      'href',
      '/containers/web-db-0123456789abcdef/logs',
    );
    expect(screen.getByText('8080 → 80/tcp')).toBeInTheDocument();
    expect(screen.getByText('ipc, net, uts')).toBeInTheDocument();
  });

  it('run new containers from their page, into themselves', async () => {
    const user = setup();
    renderApp('/podman-pods/p1');
    await user.click(await screen.findByRole('button', { name: 'Run container' }));
    const dialog = await screen.findByRole('dialog', { name: 'Run a container in web' });
    expect(within(dialog).getByRole('combobox', { name: 'Pod' })).toHaveTextContent('web');
    expect(within(dialog).queryByText('Ports')).toBeNull();
  });

  it('are named on the containers in them', async () => {
    setup();
    renderApp('/containers');
    const row = (await screen.findByText('web-app')).closest('tr')!;
    expect(within(row).getByRole('link', { name: 'web' })).toHaveAttribute(
      'href',
      '/podman-pods/p1',
    );
    // redis runs in no pod: its only link is its own name, to its logs.
    const redis = screen.getByText('redis').closest('tr')!;
    expect(
      within(redis)
        .queryAllByRole('link')
        .filter((a) => a.getAttribute('href')?.startsWith('/podman-pods/')),
    ).toEqual([]);
  });
});
