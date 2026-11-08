import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Settings } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const appInfo = () => ({ name: 'homeport', version: '0.13.0', os: 'linux', arch: 'x86_64' });

describe('notification settings', () => {
  it('turns one kind off and keeps the others on', async () => {
    const saved: Settings[] = [];
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      update_settings: ({ settings }) => {
        saved.push(settings as Settings);
        return settings;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    const back = await screen.findByRole('switch', { name: 'An unreachable host is back' });
    expect(back).toBeChecked();
    await user.click(back);
    await waitFor(() =>
      expect(saved.at(-1)?.notifications).toEqual({
        containerDied: true,
        containerUnhealthy: true,
        hostUnreachable: true,
        hostBack: false,
      }),
    );
  });

  it('shows every kind on for settings saved before notifications existed', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_settings: () => ({ theme: 'system', lastHostId: 'h1' }),
    });
    renderApp('/settings');
    for (const name of [
      'A container stops with an error or runs out of memory',
      'A container turns unhealthy',
      'A connected host becomes unreachable',
      'An unreachable host is back',
    ]) {
      expect(await screen.findByRole('switch', { name })).toBeChecked();
    }
  });
});
