import { openUrl } from '@tauri-apps/plugin-opener';
import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { connectedHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

vi.mock('@tauri-apps/plugin-opener', () => ({ openUrl: vi.fn(() => Promise.resolve()) }));

describe('app', () => {
  beforeEach(() =>
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      update_settings: ({ settings }) => settings,
      get_capabilities: () => ({
        keychain: true,
        helm: { state: 'available', cli: { program: 'helm', version: 'v3.16.2' } },
        stackDeploy: {
          state: 'available',
          cli: { program: 'docker', prefix: [], version: '29.1.0' },
        },
        git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
        compose: {
          state: 'unavailable',
          problem: {
            code: 'compose_missing',
            message: 'No compose CLI.',
            hint: null,
            detail: null,
          },
        },
      }),
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
        return { images: line, containers: line, volumes: line, buildCache: line, partial: false };
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

  it.each([
    ['Documentation', 'https://damienreichhart.github.io/Homeport/docs/'],
    ['GitHub', 'https://github.com/DamienReichhart/Homeport'],
  ])('opens %s in the browser from the bottom of the sidebar', async (label, url) => {
    renderApp('/settings');
    const nav = await screen.findByRole('navigation', { name: 'Main' });
    const link = await screen.findByRole('link', { name: `${label} (opens in your browser)` });
    expect(nav).toContainElement(link);
    expect(link).not.toHaveAttribute('aria-current');

    await userEvent.click(link);
    expect(openUrl).toHaveBeenCalledWith(url);
    // The app stays where it was.
    expect(screen.getByRole('link', { name: 'Settings' })).toHaveAttribute('aria-current', 'page');
  });

  it('says so when a sidebar link can’t be opened', async () => {
    vi.mocked(openUrl).mockRejectedValueOnce(new Error('no browser'));
    renderApp('/');
    await userEvent.click(await screen.findByRole('link', { name: /^GitHub/ }));
    const message = await screen.findByText('Something unexpected went wrong.');
    expect(message.closest('[data-sonner-toast]')).not.toBeNull();
    expect(screen.getByRole('navigation', { name: 'Main' })).toBeInTheDocument();
  });

  it('warns when no keychain is available', async () => {
    mockCommands({
      get_settings: () => ({ theme: 'system', lastHostId: null }),
      get_capabilities: () => ({
        keychain: false,
        helm: { state: 'available', cli: { program: 'helm', version: 'v3.16.2' } },
        stackDeploy: {
          state: 'available',
          cli: { program: 'docker', prefix: [], version: '29.1.0' },
        },
        git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
        compose: {
          state: 'unavailable',
          problem: {
            code: 'compose_missing',
            message: 'No compose CLI.',
            hint: null,
            detail: null,
          },
        },
      }),
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
      get_capabilities: () => ({
        keychain: true,
        helm: { state: 'available', cli: { program: 'helm', version: 'v3.16.2' } },
        stackDeploy: {
          state: 'available',
          cli: { program: 'docker', prefix: [], version: '29.1.0' },
        },
        git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
        compose: {
          state: 'unavailable',
          problem: {
            code: 'compose_missing',
            message: 'No compose CLI.',
            hint: null,
            detail: null,
          },
        },
      }),
      hosts_list: () => [],
      hosts_detect_local: () => [
        {
          label: 'Docker Engine',
          socketPath: '/var/run/docker.sock',
          engine: 'docker',
          engineVersion: '29.1',
          problem: null,
        },
      ],
    });
    renderApp('/containers');
    expect(await screen.findByText('Found on this machine')).toBeInTheDocument();
    expect(screen.getByText('/var/run/docker.sock')).toBeInTheDocument();
  });
});
