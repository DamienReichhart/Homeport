import { screen } from '@testing-library/react';
import { containerdHost } from '@/test/containerd';
import { baseCommands, connectedHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('the Overview of a containerd Host', () => {
  it('shows containerd and its namespaces, and asks nothing of the Docker Engine API', async () => {
    const docker: string[] = [];
    const asked = (cmd: string) => () => {
      docker.push(cmd);
      return null;
    };
    mockCommands({
      ...baseCommands,
      hosts_list: () => [containerdHost],
      containerd_namespaces: () => [
        { name: 'default', labels: {}, containers: 0, images: 0 },
        { name: 'k8s.io', labels: {}, containers: 12, images: 30 },
      ],
      system_info: asked('system_info'),
      system_disk_usage: asked('system_disk_usage'),
      containers_list: asked('containers_list'),
    });
    renderApp('/');
    expect(await screen.findByText('12 containers in 2 namespaces')).toBeInTheDocument();
    // In the dashboard and in the namespace picker.
    expect(screen.getAllByText('12 containers, 30 images').length).toBeGreaterThan(0);
    expect(screen.getByText('2.1.4')).toBeInTheDocument();
    expect(screen.queryByRole('link', { name: 'Its containers on the board' })).toBeNull();
    expect(docker).toEqual([]);
  });

  it('leaves Docker Hosts’ cards as they were', async () => {
    mockCommands({ ...baseCommands, hosts_list: () => [connectedHost] });
    renderApp('/');
    expect((await screen.findAllByText('local')).length).toBeGreaterThan(0);
    expect(screen.queryByText(/namespaces?$/)).toBeNull();
  });
});
