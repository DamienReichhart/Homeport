import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Loss, RunSpec } from '@/shared/api';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';
import { emptyRunForm, toRunSpec } from '../../model/runForm';

const web = container('web', null);
const original: RunSpec = {
  ...toRunSpec(emptyRunForm('nginx:1.27')),
  env: ['MODE=prod'],
  basedOn: web.id,
};

function page({ losses = [], fail = false }: { losses?: Loss[]; fail?: boolean } = {}) {
  const sent: { id: unknown; spec: RunSpec }[] = [];
  mockCommands({
    ...baseCommands,
    containers_list: () => [web],
    usage_subscribe: () => 1,
    images_list: () => [],
    networks_list: () => [],
    containers_run_spec: () => original,
    containers_edit_losses: () => losses,
    containers_recreate: ({ id, spec }) => {
      sent.push({ id, spec: spec as RunSpec });
      if (fail)
        throw {
          code: 'port_in_use',
          message: 'Port 8080 is already in use.',
          hint: null,
          detail: null,
        };
      return 'new-id';
    },
    logs_subscribe: () => 1,
    containers_inspect: () => ({}),
  });
  renderApp('/containers');
  return sent;
}

async function editAndApply(user: ReturnType<typeof userEvent.setup>) {
  const row = (await screen.findByText('web')).closest('tr')!;
  await user.click(within(row).getByRole('button', { name: 'More actions' }));
  await user.click(await screen.findByRole('menuitem', { name: 'Edit' }));
  const dialog = await screen.findByRole('dialog', { name: 'Edit web' });
  expect(within(dialog).getByLabelText('Name')).toBeDisabled();
  const env = within(dialog).getByLabelText('Environment');
  await user.clear(env);
  await user.type(env, 'MODE=staging');
  await user.click(within(dialog).getByRole('button', { name: 'Apply' }));
  return dialog;
}

describe('editing a container', () => {
  it('says what is lost, then recreates it with the new settings', async () => {
    const sent = page({
      losses: [
        { kind: 'anonymousVolume', path: '/var/cache' },
        { kind: 'filesystemChanges', count: 4 },
        { kind: 'downtime' },
        { kind: 'stackRedeploy', stack: 'shop' },
      ],
    });
    const user = userEvent.setup();
    await editAndApply(user);

    const confirmation = await screen.findByRole('alertdialog', { name: 'Recreate web?' });
    expect(
      await within(confirmation).findByText(/anonymous volume at \/var\/cache/),
    ).toBeInTheDocument();
    expect(within(confirmation).getByText(/4 files changed/)).toBeInTheDocument();
    expect(within(confirmation).getByText(/the shop Stack/)).toBeInTheDocument();
    await user.click(within(confirmation).getByRole('button', { name: 'Recreate' }));

    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]).toMatchObject({ id: web.id, spec: { env: ['MODE=staging'], basedOn: web.id } });
    expect(await screen.findByText('Container recreated')).toBeInTheDocument();
  });

  it('says nothing else is lost when nothing is', async () => {
    page();
    const user = userEvent.setup();
    await editAndApply(user);
    const confirmation = await screen.findByRole('alertdialog');
    expect(await within(confirmation).findByText(/Nothing else is lost/)).toBeInTheDocument();
  });

  it('shows a failed recreate in the form, the original untouched', async () => {
    page({ fail: true });
    const user = userEvent.setup();
    const dialog = await editAndApply(user);
    const confirmation = await screen.findByRole('alertdialog');
    await user.click(await within(confirmation).findByRole('button', { name: 'Recreate' }));

    expect(await within(dialog).findByText('Port 8080 is already in use.')).toBeInTheDocument();
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(within(dialog).getByRole('button', { name: 'Apply' })).toBeEnabled();
  });
});
