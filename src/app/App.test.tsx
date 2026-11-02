import { render, screen } from '@testing-library/react';
import { RouterProvider } from 'react-router-dom';
import { mockCommands } from '@/test/ipc';
import { AppProviders, createQueryClient } from './providers/AppProviders';
import { createAppRouter } from './routes/router';

const connected = {
  host: {
    id: 'h1',
    name: 'local',
    enabled: true,
    hostKey: null,
    createdAt: '',
    connection: { kind: 'local', socketPath: '/var/run/docker.sock' },
  },
  status: {
    state: 'connected',
    engineVersion: '29.1',
    apiVersion: '1.52',
    os: 'linux',
    arch: 'amd64',
  },
};

function renderAt(path: string) {
  render(
    <AppProviders client={createQueryClient()}>
      <RouterProvider router={createAppRouter([path])} />
    </AppProviders>,
  );
}

describe('app', () => {
  beforeEach(() =>
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      update_settings: ({ settings }) => settings,
      get_capabilities: () => ({ keychain: true, compose: null }),
      get_app_info: () => ({ name: 'Homeport', version: '0.1.0', os: 'linux', arch: 'x86_64' }),
      hosts_list: () => [connected],
      system_info: () => ({
        name: 'box',
        serverVersion: '29.1',
        operatingSystem: 'Ubuntu',
        kernelVersion: '7.0',
        architecture: 'x86_64',
        cpus: 8,
        memory: 16 * 1024 ** 3,
        rootDir: '/var/lib/docker',
        storageDriver: 'overlay2',
        containers: 3,
        running: 2,
        paused: 0,
        stopped: 1,
        images: 5,
      }),
      system_disk_usage: () => {
        const line = { total: 1, active: 1, size: 1024, reclaimable: 0 };
        return { images: line, containers: line, volumes: line, buildCache: line };
      },
      hosts_events: () => [],
    }),
  );

  it('shows each host with its live numbers on the overview', async () => {
    renderAt('/');
    expect(await screen.findByText('2')).toBeInTheDocument();
    expect(screen.getByText('8 CPUs')).toBeInTheDocument();
    expect(screen.getAllByText('Connected').length).toBeGreaterThan(0);
  });

  it('warns when no keychain is available', async () => {
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      get_capabilities: () => ({ keychain: false, compose: null }),
      hosts_list: () => [],
    });
    renderAt('/settings');
    expect(await screen.findByRole('alert')).toHaveTextContent(/No OS keychain/);
  });
});
