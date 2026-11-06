import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Settings } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const appInfo = () => ({ name: 'homeport', version: '0.13.0', os: 'linux', arch: 'x86_64' });

describe('desktop settings', () => {
  it('turns close to tray on', async () => {
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
    const toggle = await screen.findByRole('switch', { name: 'Close to the system tray' });
    expect(toggle).not.toBeChecked();
    await user.click(toggle);
    await waitFor(() => expect(saved.at(-1)?.closeToTray).toBe(true));
  });

  it('explains a desktop without a tray', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      get_capabilities: () => ({ ...baseCommands.get_capabilities(), tray: false }),
    });
    renderApp('/settings');
    expect(
      await screen.findByText(
        'This desktop has no system tray Homeport can use, so closing the window quits it.',
      ),
    ).toBeInTheDocument();
    expect(screen.getByRole('switch', { name: 'Close to the system tray' })).toBeDisabled();
  });

  it('turns start at login on, then allows start hidden', async () => {
    let atLogin = false;
    const saved: Settings[] = [];
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      app_start_at_login: () => atLogin,
      app_set_start_at_login: ({ enabled }) => {
        atLogin = enabled as boolean;
        return atLogin;
      },
      update_settings: ({ settings }) => {
        saved.push(settings as Settings);
        return settings;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    const hidden = await screen.findByRole('switch', { name: 'Start hidden in the tray' });
    expect(hidden).toBeDisabled();
    await user.click(await screen.findByRole('switch', { name: 'Start at login' }));
    await waitFor(() => expect(hidden).toBeEnabled());
    expect(atLogin).toBe(true);
    await user.click(hidden);
    await waitFor(() => expect(saved.at(-1)?.startHidden).toBe(true));
  });

  it('shows a login entry removed outside Homeport as off', async () => {
    mockCommands({ ...baseCommands, get_app_info: appInfo, app_start_at_login: () => false });
    renderApp('/settings');
    const toggle = await screen.findByRole('switch', { name: 'Start at login' });
    await waitFor(() => expect(toggle).toBeEnabled());
    expect(toggle).not.toBeChecked();
  });

  it('says why the login entry couldn’t be changed', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      app_set_start_at_login: () => {
        throw {
          code: 'autostart_failed',
          key: 'errors.desktop.autostartFailed',
          params: {},
          message: 'Homeport couldn’t change whether it starts when you log in.',
          hint: null,
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    const toggle = await screen.findByRole('switch', { name: 'Start at login' });
    await waitFor(() => expect(toggle).toBeEnabled());
    await user.click(toggle);
    expect(
      await screen.findByText("Homeport couldn't change whether it starts when you log in."),
    ).toBeInTheDocument();
  });
});
