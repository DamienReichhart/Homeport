import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { createQueryClient } from '@/app/providers/queryClient';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const secret = {
  view: {
    namespace: 'default',
    name: 'db',
    type: 'Opaque',
    keys: [{ key: 'password', size: 7, binary: false }],
    created: null,
  },
  labels: {},
  annotations: {},
};

describe('a Secret', () => {
  it('shows its keys masked, reveals one on demand, and never caches it', async () => {
    const revealed: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_secret_get: () => secret,
      clusters_secrets_list: () => [secret.view],
      clusters_secret_reveal: (args) => {
        revealed.push(args);
        return { value: 'hunter2', binary: false };
      },
    });
    const client = createQueryClient();
    const user = userEvent.setup();
    renderApp('/secrets/default/db', client);
    expect(await screen.findByText('password')).toBeInTheDocument();
    expect(screen.queryByText('hunter2')).not.toBeInTheDocument();
    expect(revealed).toEqual([]);

    await user.click(screen.getByRole('button', { name: 'Reveal password' }));
    expect(await screen.findByText('hunter2')).toBeInTheDocument();
    expect(revealed).toEqual([{ id: 'c1', namespace: 'default', name: 'db', key: 'password' }]);
    expect(
      JSON.stringify(
        client
          .getQueryCache()
          .getAll()
          .map((q) => q.state.data),
      ),
    ).not.toContain('hunter2');
    expect(
      JSON.stringify(
        client
          .getMutationCache()
          .getAll()
          .map((m) => m.state.data),
      ),
    ).not.toContain('hunter2');

    // Leave the page, then come back to it.
    await user.click(screen.getByRole('link', { name: /Configuration/ }));
    await user.click(await screen.findByRole('tab', { name: 'Secrets' }));
    await user.click(await screen.findByText('db'));
    expect(await screen.findByText('password')).toBeInTheDocument();
    expect(screen.queryByText('hunter2')).not.toBeInTheDocument();
  });

  it('says when the credentials can’t list Secrets, and ConfigMaps still work', async () => {
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      clusters_secrets_list: () => {
        throw { code: 'forbidden', message: 'Your credentials can’t list secrets in default.' };
      },
      clusters_config_maps_list: () => [
        { namespace: 'default', name: 'app-config', keys: [], created: null },
      ],
    });
    const user = userEvent.setup();
    renderApp('/secrets');
    expect(
      await screen.findByText('Your credentials can’t list secrets in default.'),
    ).toBeInTheDocument();
    await user.click(screen.getByRole('tab', { name: 'ConfigMaps' }));
    expect(await screen.findByText('app-config')).toBeInTheDocument();
  });
});
