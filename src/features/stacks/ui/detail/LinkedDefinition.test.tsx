import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { LinkedFiles, Stack } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const site: Stack = {
  name: 'site',
  kind: 'linked',
  id: 'l1',
  state: 'notDeployed',
  services: [],
  running: 0,
  total: 0,
  configFiles: null,
  workingDir: '/home/me/site',
  problem: null,
};

const files = (changedOutside = false): LinkedFiles => ({
  workingDir: '/home/me/site',
  fingerprint: 'all',
  changedOutside,
  files: [
    {
      path: '/home/me/site/compose.yaml',
      content: 'services:\n  web:\n    build: .\n',
      fingerprint: 'f1',
    },
  ],
});

function mock(overrides: Record<string, (args: Record<string, unknown>) => unknown> = {}) {
  mockCommands({
    ...baseCommands,
    containers_list: () => [],
    images_freshness: () => [],
    stacks_list: () => [site],
    stacks_check: () => ({ errors: [], warnings: [] }),
    stacks_linked_read: () => files(),
    ...overrides,
  });
}

describe('a Linked Stack’s definition', () => {
  it('shows its compose files from the folder, and that Compose reads its .env', async () => {
    mock();
    renderApp('/stacks/site');
    expect(await screen.findByRole('tab', { name: 'compose.yaml' })).toBeInTheDocument();
    expect(screen.getByText(/Compose reads the folder’s .env/)).toBeInTheDocument();
    expect(screen.queryByLabelText('Environment')).toBeNull();
  });

  it('says when the files changed outside Homeport', async () => {
    mock({ stacks_linked_read: ({ acknowledge }) => files(!acknowledge) });
    renderApp('/stacks/site');
    expect(await screen.findByText('Changed outside Homeport')).toBeInTheDocument();
    await userEvent.click(screen.getByRole('button', { name: 'Reload' }));
    await waitFor(() => expect(screen.queryByText('Changed outside Homeport')).toBeNull());
  });

  it('explains a missing folder', async () => {
    mock({
      stacks_linked_read: () => {
        throw {
          code: 'stack_folder_missing',
          message: 'The folder /home/me/site is no longer on this computer.',
        };
      },
    });
    renderApp('/stacks/site');
    expect(
      await screen.findByText('The folder /home/me/site is no longer on this computer.'),
    ).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Forget' })).toBeInTheDocument();
  });
});
