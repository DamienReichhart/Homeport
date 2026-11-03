import { screen } from '@testing-library/react';
import { connectedHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('app', () => {
  beforeEach(() =>
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      update_settings: ({ settings }) => settings,
      get_capabilities: () => ({ keychain: true, compose: null }),
      get_app_info: () => ({ name: 'Homeport', version: '0.2.0', os: 'linux', arch: 'x86_64' }),
      hosts_list: () => [connectedHost],
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
      usage_subscribe: () => 1,
      stream_cancel: () => null,
    }),
  );

  it('shows each host with its live numbers on the overview', async () => {
    renderApp('/');
    expect(await screen.findByText('2')).toBeInTheDocument();
    expect(screen.getByText(/of 8 CPUs/)).toBeInTheDocument();
    expect(screen.getAllByText('Connected').length).toBeGreaterThan(0);
  });

  it('warns when no keychain is available', async () => {
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      get_capabilities: () => ({ keychain: false, compose: null }),
      hosts_list: () => [],
    });
    renderApp('/settings');
    expect(await screen.findByRole('alert')).toHaveTextContent(/No OS keychain/);
  });
});

describe('first launch', () => {
  it('offers the engines found on this machine', async () => {
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      get_capabilities: () => ({ keychain: true, compose: null }),
      hosts_list: () => [],
      hosts_detect_local: () => [
        { label: 'Docker Engine', socketPath: '/var/run/docker.sock', engineVersion: '29.1' },
      ],
    });
    renderApp('/containers');
    expect(await screen.findByText('Found on this machine')).toBeInTheDocument();
    expect(screen.getByText('/var/run/docker.sock')).toBeInTheDocument();
  });
});
