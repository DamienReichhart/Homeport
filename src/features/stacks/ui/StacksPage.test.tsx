import { screen } from '@testing-library/react';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('stacks page', () => {
  it('distinguishes managed, linked and external stacks', async () => {
    mockCommands({
      ...baseCommands,
      get_capabilities: () => ({
        keychain: true,
        helm: { state: 'available', cli: { program: 'helm', version: 'v3.16.2' } },
        stackDeploy: {
          state: 'available',
          cli: { program: 'docker', prefix: [], version: '29.1.0' },
        },
        git: { state: 'available', cli: { program: 'git', version: '2.43.0' } },
        compose: {
          state: 'available',
          cli: { program: 'docker', prefix: ['compose'], version: '5.6.1' },
        },
      }),
      stacks_list: () => [
        {
          name: 'blog',
          kind: 'external',
          id: null,
          state: 'running',
          services: [{ name: 'app', containerIds: ['a'], running: 1 }],
          running: 1,
          total: 1,
          configFiles: '/srv/blog/compose.yaml',
          workingDir: null,
          problem: null,
          updateAvailable: false,
        },
        {
          name: 'docs',
          kind: 'linked',
          id: 'l1',
          state: 'running',
          services: [{ name: 'site', containerIds: ['d'], running: 1 }],
          running: 1,
          total: 1,
          configFiles: '/home/me/docs/compose.yaml',
          workingDir: '/home/me/docs',
          problem: null,
          updateAvailable: false,
        },
        {
          name: 'moved',
          kind: 'external',
          id: 'l2',
          state: 'running',
          services: [{ name: 'app', containerIds: ['m'], running: 1 }],
          running: 1,
          total: 1,
          configFiles: '/home/me/moved/compose.yaml',
          workingDir: '/home/me/moved',
          problem: 'filesMissing',
        },
        {
          name: 'shop',
          kind: 'managed',
          id: 'm1',
          state: 'notDeployed',
          services: [],
          running: 0,
          total: 0,
          configFiles: null,
          workingDir: null,
          problem: null,
          updateAvailable: false,
        },
      ],
    });
    renderApp('/stacks');
    expect(await screen.findByText('blog')).toBeInTheDocument();
    expect(screen.getAllByText('External')).toHaveLength(2);
    expect(screen.getByText('Managed')).toBeInTheDocument();
    expect(screen.getByText('Linked')).toBeInTheDocument();
    expect(screen.getByText('/home/me/docs')).toBeInTheDocument();
    expect(
      screen.getByRole('img', { name: 'Its compose files are no longer on this computer.' }),
    ).toBeInTheDocument();
    expect(screen.getByText('Not deployed')).toBeInTheDocument();
    // Only the External Stack Homeport has no record of can be adopted.
    expect(screen.getAllByRole('button', { name: 'Adopt' })).toHaveLength(1);
    expect(screen.getByRole('button', { name: /Deploy/ })).toBeEnabled();
    // The running Linked Stack redeploys from its folder.
    expect(screen.getByRole('button', { name: /Redeploy/ })).toBeEnabled();
  });
});
