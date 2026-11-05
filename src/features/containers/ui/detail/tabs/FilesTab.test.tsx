import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, fileEntries } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { FilesTab } from './FilesTab';

const dialogs = vi.hoisted(() => ({
  pickSavePath: vi.fn<() => Promise<string | null>>(),
  pickFiles: vi.fn<() => Promise<string[]>>(),
}));
vi.mock('@/shared/lib/dialogs', () => ({
  pickFile: vi.fn(),
  pickFolder: vi.fn(),
  ...dialogs,
}));

function listing() {
  const listed: string[] = [];
  const downloads: Record<string, unknown>[] = [];
  const uploads: Record<string, unknown>[] = [];
  mockCommands({
    ...baseCommands,
    files_list: ({ path }) => {
      listed.push(path as string);
      return fileEntries(path as string);
    },
    files_download: (args) => {
      downloads.push(args);
      return false;
    },
    files_upload: (args) => {
      uploads.push(args);
      return null;
    },
  });
  return { listed, downloads, uploads };
}

describe('files tab', () => {
  beforeEach(() => {
    dialogs.pickSavePath.mockReset();
    dialogs.pickFiles.mockReset();
  });

  it('lists a directory, opens a folder and goes back up', async () => {
    const { listed } = listing();
    const user = userEvent.setup();
    renderWithProviders(<FilesTab hostId="h1" containerId="web-1" running />);

    expect(await screen.findByText('hello.txt')).toBeInTheDocument();
    expect(screen.getByText('2.0 KB')).toBeInTheDocument();
    await user.click(screen.getByText('etc'));
    await waitFor(() => expect(listed).toEqual(['/', '/etc']));
    const path = screen.getByRole('navigation', { name: 'Path' });
    expect(
      within(path)
        .getAllByRole('button')
        .map((b) => b.textContent),
    ).toEqual(['/', 'etc']);

    await user.click(within(path).getByRole('button', { name: '/' }));
    await waitFor(() =>
      expect(
        within(path)
          .getAllByRole('button')
          .map((b) => b.textContent),
      ).toEqual(['/']),
    );
  });

  it('downloads to the chosen place, and a folder as a tar', async () => {
    const { downloads } = listing();
    const user = userEvent.setup();
    dialogs.pickSavePath.mockResolvedValue('/home/me/etc.tar');
    renderWithProviders(<FilesTab hostId="h1" containerId="web-1" running />);

    await user.click(await screen.findByRole('button', { name: 'Download etc' }));
    await waitFor(() => expect(downloads).toHaveLength(1));
    expect(dialogs.pickSavePath).toHaveBeenCalledWith('Save etc', 'etc.tar');
    expect(downloads[0]).toMatchObject({ path: '/etc', destination: '/home/me/etc.tar' });
  });

  it('does nothing when the save dialog is cancelled', async () => {
    const { downloads } = listing();
    const user = userEvent.setup();
    dialogs.pickSavePath.mockResolvedValue(null);
    renderWithProviders(<FilesTab hostId="h1" containerId="web-1" running />);
    await user.click(await screen.findByRole('button', { name: 'Download hello.txt' }));
    expect(dialogs.pickSavePath).toHaveBeenCalledWith('Save hello.txt', 'hello.txt');
    expect(downloads).toHaveLength(0);
  });

  it('uploads into the current directory and lists it again', async () => {
    const { listed, uploads } = listing();
    const user = userEvent.setup();
    dialogs.pickFiles.mockResolvedValue(['/home/me/a.conf', '/home/me/b.conf']);
    renderWithProviders(<FilesTab hostId="h1" containerId="web-1" running />);
    await screen.findByText('hello.txt');

    await user.click(screen.getByRole('button', { name: 'Upload here' }));
    expect(await screen.findByText('Uploaded 2 files to /')).toBeInTheDocument();
    expect(uploads[0]).toMatchObject({
      directory: '/',
      sources: ['/home/me/a.conf', '/home/me/b.conf'],
    });
    expect(listed.filter((p) => p === '/')).toHaveLength(2);
  });

  it('explains a path that no longer exists', async () => {
    mockCommands({
      ...baseCommands,
      files_list: () => {
        throw {
          code: 'path_not_found',
          message: '/ doesn’t exist in web.',
          hint: null,
          detail: null,
        };
      },
    });
    renderWithProviders(<FilesTab hostId="h1" containerId="web-1" running />);
    expect(await screen.findByText('/ doesn’t exist in web.')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Retry' })).toBeInTheDocument();
  });

  it('asks to start a stopped container', () => {
    const { listed } = listing();
    renderWithProviders(<FilesTab hostId="h1" containerId="web-1" running={false} />);
    expect(screen.getByText('Start the container to browse its files.')).toBeInTheDocument();
    expect(listed).toHaveLength(0);
  });
});
