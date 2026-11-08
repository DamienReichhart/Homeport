import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, connectedHost, container, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const prod = hostView('prod-server', { id: 'h2' });
const down = hostView('nas', {
  id: 'h3',
  status: {
    state: 'unreachable',
    retryInSecs: 8,
    error: { code: 'timeout', key: '', params: {}, message: 'Timed out', hint: null, detail: null },
  },
});

function setup(acted: { hostId: unknown; id: unknown; action: unknown }[] = []) {
  mockCommands({
    ...baseCommands,
    hosts_list: () => [connectedHost, prod, down],
    containers_list: ({ hostId }) =>
      hostId === 'h1' ? [container('web', null), container('db', null)] : [container('api', null)],
    containers_act: ({ hostId, id, action }) => {
      acted.push({ hostId, id, action });
      return null;
    },
    usage_subscribe: () => null,
    images_freshness: () => [],
    containers_inspect: () => {
      throw new Error('not needed');
    },
    logs_subscribe: () => null,
  });
  return userEvent.setup();
}

const row = (name: string) => screen.getByRole('row', { name: new RegExp(`\\b${name}\\b`) });

describe('board', () => {
  it('lists the containers of every connected host, each with its host', async () => {
    setup();
    renderApp('/board');
    expect(await screen.findByText('api')).toBeInTheDocument();
    expect(within(row('web')).getByText('local')).toBeInTheDocument();
    expect(within(row('api')).getByText('prod-server')).toBeInTheDocument();
    expect(screen.getByText('3 of 3 containers running on 2 hosts')).toBeInTheDocument();
    // The unreachable host is named, not shown, and doesn't block the others.
    expect(screen.getByText('nas (Unreachable) isn’t shown.')).toBeInTheDocument();
  });

  it('filters by host', async () => {
    const user = setup();
    renderApp('/board');
    await screen.findByText('api');
    await user.click(screen.getByRole('button', { name: 'Hosts' }));
    await user.click(await screen.findByRole('menuitemcheckbox', { name: 'prod-server' }));
    await user.keyboard('{Escape}');
    await waitFor(() => expect(screen.queryByText('web')).not.toBeInTheDocument());
    expect(screen.getByText('api')).toBeInTheDocument();
  });

  it('stops a selection that spans two hosts, asking each host', async () => {
    const acted: { hostId: unknown; id: unknown; action: unknown }[] = [];
    const user = setup(acted);
    renderApp('/board');
    await screen.findByText('api');
    await user.click(screen.getByRole('checkbox', { name: 'Select web on local' }));
    await user.click(screen.getByRole('checkbox', { name: 'Select api on prod-server' }));
    const bar = screen.getByRole('toolbar', { name: 'Selected containers' });
    expect(within(bar).getByText('2 selected')).toBeInTheDocument();
    await user.click(within(bar).getByRole('button', { name: 'Stop' }));
    await waitFor(() => expect(acted).toHaveLength(2));
    expect(acted).toEqual(
      expect.arrayContaining([
        { hostId: 'h1', id: 'web-0123456789abcdef', action: 'stop' },
        { hostId: 'h2', id: 'api-0123456789abcdef', action: 'stop' },
      ]),
    );
  });

  it('keeps its filters after opening a container and coming back', async () => {
    const user = setup();
    renderApp('/board?q=ap');
    await screen.findByText('api');
    expect(screen.queryByText('web')).not.toBeInTheDocument();
    await user.click(screen.getByText('api'));
    await user.click(await screen.findByRole('button', { name: 'Back' }));
    expect(await screen.findByRole('heading', { name: 'Board' })).toBeInTheDocument();
    expect(screen.getByRole('textbox', { name: 'Search every host' })).toHaveValue('ap');
    expect(screen.queryByText('web')).not.toBeInTheDocument();
  });

  it('sorts by a column', async () => {
    const user = setup();
    renderApp('/board');
    await screen.findByText('api');
    await user.click(screen.getByRole('button', { name: 'Sort by Name' }));
    const names = screen
      .getAllByRole('row')
      .slice(1)
      .map((r) => within(r).queryAllByText(/^(web|db|api)$/)[0]?.textContent);
    expect(names).toEqual(['api', 'db', 'web']);
  });
});
