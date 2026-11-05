import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const dialogs = vi.hoisted(() => ({ pickFolder: vi.fn<() => Promise<string | null>>() }));
vi.mock('@/shared/lib/dialogs', () => ({
  pickFile: vi.fn(),
  pickFiles: vi.fn(),
  pickSavePath: vi.fn(),
  ...dialogs,
}));

const project = {
  workingDir: '/home/me/site',
  name: 'site',
  files: [
    { path: '/home/me/site/compose.yaml', content: 'services: {}', fingerprint: 'a' },
    { path: '/home/me/site/compose.override.yaml', content: 'services: {}', fingerprint: 'b' },
  ],
};

describe('creating a Linked Stack', () => {
  it('uses the compose files Compose would, in order', async () => {
    const created: Record<string, unknown>[] = [];
    dialogs.pickFolder.mockResolvedValue('/home/me/site');
    mockCommands({
      ...baseCommands,
      stacks_list: () => [],
      stacks_check: () => ({ errors: [], warnings: [] }),
      stacks_linked_inspect: () => project,
      stacks_linked_create: (args) => {
        created.push(args);
        return { id: 'l1', name: args.name };
      },
      stacks_linked_read: () => ({ ...project, fingerprint: 'x', changedOutside: false }),
      containers_list: () => [],
      images_freshness: () => [],
    });
    renderApp('/stacks/new');
    await userEvent.click(
      await screen.findByRole('radio', { name: 'Use a folder on this computer' }),
    );
    await userEvent.click(screen.getByRole('button', { name: 'Choose a folder…' }));
    expect(await screen.findByText('compose.override.yaml')).toBeInTheDocument();
    expect(screen.getByLabelText('Name')).toHaveValue('site');
    await userEvent.click(screen.getByRole('button', { name: 'Save' }));
    await waitFor(() => expect(created).toHaveLength(1));
    expect(created[0]).toMatchObject({
      name: 'site',
      workingDir: '/home/me/site',
      configFiles: ['/home/me/site/compose.yaml', '/home/me/site/compose.override.yaml'],
    });
  });

  it('explains a folder without compose files', async () => {
    dialogs.pickFolder.mockResolvedValue('/home/me/empty');
    mockCommands({
      ...baseCommands,
      stacks_list: () => [],
      stacks_check: () => ({ errors: [], warnings: [] }),
      stacks_linked_inspect: () => {
        throw { code: 'validation', message: '/home/me/empty has no compose file.' };
      },
    });
    renderApp('/stacks/new');
    await userEvent.click(
      await screen.findByRole('radio', { name: 'Use a folder on this computer' }),
    );
    await userEvent.click(screen.getByRole('button', { name: 'Choose a folder…' }));
    expect(await screen.findByText('/home/me/empty has no compose file.')).toBeInTheDocument();
    expect(screen.queryByRole('button', { name: 'Save' })).toBeNull();
  });
});
