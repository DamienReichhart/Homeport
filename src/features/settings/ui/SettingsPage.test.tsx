import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const appInfo = () => ({ name: 'homeport', version: '0.2.0', os: 'linux', arch: 'x86_64' });

describe('settings licenses', () => {
  it('shows Homeport’s license and the bundled third-party notices', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_licenses: () => ({
        app: 'GNU AFFERO GENERAL PUBLIC LICENSE',
        third_party: 'serde 1.0.0 (crate)',
      }),
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Licenses' }));
    expect(await screen.findByText('GNU AFFERO GENERAL PUBLIC LICENSE')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Source of v0.2.0' })).toBeInTheDocument();
    expect(screen.getByText(/not affiliated with or endorsed by any of them/)).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Third-party software' }));
    expect(await screen.findByText('serde 1.0.0 (crate)')).toBeInTheDocument();
  });

  it('explains when a build has no third-party notices', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_licenses: () => ({ app: 'GNU AFFERO GENERAL PUBLIC LICENSE', third_party: null }),
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Licenses' }));
    await user.click(await screen.findByRole('tab', { name: 'Third-party software' }));
    expect(await screen.findByText('Not included in this build')).toBeInTheDocument();
  });
});

describe('settings diagnostics', () => {
  it('explains a missing compose CLI and opens the log folder', async () => {
    let opened = 0;
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_capabilities: () => ({
        keychain: true,
        git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
        compose: {
          state: 'unavailable',
          problem: {
            code: 'compose_missing',
            message: 'The Docker CLI is installed without its compose plugin.',
            hint: {
              text: 'Install the compose plugin.',
              commands: ['sudo apt-get install docker-compose-plugin'],
            },
            detail: null,
          },
        },
      }),
      open_log_folder: () => {
        opened += 1;
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    expect(
      await screen.findByText('The Docker CLI is installed without its compose plugin.'),
    ).toBeInTheDocument();
    expect(screen.getByText('sudo apt-get install docker-compose-plugin')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'Open log folder' }));
    expect(opened).toBe(1);
  });

  it('checks the machine again each time Settings opens', async () => {
    let installed = false;
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_capabilities: () => ({
        keychain: true,
        git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
        compose: installed
          ? {
              state: 'available',
              cli: { program: 'docker', prefix: ['compose'], version: '2.40.0' },
            }
          : {
              state: 'unavailable',
              problem: {
                code: 'compose_missing',
                message: 'The docker compose CLI isn’t installed on this computer.',
                hint: null,
                detail: null,
              },
            },
      }),
    });
    const user = userEvent.setup();
    renderApp('/settings');
    expect(
      await screen.findByText('The docker compose CLI isn’t installed on this computer.'),
    ).toBeInTheDocument();

    installed = true;
    await user.click(screen.getByRole('link', { name: 'Overview' }));
    await user.click(screen.getByRole('link', { name: 'Settings' }));
    expect(await screen.findByText('docker compose 2.40.0')).toBeInTheDocument();
  });
});

describe('settings data', () => {
  it('keeps Engine events for the days chosen, within a year', async () => {
    const saved: unknown[] = [];
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      registries_list: () => [],
      repositories_list: () => [],
      update_settings: ({ settings }) => {
        saved.push(settings);
        return settings;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    const days = await screen.findByLabelText('Keep Engine events for');
    expect(days).toHaveValue(30);
    await user.clear(days);
    await user.type(days, '900{Enter}');
    expect(days).toHaveValue(365);
    expect(saved).toEqual([expect.objectContaining({ keepEventsDays: 365 })]);
  });
});
