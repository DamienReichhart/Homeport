import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const swarm = { id: 's1', managers: ['h1'], workers: [], reachable: true };
const shop = {
  name: 'shop',
  kind: 'external',
  id: null,
  state: 'partial',
  services: [
    { name: 'web', containerIds: [], running: 2, serviceId: 'svc-web', desired: 2 },
    { name: 'db', containerIds: [], running: 0, serviceId: 'svc-db', desired: 1 },
  ],
  running: 2,
  total: 3,
  configFiles: null,
  workingDir: null,
  problem: null,
  updateAvailable: false,
};

describe('a Swarm’s Stacks', () => {
  it('lists an External swarm stack and removes it from its page', async () => {
    const removed: unknown[] = [];
    mockCommands({
      ...baseCommands,
      swarms_list: () => [swarm],
      swarm_inspect: () => ({
        id: 's1',
        created: null,
        updated: null,
        nodes: 1,
        managers: 1,
        services: 2,
      }),
      swarm_stacks_list: () => [shop],
      swarm_stacks_parts: () => ({
        networks: ['shop_default'],
        secrets: [],
        configs: ['shop_conf'],
      }),
      swarm_stacks_remove: (args) => {
        removed.push(args);
        return { kept: [] };
      },
    });
    const user = userEvent.setup();
    renderApp('/swarms/s1/stacks');
    await user.click(await screen.findByText('shop'));
    expect(await screen.findByRole('link', { name: 'db' })).toHaveAttribute(
      'href',
      '/swarms/s1/services/svc-db',
    );
    expect(await screen.findByText('shop_default')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Remove' }));
    await user.click(await screen.findByRole('button', { name: 'Remove' }));
    expect(removed).toEqual([{ swarmId: 's1', name: 'shop' }]);
  });
});
