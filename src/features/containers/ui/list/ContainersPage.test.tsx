import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('containers page', () => {
  beforeEach(() =>
    mockCommands({
      ...baseCommands,
      containers_list: () => [
        container('shop-web-1', 'shop'),
        container('shop-db-1', 'shop', 'exited'),
        container('redis', null),
      ],
      usage_subscribe: () => 1,
      images_list: () => [],
      networks_list: () => [],
    }),
  );

  it('nests stack containers under their stack', async () => {
    renderApp('/containers');
    const stack = await screen.findByText('shop');
    expect(within(stack.closest('tr')!).getByText('1/2 running')).toBeInTheDocument();
    expect(screen.getByText('web')).toBeInTheDocument();
    expect(screen.getByText('redis')).toBeInTheDocument();
    expect(screen.getByText('2 running of 3')).toBeInTheDocument();
  });

  it('collapses a stack and filters by search', async () => {
    const user = userEvent.setup();
    renderApp('/containers');
    await user.click(await screen.findByText('shop'));
    expect(screen.queryByText('web')).not.toBeInTheDocument();
    await user.type(screen.getByLabelText('Search containers'), 'redis');
    expect(screen.queryByText('shop')).not.toBeInTheDocument();
    expect(screen.getByText('redis')).toBeInTheDocument();
  });

  it('opens a container and toggles a stack from the keyboard', async () => {
    const user = userEvent.setup();
    renderApp('/containers');
    const toggle = await screen.findByRole('button', { name: 'shop' });
    expect(toggle).toHaveAttribute('aria-expanded', 'true');
    toggle.focus();
    await user.keyboard('{Enter}');
    expect(toggle).toHaveAttribute('aria-expanded', 'false');
    expect(screen.queryByText('web')).not.toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'redis' })).toHaveAttribute(
      'href',
      expect.stringMatching(/\/containers\/.+\/logs$/),
    );
  });
});
