import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

describe('images page', () => {
  it('lists images with their usage', async () => {
    mockCommands({
      ...baseCommands,
      images_list: () => [
        {
          id: 'sha256:aaaaaaaaaaaaaaaa',
          tags: ['nginx:1.27', 'nginx:latest'],
          digests: [],
          created: 1_700_000_000,
          size: 190 * 1024 ** 2,
          containers: 2,
          labels: {},
        },
        {
          id: 'sha256:bbbbbbbbbbbbbbbb',
          tags: [],
          digests: [],
          created: 1_700_000_000,
          size: 1024,
          containers: 0,
          labels: {},
        },
      ],
    });
    renderApp('/images');
    expect(await screen.findByText('nginx:1.27')).toBeInTheDocument();
    expect(screen.getByText('+1 more tag')).toBeInTheDocument();
    expect(screen.getByText('Used by 2 containers')).toBeInTheDocument();
    expect(screen.getByText('Unused')).toBeInTheDocument();
    expect(screen.getByText('<none>')).toBeInTheDocument();
    expect(screen.getByText('2 images · 190.0 MB')).toBeInTheDocument();
  });

  it('reports a clean-up that fails and keeps the confirmation open', async () => {
    mockCommands({
      ...baseCommands,
      images_list: () => [],
      images_prune: () => {
        throw {
          code: 'engine_conflict',
          message: 'The Docker Engine is already cleaning up images.',
          hint: null,
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('button', { name: 'More actions' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Dangling images' }));
    expect(await screen.findByText('Remove dangling images on local?')).toBeInTheDocument();
    const confirmation = await screen.findByRole('alertdialog');
    await user.click(within(confirmation).getByRole('button', { name: 'Clean up' }));

    const message = await screen.findByText('The Docker Engine is already cleaning up images.');
    expect(message.closest('[data-sonner-toast]')).not.toBeNull();
    expect(screen.getByRole('alertdialog')).toBeInTheDocument();
  });

  it('marks an image whose tag has a newer version in amber', async () => {
    mockCommands({
      ...baseCommands,
      images_list: () => [
        {
          id: 'sha256:aaaa',
          tags: ['nginx:1.27'],
          digests: ['nginx@sha256:old'],
          created: 1_700_000_000,
          size: 1024,
          containers: 1,
          labels: {},
        },
      ],
      images_freshness: () => [
        { reference: 'nginx:1.27', imageId: 'sha256:aaaa', freshness: 'newer' },
      ],
    });
    renderApp('/images');
    const mark = await screen.findByLabelText('A newer nginx:1.27 was published');
    expect(mark).toHaveTextContent('Newer version');
    expect(mark).toHaveAttribute('data-variant', 'warning');

    // The row's menu offers to pull it, with the reference filled in.
    const user = userEvent.setup();
    await user.click(screen.getByRole('button', { name: 'More actions for nginx:1.27' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Pull the newer nginx:1.27' }));
    const dialog = await screen.findByRole('dialog', { name: 'Pull an image' });
    expect(within(dialog).getByLabelText('Image reference')).toHaveValue('nginx:1.27');
  });

  it('keeps the removal open when the image is in use, and retries it', async () => {
    let attempts = 0;
    const image = {
      id: 'sha256:aaaaaaaaaaaaaaaa',
      tags: ['nginx:1.27'],
      digests: [],
      created: 1_700_000_000,
      size: 1024,
      containers: 1,
      labels: {},
    };
    mockCommands({
      ...baseCommands,
      images_list: () => [image],
      images_remove: () => {
        attempts += 1;
        if (attempts === 1)
          throw {
            code: 'conflict',
            message: 'nginx:1.27 is used by a container.',
            hint: null,
            detail: null,
          };
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/images');
    await user.click(await screen.findByRole('button', { name: 'More actions for nginx:1.27' }));
    await user.click(await screen.findByRole('menuitem', { name: /Remove/ }));
    const confirmation = await screen.findByRole('alertdialog');
    const remove = within(confirmation).getByRole('button', { name: 'Remove' });
    await user.click(remove);

    expect(await screen.findByText('nginx:1.27 is used by a container.')).toBeInTheDocument();
    await waitFor(() => expect(remove).toBeEnabled());
    expect(screen.getByRole('alertdialog')).toBeInTheDocument();

    await user.click(remove);
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(attempts).toBe(2);
  });
});
