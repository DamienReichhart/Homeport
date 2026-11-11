import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const dialogs = vi.hoisted(() => ({
  pickSavePath: vi.fn<() => Promise<string | null>>(),
}));
vi.mock('@/shared/lib/dialogs', () => ({
  pickFile: vi.fn(),
  pickFiles: vi.fn(),
  pickFolder: vi.fn(),
  ...dialogs,
}));

const preview = (includeNames: boolean) => ({
  files: [
    { name: 'summary.json', size: 412 },
    { name: 'logs/Homeport.log', size: 20480 },
  ],
  summary: JSON.stringify(
    { namesIncluded: includeNames, hosts: [{ name: includeNames ? 'prod' : 'host-1' }] },
    null,
    2,
  ),
});

function settingsPage(extra: Record<string, (args: Record<string, unknown>) => unknown> = {}) {
  mockCommands({
    ...baseCommands,
    get_app_info: () => ({ name: 'homeport', version: '1.0.0', os: 'linux', arch: 'x86_64' }),
    diagnostics_preview: ({ includeNames }) => preview(includeNames === true),
    ...extra,
  });
  renderApp('/settings');
}

describe('exporting diagnostics', () => {
  beforeEach(() => dialogs.pickSavePath.mockReset());

  it('shows what the zip holds before saving, without names by default', async () => {
    settingsPage();
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Export diagnostics…' }));

    expect(await screen.findByText('logs/Homeport.log')).toBeInTheDocument();
    expect(screen.getByText(/"name": "host-1"/)).toBeInTheDocument();
    expect(screen.queryByText(/"name": "prod"/)).not.toBeInTheDocument();

    await user.click(screen.getByRole('checkbox', { name: /Include names and addresses/ }));
    expect(await screen.findByText(/"name": "prod"/)).toBeInTheDocument();
  });

  it('saves the zip where the user chooses', async () => {
    const exports: unknown[] = [];
    settingsPage({
      diagnostics_export: (args) => {
        exports.push(args);
        return null;
      },
    });
    dialogs.pickSavePath.mockResolvedValue('/home/me/homeport-diagnostics.zip');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Export diagnostics…' }));
    await screen.findByText('logs/Homeport.log');
    await user.click(screen.getByRole('button', { name: 'Save…' }));

    await waitFor(() =>
      expect(exports).toEqual([{ path: '/home/me/homeport-diagnostics.zip', includeNames: false }]),
    );
    expect(
      await screen.findByText('Diagnostics saved to /home/me/homeport-diagnostics.zip.'),
    ).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByRole('dialog')).not.toBeInTheDocument());
  });

  it('saves nothing when the save dialog is cancelled, and shows a failure in place', async () => {
    settingsPage({
      diagnostics_export: () => {
        throw {
          code: 'io',
          key: '',
          params: {},
          message: 'Homeport can’t write there.',
          hint: null,
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Export diagnostics…' }));
    await screen.findByText('logs/Homeport.log');

    dialogs.pickSavePath.mockResolvedValue(null);
    await user.click(screen.getByRole('button', { name: 'Save…' }));
    expect(screen.getByRole('dialog')).toBeInTheDocument();

    dialogs.pickSavePath.mockResolvedValue('/root/x.zip');
    await user.click(screen.getByRole('button', { name: 'Save…' }));
    expect(await screen.findByText('Homeport can’t write there.')).toBeInTheDocument();
    expect(screen.getByRole('dialog')).toBeInTheDocument();
  });
});
