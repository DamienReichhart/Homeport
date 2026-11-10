import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const dialogs = vi.hoisted(() => ({
  pickSavePath: vi.fn<() => Promise<string | null>>(),
  pickFile: vi.fn<() => Promise<string | null>>(),
}));
vi.mock('@/shared/lib/dialogs', () => ({ pickFiles: vi.fn(), pickFolder: vi.fn(), ...dialogs }));

const contents = {
  createdAt: '2026-11-06T10:00:00Z',
  appVersion: '0.12.0',
  hosts: ['prod', 'lab'],
  clusters: ['kind-dev'],
  stacks: 3,
  registries: 1,
  repositories: 1,
  settings: true,
  withoutSecrets: [] as string[],
};

function settingsPage(extra: Record<string, (args: Record<string, unknown>) => unknown>) {
  mockCommands({
    ...baseCommands,
    get_app_info: () => ({ name: 'homeport', version: '0.12.0', os: 'linux', arch: 'x86_64' }),
    ...extra,
  });
  renderApp('/settings');
}

describe('backing up and restoring Homeport', () => {
  beforeEach(() => {
    dialogs.pickSavePath.mockReset();
    dialogs.pickFile.mockReset();
  });

  it('backs up with a passphrase typed twice', async () => {
    const backups: unknown[] = [];
    settingsPage({
      data_backup: (args) => {
        backups.push(args);
        return { contents, withoutSecrets: [] };
      },
    });
    dialogs.pickSavePath.mockResolvedValue('/home/me/homeport.homeport');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Back up…' }));
    const start = screen.getAllByRole('button', { name: 'Back up…' }).at(-1)!;
    await user.type(screen.getByLabelText('Passphrase'), 'correct horse battery');
    expect(start).toBeDisabled();
    expect(screen.getByText('The two passphrases differ.')).toBeInTheDocument();
    await user.type(screen.getByLabelText('Passphrase again'), 'correct horse battery');
    await user.click(start);
    await waitFor(() =>
      expect(backups).toEqual([
        { path: '/home/me/homeport.homeport', passphrase: 'correct horse battery' },
      ]),
    );
    expect(await screen.findByText('Backed up 2 Hosts and 3 Stacks')).toBeInTheDocument();
  });

  it('lists what a backup holds and replaces everything once confirmed', async () => {
    const restored: unknown[] = [];
    settingsPage({
      data_restore_preview: () => contents,
      data_restore: (args) => {
        restored.push(args);
        return contents;
      },
    });
    dialogs.pickFile.mockResolvedValue('/home/me/homeport.homeport');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Restore…' }));
    await user.type(await screen.findByLabelText('Passphrase'), 'correct horse battery');
    await user.click(screen.getByRole('button', { name: 'Open' }));

    expect(await screen.findByText('2 Hosts: prod, lab')).toBeInTheDocument();
    expect(screen.getByText('3 Stacks')).toBeInTheDocument();
    expect(screen.getByText(/Restoring replaces every Host/)).toBeInTheDocument();
    expect(screen.queryByRole('list', { name: 'Backed up without their secrets' })).toBeNull();
    expect(restored).toEqual([]);
    await user.click(screen.getByRole('button', { name: 'Replace everything' }));
    await waitFor(() =>
      expect(restored).toEqual([
        { path: '/home/me/homeport.homeport', passphrase: 'correct horse battery' },
      ]),
    );
    expect(await screen.findByText('Restored 2 Hosts and 3 Stacks')).toBeInTheDocument();
  });

  it('names what the backup holds without its secrets before replacing anything', async () => {
    settingsPage({
      data_restore_preview: () => ({ ...contents, withoutSecrets: ['prod', 'ghcr.io'] }),
      data_restore: () => contents,
    });
    dialogs.pickFile.mockResolvedValue('/home/me/homeport.homeport');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Restore…' }));
    await user.type(await screen.findByLabelText('Passphrase'), 'correct horse battery');
    await user.click(screen.getByRole('button', { name: 'Open' }));

    const list = await screen.findByRole('list', { name: 'Backed up without their secrets' });
    expect(
      within(list)
        .getAllByRole('listitem')
        .map((i) => i.textContent),
    ).toEqual(['prod', 'ghcr.io']);
    expect(screen.getByText(/replacing the secrets they have here/)).toBeInTheDocument();
  });

  it('says why a backup can’t be opened, and restores nothing', async () => {
    const restored: unknown[] = [];
    settingsPage({
      data_restore_preview: () => {
        throw {
          code: 'bundle_unreadable',
          message: 'The passphrase is wrong, or the file is damaged.',
          hint: null,
          detail: null,
        };
      },
      data_restore: (args) => restored.push(args),
    });
    dialogs.pickFile.mockResolvedValue('/home/me/homeport.homeport');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Restore…' }));
    await user.type(await screen.findByLabelText('Passphrase'), 'wrong passphrase!');
    await user.click(screen.getByRole('button', { name: 'Open' }));
    expect(
      await screen.findByText('The passphrase is wrong, or the file is damaged.'),
    ).toBeInTheDocument();
    expect(restored).toEqual([]);
  });
});
