import { screen } from '@testing-library/react';
import { baseCommands } from '@/test/fixtures';
import { incusHost, incusProject } from '@/test/incus';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('the Overview of an Incus Host', () => {
  it('shows Incus and its projects, and asks nothing of the Docker Engine API', async () => {
    const docker: string[] = [];
    const asked = (cmd: string) => () => {
      docker.push(cmd);
      return null;
    };
    mockCommands({
      ...baseCommands,
      hosts_list: () => [incusHost],
      incus_projects: () => [incusProject('default', 3, 2), incusProject('lab', 1, 0)],
      system_info: asked('system_info'),
      system_disk_usage: asked('system_disk_usage'),
      containers_list: asked('containers_list'),
    });
    renderApp('/');
    expect(await screen.findByText('4 instances in 2 projects')).toBeInTheDocument();
    expect(screen.getAllByText('3 instances, 2 images').length).toBeGreaterThan(0);
    expect(screen.getByText('6.0.4')).toBeInTheDocument();
    // The sidebar offers Incus's pages, and the dashboard links them.
    expect(screen.getAllByRole('link', { name: 'Instances' })).toHaveLength(2);
    expect(screen.queryByRole('link', { name: 'Containers' })).toBeNull();
    expect(docker).toEqual([]);
  });
});
