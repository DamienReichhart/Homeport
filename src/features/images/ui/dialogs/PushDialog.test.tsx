import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

const image = {
  id: 'sha256:aaaa',
  tags: ['app:dev'],
  digests: [],
  created: 1_700_000_000,
  size: 1024,
  containers: 0,
  labels: {},
};
const ghcr = { id: 'r1', address: 'ghcr.io', username: 'alice', createdAt: '', updatedAt: '' };

async function openPush() {
  const user = userEvent.setup();
  renderApp('/images');
  await user.click(await screen.findByRole('button', { name: 'More actions' }));
  await user.click(await screen.findByRole('menuitem', { name: 'Push' }));
  return { user, dialog: await screen.findByRole('dialog') };
}

describe('push dialog', () => {
  it('warns when no login is stored for the target registry', async () => {
    mockCommands({ ...baseCommands, images_list: () => [image], registries_login_for: () => null });
    const { dialog } = await openPush();
    expect(within(dialog).getByLabelText('Name to push')).toHaveValue('app:dev');
    expect(await within(dialog).findByText('No login for docker.io')).toBeInTheDocument();
  });

  it('tags a new name first, then pushes it with the stored login', async () => {
    const calls: string[] = [];
    mockCommands({
      ...baseCommands,
      images_list: () => [image],
      registries_login_for: () => ghcr,
      images_tag: ({ target }) => {
        calls.push(`tag ${String(target)}`);
        return null;
      },
      images_push_subscribe: ({ reference, onEvent }) => {
        calls.push(`push ${String(reference)}`);
        setTimeout(() =>
          sendToChannel(
            onEvent,
            { type: 'progress', id: null, status: 'Pushed', current: null, total: null },
            { type: 'done' },
          ),
        );
        return 1;
      },
    });
    const { user, dialog } = await openPush();
    const input = within(dialog).getByLabelText('Name to push');
    await user.clear(input);
    await user.type(input, 'ghcr.io/org/app:1');
    expect(await within(dialog).findByText('Pushed as alice on ghcr.io')).toBeInTheDocument();
    expect(within(dialog).getByText(/given this name first/)).toBeInTheDocument();
    expect(within(dialog).queryByText(/No login for/)).not.toBeInTheDocument();

    await user.click(within(dialog).getByRole('button', { name: 'Push' }));
    expect(await within(dialog).findByText('Done.')).toBeInTheDocument();
    expect(within(dialog).getByText('1 layer pushed')).toBeInTheDocument();
    expect(calls).toEqual(['tag ghcr.io/org/app:1', 'push ghcr.io/org/app:1']);
  });
});
