import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Settings } from '@/shared/api';
import { baseCommands, connectedHost, container, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const prod = hostView('prod-server', { id: 'h2' });
const down = hostView('nas', { id: 'h3', status: { state: 'disabled' } });

function setup(listed: string[] = [], saved: Settings[] = []) {
  const list =
    (name: string, result: (hostId: unknown) => unknown) =>
    ({ hostId }: Record<string, unknown>) => {
      listed.push(`${name}:${String(hostId)}`);
      return result(hostId);
    };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [connectedHost, prod, down],
    update_settings: ({ settings }) => {
      saved.push(settings as Settings);
      return settings;
    },
    containers_list: list('containers', (hostId) =>
      hostId === 'h2' ? [container('shop-postgres-1', 'shop')] : [container('web', null)],
    ),
    stacks_list: list('stacks', () => []),
    images_list: list('images', () => []),
    volumes_list: list('volumes', (hostId) =>
      hostId === 'h1'
        ? [
            {
              name: 'pgdata',
              driver: 'local',
              mountpoint: '',
              createdAt: null,
              labels: {},
              stack: null,
              usedBy: [],
            },
          ]
        : [],
    ),
    volumes_sizes: () => [],
    networks_list: list('networks', () => []),
    usage_subscribe: () => null,
    logs_subscribe: () => null,
    containers_inspect: () => {
      throw new Error('not needed');
    },
  });
  return userEvent.setup();
}

const palette = () => screen.findByRole('dialog', { name: 'Command palette' });

describe('searching every host from the palette', () => {
  it('finds a container on another host, switches to it and opens it', async () => {
    const saved: Settings[] = [];
    const user = setup([], saved);
    renderApp('/settings');
    await screen.findByRole('heading', { name: 'Settings' });
    await user.keyboard('{Control>}k{/Control}');
    const dialog = await palette();
    await user.keyboard('postgres');
    const group = await within(dialog).findByRole('group', { name: 'Containers' });
    const hit = within(group).getByRole('option', { name: /shop-postgres-1/ });
    expect(within(hit).getByText('prod-server · running')).toBeInTheDocument();
    await user.click(hit);
    await waitFor(() => expect(saved.at(-1)?.lastHostId).toBe('h2'));
    expect(await screen.findByRole('heading', { name: /shop-postgres-1/ })).toBeInTheDocument();
  });

  it('loads nothing until two characters are typed, and counts the hosts it skipped', async () => {
    const listed: string[] = [];
    const user = setup(listed);
    renderApp('/settings');
    await screen.findByRole('heading', { name: 'Settings' });
    await user.keyboard('{Control>}k{/Control}');
    const dialog = await palette();
    await user.keyboard('p');
    expect(listed.filter((l) => !l.startsWith('containers'))).toEqual([]);
    await user.keyboard('g');
    expect(await within(dialog).findByText('pgdata')).toBeInTheDocument();
    expect(listed).toEqual(expect.arrayContaining(['networks:h1', 'networks:h2']));
    expect(listed.some((l) => l.endsWith(':h3'))).toBe(false);
    expect(within(dialog).getByText('1 host not searched (not connected)')).toBeInTheDocument();
  });

  it('opens a volume’s list, searched for it', async () => {
    const user = setup();
    renderApp('/settings');
    await screen.findByRole('heading', { name: 'Settings' });
    await user.keyboard('{Control>}k{/Control}');
    await palette();
    await user.keyboard('pgdata');
    await user.click(await screen.findByRole('option', { name: /pgdata/ }));
    expect(await screen.findByRole('heading', { name: 'Volumes' })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: 'Search volumes' })).toHaveValue('pgdata');
  });
});
