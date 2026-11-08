import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { containerdContainer, containerdHost } from '@/test/containerd';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const web = containerdContainer('web');
const old = containerdContainer('old', { state: 'stopped', pid: null, exitStatus: 137 });
const dns = containerdContainer('9c1d', {
  name: 'coredns-7db6/coredns',
  createdBy: 'kubernetes',
  pod: 'kube-system/coredns-7db6',
});
const pause = containerdContainer('a1', {
  name: 'coredns-7db6',
  sandbox: true,
  createdBy: 'kubernetes',
});

function setup(calls: { cmd: string; args: Record<string, unknown> }[] = []) {
  const record = (cmd: string, result: unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [containerdHost],
    containerd_namespaces: () => [{ name: 'default', labels: {}, containers: 4, images: 1 }],
    containerd_containers: () => [web, old, dns, pause],
    containerd_container_inspect: ({ id }) => ({
      container: [web, old, dns, pause].find((c) => c.id === id),
      spec: { ociVersion: '1.2.0', process: { args: ['sh'] } },
      extensions: [],
      updatedAt: null,
    }),
    containerd_container_act: record('containerd_container_act', null),
    containerd_container_kill: record('containerd_container_kill', null),
    containerd_container_remove: record('containerd_container_remove', null),
  });
  return userEvent.setup();
}

const nav = () => screen.getByRole('navigation', { name: 'Main' });

describe('containerd containers', () => {
  it('list the namespace’s containers with their task, hiding pause containers', async () => {
    const user = setup();
    renderApp('/containerd/containers');
    expect(await screen.findByText('web')).toBeInTheDocument();
    expect(screen.getByText('Exited (137)')).toBeInTheDocument();
    expect(screen.getByText('kube-system/coredns-7db6')).toBeInTheDocument();
    expect(screen.queryByText('pause')).toBeNull();
    await user.click(screen.getByLabelText('Show Pods’ pause containers'));
    expect(screen.getByText('pause')).toBeInTheDocument();
    // The Docker pages give way to containerd's.
    expect(within(nav()).getAllByRole('link', { name: 'Containers' })).toHaveLength(1);
    expect(within(nav()).queryByRole('link', { name: 'Volumes' })).toBeNull();
  });

  it('stop, start and remove from the list, keeping the snapshot when asked', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/containerd/containers');
    await user.click(await screen.findByRole('button', { name: 'Stop web' }));
    await user.click(screen.getByRole('button', { name: 'Start old' }));
    await waitFor(() =>
      expect(calls.filter((c) => c.cmd === 'containerd_container_act').map((c) => c.args)).toEqual([
        { hostId: 'c1', namespace: 'default', id: 'web', action: 'stop' },
        { hostId: 'c1', namespace: 'default', id: 'old', action: 'start' },
      ]),
    );

    await user.click(screen.getByRole('button', { name: 'Remove web' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByText(/Homeport kills it/)).toBeInTheDocument();
    await user.click(within(dialog).getByRole('checkbox'));
    await user.click(within(dialog).getByRole('button', { name: 'Kill and remove' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'containerd_container_remove',
        args: { hostId: 'c1', namespace: 'default', id: 'web', force: true, keepSnapshot: true },
      }),
    );
  });

  it('warn that the kubelet undoes changes to its containers', async () => {
    setup();
    renderApp('/containerd/containers/9c1d');
    expect(await screen.findByText(/the kubelet starts it again/)).toBeInTheDocument();
    expect(screen.getByText('kube-system/coredns-7db6')).toBeInTheDocument();
  });

  it('send a signal of the user’s choice from a container’s page', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const user = setup(calls);
    renderApp('/containerd/containers/web');
    await user.click(await screen.findByRole('button', { name: 'Send signal' }));
    const signal = await screen.findByLabelText('Signal');
    await user.clear(signal);
    await user.type(signal, 'SIGHUP');
    await user.click(screen.getByRole('button', { name: 'Send' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'containerd_container_kill',
        args: { hostId: 'c1', namespace: 'default', id: 'web', signal: 'SIGHUP' },
      }),
    );
    await user.click(screen.getByRole('tab', { name: 'Spec' }));
    expect(await screen.findByText(/ociVersion/)).toBeInTheDocument();
  });
});
