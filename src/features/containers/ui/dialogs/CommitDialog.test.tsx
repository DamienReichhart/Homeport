import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { CommitSpec } from '@/shared/api';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const web = container('web', null);

function page(fail = false) {
  const sent: CommitSpec[] = [];
  let imageLists = 0;
  mockCommands({
    ...baseCommands,
    containers_list: () => [web],
    usage_subscribe: () => 1,
    networks_list: () => [],
    images_list: () => {
      imageLists += 1;
      return [
        {
          id: 'sha256:old',
          tags: ['web:backup'],
          digests: [],
          created: 0,
          size: 0,
          containers: 0,
          labels: {},
        },
      ];
    },
    containers_commit: ({ spec }) => {
      sent.push(spec as CommitSpec);
      if (fail)
        throw {
          code: 'disk_full',
          message: 'The Docker Engine is out of disk space.',
          hint: null,
          detail: null,
        };
      return 'sha256:new';
    },
  });
  renderApp('/containers');
  return { sent, imageLists: () => imageLists };
}

async function openCommit(user: ReturnType<typeof userEvent.setup>) {
  const row = (await screen.findByText('web')).closest('tr')!;
  await user.click(within(row).getByRole('button', { name: 'More actions' }));
  await user.click(await screen.findByRole('menuitem', { name: 'Commit to image…' }));
  return screen.findByRole('dialog', { name: 'Commit web to an image' });
}

describe('committing a container', () => {
  it('commits under a dated name with the changes, pausing by default', async () => {
    const { sent, imageLists } = page();
    const user = userEvent.setup();
    const dialog = await openCommit(user);
    const name = within(dialog).getByLabelText('Image name');
    expect(name).toHaveValue(`web:${new Date().toISOString().slice(0, 10)}`);
    expect(within(dialog).getByRole('switch', { name: /Pause/ })).toBeChecked();

    await user.type(within(dialog).getByLabelText('Message'), 'Before the upgrade');
    await user.type(within(dialog).getByLabelText('Changes'), 'ENV MODE=prod');
    const before = imageLists();
    await user.click(within(dialog).getByRole('button', { name: 'Commit' }));

    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]).toMatchObject({
      comment: 'Before the upgrade',
      pause: true,
      changes: ['ENV MODE=prod'],
    });
    expect(await screen.findByText(/Committed to web:/)).toBeInTheDocument();
    await waitFor(() => expect(imageLists()).toBeGreaterThan(before));
  });

  it('says when the name moves from another image', async () => {
    page();
    const user = userEvent.setup();
    const dialog = await openCommit(user);
    const name = within(dialog).getByLabelText('Image name');
    await user.clear(name);
    await user.type(name, 'web:backup');
    expect(await within(dialog).findByText(/already named web:backup/)).toBeInTheDocument();
  });

  it('shows a failed commit in place', async () => {
    page(true);
    const user = userEvent.setup();
    const dialog = await openCommit(user);
    await user.click(within(dialog).getByRole('button', { name: 'Commit' }));
    expect(
      await within(dialog).findByText('The Docker Engine is out of disk space.'),
    ).toBeInTheDocument();
  });
});
