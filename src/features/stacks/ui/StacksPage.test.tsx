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
        },
      ],
    });
    renderApp('/stacks');
    expect(await screen.findByText('blog')).toBeInTheDocument();
    expect(screen.getByText('External')).toBeInTheDocument();
    expect(screen.getByText('Managed')).toBeInTheDocument();
    expect(screen.getByText('Linked')).toBeInTheDocument();
    expect(screen.getByText('Not deployed')).toBeInTheDocument();
    // Only the External Stack can be adopted.
    expect(screen.getAllByRole('button', { name: 'Adopt' })).toHaveLength(1);
    expect(screen.getByRole('button', { name: /Deploy/ })).toBeEnabled();
  });
});
