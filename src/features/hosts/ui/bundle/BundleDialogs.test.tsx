import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { BundleHost } from '@/shared/api';
import { baseCommands, connectedHost, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const dialogs = vi.hoisted(() => ({
  pickSavePath: vi.fn<() => Promise<string | null>>(),
  pickFile: vi.fn<() => Promise<string | null>>(),
}));
vi.mock('@/shared/lib/dialogs', () => ({
  pickFiles: vi.fn(),
  pickFolder: vi.fn(),
  ...dialogs,
}));

const PASS = 'a long passphrase';

beforeEach(() => {
  dialogs.pickSavePath.mockReset();
  dialogs.pickFile.mockReset();
});

describe('export hosts dialog', () => {
  function setup(withoutSecrets: string[] = []) {
    const exported: unknown[] = [];
    mockCommands({
      ...baseCommands,
      hosts_list: () => [connectedHost, hostView('prod')],
      hosts_export: (args) => {
        exported.push(args);
        return { exported: (args.ids as string[]).length, withoutSecrets };
      },
    });
    renderApp('/hosts');
    return exported;
  }

  it('exports the picked hosts once the passphrase is long enough and typed twice', async () => {
    const exported = setup();
    dialogs.pickSavePath.mockResolvedValue('/home/me/hosts.homeport');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Export' }));
    const dialog = await screen.findByRole('dialog');
    await user.click(within(dialog).getByLabelText(/local/));
    const submit = within(dialog).getByRole('button', { name: 'Export 1 host…' });
    expect(submit).toBeDisabled();

    await user.type(within(dialog).getByLabelText('Passphrase'), 'short');
    expect(within(dialog).getByText('Use at least 12 characters.')).toBeInTheDocument();
    await user.clear(within(dialog).getByLabelText('Passphrase'));
    await user.type(within(dialog).getByLabelText('Passphrase'), PASS);
    await user.type(within(dialog).getByLabelText('Passphrase again'), `${PASS}!`);
    expect(within(dialog).getByText('The two passphrases differ.')).toBeInTheDocument();
    expect(submit).toBeDisabled();
    await user.type(within(dialog).getByLabelText('Passphrase again'), '{Backspace}');
    await user.click(submit);

    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull());
    expect(exported).toEqual([
      { ids: ['prod-id'], path: '/home/me/hosts.homeport', passphrase: PASS },
    ]);
  });

  it('writes nothing when the save dialog is cancelled', async () => {
    const exported = setup();
    dialogs.pickSavePath.mockResolvedValue(null);
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Export' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Passphrase'), PASS);
    await user.type(within(dialog).getByLabelText('Passphrase again'), PASS);
    await user.click(within(dialog).getByRole('button', { name: 'Export 2 hosts…' }));
    await waitFor(() => expect(dialogs.pickSavePath).toHaveBeenCalled());
    expect(exported).toEqual([]);
    expect(screen.getByRole('dialog')).toBeInTheDocument();
  });

  it('names the hosts exported without their credentials', async () => {
    setup(['prod']);
    dialogs.pickSavePath.mockResolvedValue('/tmp/x.homeport');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Export' }));
    const dialog = await screen.findByRole('dialog');
    await user.type(within(dialog).getByLabelText('Passphrase'), PASS);
    await user.type(within(dialog).getByLabelText('Passphrase again'), PASS);
    await user.click(within(dialog).getByRole('button', { name: 'Export 2 hosts…' }));
    expect(
      await within(dialog).findByText(/exported without their credentials/),
    ).toBeInTheDocument();
    expect(within(dialog).getByText('prod')).toBeInTheDocument();
  });
});

describe('import bundle dialog', () => {
  const bundled = (
    index: number,
    name: string,
    overrides: Partial<BundleHost> = {},
  ): BundleHost => ({
    index,
    name,
    suggestedName: name,
    nameTaken: false,
    connection: { kind: 'tcp', address: `${name}.example.com:2375` },
    secretsMissing: false,
    ...overrides,
  });

  it('opens a file with its passphrase and imports the picked hosts', async () => {
    const imported: unknown[] = [];
    mockCommands({
      ...baseCommands,
      hosts_bundle_preview: ({ passphrase }) => {
        if (passphrase !== PASS) {
          throw {
            code: 'bundle_unreadable',
            key: '',
            params: {},
            message: 'The passphrase doesn’t open this file, or the file is damaged.',
            hint: null,
            detail: null,
          };
        }
        return [
          bundled(0, 'local', { nameTaken: true, suggestedName: 'local (2)' }),
          bundled(1, 'edge', { secretsMissing: true }),
        ];
      },
      hosts_bundle_import: (args) => {
        imported.push(args);
        return { created: [hostView('local (2)')], failed: [] };
      },
    });
    dialogs.pickFile.mockResolvedValue('/home/me/hosts.homeport');
    const user = userEvent.setup();
    renderApp('/hosts');
    await user.click(await screen.findByRole('button', { name: 'Import' }));
    const dialog = await screen.findByRole('dialog');
    await user.click(within(dialog).getByRole('button', { name: 'Choose file…' }));
    expect(await within(dialog).findByText('/home/me/hosts.homeport')).toBeInTheDocument();

    await user.type(within(dialog).getByLabelText('Passphrase'), 'wrong one');
    await user.click(within(dialog).getByRole('button', { name: 'Open' }));
    expect(await within(dialog).findByText(/doesn’t open this file/)).toBeInTheDocument();

    await user.clear(within(dialog).getByLabelText('Passphrase'));
    await user.type(within(dialog).getByLabelText('Passphrase'), PASS);
    await user.click(within(dialog).getByRole('button', { name: 'Open' }));
    expect(await within(dialog).findByDisplayValue('local (2)')).toBeInTheDocument();
    expect(within(dialog).getByText(/so this one is renamed/)).toBeInTheDocument();
    expect(within(dialog).getByText(/Exported without its credentials/)).toBeInTheDocument();

    await user.click(within(dialog).getByLabelText('Import edge'));
    await user.click(within(dialog).getByRole('button', { name: 'Import 1 host' }));
    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull());
    expect(imported).toEqual([
      {
        path: '/home/me/hosts.homeport',
        passphrase: PASS,
        choices: [{ index: 0, name: 'local (2)' }],
      },
    ]);
  });
});
