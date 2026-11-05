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
  profiles: [],
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
    stacks_check: () => ({ errors: [], warnings: [], profiles: [] }),
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

describe('a Stack’s profiles', () => {
  it('turns on a declared profile and saves it', async () => {
    const saved: unknown[] = [];
    mock({
      stacks_check: () => ({ errors: [], warnings: [], profiles: ['debug', 'tools'] }),
      stacks_set_profiles: ({ profiles }) => {
        saved.push(profiles);
        return profiles;
      },
    });
    renderApp('/stacks/site');
    await userEvent.click(await screen.findByRole('checkbox', { name: 'debug' }));
    await userEvent.click(screen.getByRole('button', { name: 'Save profiles' }));
    await waitFor(() => expect(saved).toEqual([['debug']]));
  });

  it('says when a turned-on profile left the definition', async () => {
    mock({
      stacks_check: () => ({ errors: [], warnings: [], profiles: ['tools'] }),
      stacks_linked_read: () => ({ ...files(), profiles: ['old'] }),
    });
    renderApp('/stacks/site');
    expect(
      await screen.findByText(
        'old is no longer in the definition, so it’s turned off when you save.',
      ),
    ).toBeInTheDocument();
  });

  it('is hidden when the definition declares none', async () => {
    mock();
    renderApp('/stacks/site');
    await screen.findByRole('tab', { name: 'compose.yaml' });
    expect(screen.queryByText('Profiles')).toBeNull();
  });
});
