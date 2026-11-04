import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { RunSpec } from '@/shared/api';
import { baseCommands, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';
import { emptyRunForm, toRunSpec } from '../../model/runForm';

const web = container('web', null);
const original: RunSpec = {
  ...toRunSpec(emptyRunForm('nginx:1.27')),
  env: ['MODE=prod'],
  ports: [{ hostIp: null, hostPort: 8080, containerPort: 80, protocol: 'tcp' }],
  memory: 512 * 1024 ** 2,
  basedOn: web.id,
};

function page() {
  const sent: RunSpec[] = [];
  mockCommands({
    ...baseCommands,
    containers_list: () => [web, container('web-copy', null)],
    usage_subscribe: () => 1,
    images_list: () => [],
    networks_list: () => [],
    containers_run_spec: () => original,
    containers_run: ({ spec }) => {
      sent.push(spec as RunSpec);
      return 'copy-id';
    },
  });
  renderApp('/containers');
  return sent;
}

async function openDuplicate(user: ReturnType<typeof userEvent.setup>) {
  const row = (await screen.findByText('web')).closest('tr')!;
  await user.click(within(row).getByRole('button', { name: 'More actions' }));
  await user.click(await screen.findByRole('menuitem', { name: 'Duplicate' }));
  return screen.findByRole('dialog', { name: 'Duplicate web' });
}

describe('duplicating a container', () => {
  it('fills the Run dialog in from the container, under a free name', async () => {
    page();
    const user = userEvent.setup();
    const dialog = await openDuplicate(user);
    expect(within(dialog).getByLabelText('Name')).toHaveValue('web-copy-2');
    expect(within(dialog).getByLabelText('Environment')).toHaveValue('MODE=prod');
    expect(within(dialog).getByLabelText('Memory limit')).toHaveValue('512m');
  });

  it('flags a host port the original already publishes, until it changes', async () => {
    page();
    const user = userEvent.setup();
    const dialog = await openDuplicate(user);
    expect(within(dialog).getByText(/already publishes host port 8080/)).toBeInTheDocument();

    const hostPort = within(dialog).getByLabelText('Host port');
    await user.clear(hostPort);
    await user.type(hostPort, '8081');
    expect(within(dialog).queryByText(/already publishes host port/)).toBeNull();
  });

  it('runs the copy based on the original', async () => {
    const sent = page();
    const user = userEvent.setup();
    const dialog = await openDuplicate(user);
    await user.click(within(dialog).getByRole('button', { name: 'Run' }));
    await waitFor(() => expect(sent).toHaveLength(1));
    expect(sent[0]).toMatchObject({ name: 'web-copy-2', basedOn: web.id, env: ['MODE=prod'] });
  });
});
