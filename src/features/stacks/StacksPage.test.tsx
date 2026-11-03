import { screen } from '@testing-library/react';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('stacks page', () => {
  it('distinguishes managed and external stacks', async () => {
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
          managedId: null,
          state: 'running',
          services: [{ name: 'app', containerIds: ['a'], running: 1 }],
          running: 1,
          total: 1,
          configFiles: '/srv/blog/compose.yaml',
        },
        {
          name: 'shop',
          kind: 'managed',
          managedId: 'm1',
          state: 'notDeployed',
          services: [],
          running: 0,
          total: 0,
          configFiles: null,
        },
      ],
    });
    renderApp('/stacks');
    expect(await screen.findByText('blog')).toBeInTheDocument();
    expect(screen.getByText('External')).toBeInTheDocument();
    expect(screen.getByText('Managed')).toBeInTheDocument();
    expect(screen.getByText('Not deployed')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Adopt' })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /Deploy/ })).toBeEnabled();
  });
});
