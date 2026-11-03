import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { UpdateMethod } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const available = (method: UpdateMethod) => () => ({
  currentVersion: '0.3.0',
  available: { version: '0.4.0', notes: 'Faster', date: '2026-12-01T00:00:00Z' },
  method,
});

const pages = {
  ...baseCommands,
  get_app_info: () => ({ name: 'homeport', version: '0.3.0', os: 'linux', arch: 'x86_64' }),
  system_info: () => null,
};

describe('update notice', () => {
  it('offers to install an update Homeport can apply itself', async () => {
    const install = vi.fn(() => null);
    mockCommands({
      ...pages,
      updates_check: available({ kind: 'inApp' }),
      updates_install: install,
    });
    const user = userEvent.setup();
    renderApp('/settings');
    const notice = await screen.findByRole('status', { name: 'Update available' });
    expect(notice).toHaveTextContent('Homeport 0.4.0 is available');
    await user.click(screen.getAllByRole('button', { name: 'Install and restart' })[0]!);
    await waitFor(() => expect(install).toHaveBeenCalled());
  });

  it('gives the package manager command for a .deb or .rpm install', async () => {
    mockCommands({
      ...pages,
      updates_check: available({
        kind: 'packageManager',
        manager: 'apt',
        command: 'sudo apt update && sudo apt install --only-upgrade homeport',
      }),
    });
    renderApp('/settings');
    const notice = await screen.findByRole('status', { name: 'Update available' });
    expect(notice).toHaveTextContent('Update it with your package manager');
    expect(notice).toHaveTextContent('sudo apt install --only-upgrade homeport');
    expect(screen.queryByRole('button', { name: 'Install and restart' })).not.toBeInTheDocument();
  });

  it('stays hidden once dismissed', async () => {
    mockCommands({ ...pages, updates_check: available({ kind: 'inApp' }) });
    const user = userEvent.setup();
    renderApp('/settings');
    await screen.findByRole('status', { name: 'Update available' });
    await user.click(screen.getByRole('button', { name: 'Dismiss' }));
    expect(screen.queryByRole('status', { name: 'Update available' })).not.toBeInTheDocument();
  });

  it('doesn’t check when automatic checks are off', async () => {
    const check = vi.fn(available({ kind: 'inApp' }));
    mockCommands({
      ...pages,
      get_settings: () => ({ theme: 'system', lastHostId: 'h1', checkForUpdates: false }),
      updates_check: check,
    });
    renderApp('/settings');
    expect(await screen.findByLabelText('Check for updates automatically')).not.toBeChecked();
    expect(check).not.toHaveBeenCalled();
  });
});

describe('updates settings', () => {
  it('checks on demand and says when Homeport is up to date', async () => {
    const check = vi.fn(() => ({
      currentVersion: '0.3.0',
      available: null,
      method: { kind: 'inApp' },
    }));
    mockCommands({
      ...pages,
      get_settings: () => ({ theme: 'system', lastHostId: 'h1', checkForUpdates: false }),
      updates_check: check,
    });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByRole('button', { name: 'Check now' }));
    expect(await screen.findByText('Homeport 0.3.0 is the latest version.')).toBeInTheDocument();
    expect(check).toHaveBeenCalledTimes(1);
  });

  it('turns automatic checks off', async () => {
    const update = vi.fn(({ settings }: Record<string, unknown>) => settings);
    mockCommands({ ...pages, update_settings: update });
    const user = userEvent.setup();
    renderApp('/settings');
    await user.click(await screen.findByLabelText('Check for updates automatically'));
    await waitFor(() =>
      expect(update).toHaveBeenCalledWith({
        settings: expect.objectContaining({ checkForUpdates: false }),
      }),
    );
  });
});
