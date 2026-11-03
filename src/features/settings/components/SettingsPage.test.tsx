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
      get_licenses: () => ({ app: 'MIT License', third_party: 'serde 1.0.0 (crate)' }),
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Licenses' }));
    expect(await screen.findByText('MIT License')).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'Third-party software' }));
    expect(await screen.findByText('serde 1.0.0 (crate)')).toBeInTheDocument();
  });

  it('explains when a build has no third-party notices', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_licenses: () => ({ app: 'MIT License', third_party: null }),
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Licenses' }));
    await user.click(await screen.findByRole('tab', { name: 'Third-party software' }));
    expect(await screen.findByText('Not included in this build')).toBeInTheDocument();
  });
});
