import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderApp } from '@/test/render';

const image = {
  id: 'sha256:aaaa',
  tags: ['alpine:3.22'],
  digests: [],
  created: 1_700_000_000,
  size: 1024,
  containers: 0,
  labels: {},
};
const ghcr = { id: 'r1', address: 'ghcr.io', username: 'alice', createdAt: '', updatedAt: '' };

describe('pull dialog', () => {
  it('says which stored login a pull uses', async () => {
    mockCommands({
      ...baseCommands,
      images_list: () => [image],
      registries_login_for: ({ reference }) =>
        String(reference).startsWith('ghcr.io/') ? ghcr : null,
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('button', { name: 'Pull image' }));
    const input = await screen.findByLabelText('Image reference');
    await user.type(input, 'ghcr.io/org/app:1');
    expect(await screen.findByText('Pulled as alice on ghcr.io')).toBeInTheDocument();

    await user.clear(input);
    await user.type(input, 'nginx');
    expect(await screen.findByText('Pulled anonymously')).toBeInTheDocument();
  });

  it('shows a refused pull with its hint in place', async () => {
    mockCommands({
      ...baseCommands,
      images_list: () => [image],
      registries_login_for: () => null,
      images_pull_subscribe: ({ onEvent }) => {
        setTimeout(() =>
          sendToChannel(onEvent, {
            type: 'error',
            error: {
              code: 'registry_auth_required',
              message: 'The image doesn’t exist, or it’s private.',
              hint: {
                text: 'Check the image name. If it’s private, add a login for ghcr.io in Settings → Registries.',
                commands: [],
              },
              detail: null,
            },
          }),
        );
        return 1;
      },
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('button', { name: 'Pull image' }));
    await user.type(await screen.findByLabelText('Image reference'), 'ghcr.io/org/private');
    await user.click(screen.getByRole('button', { name: 'Pull' }));
    expect(await screen.findByText(/add a login for ghcr.io/)).toBeInTheDocument();
  });
});
