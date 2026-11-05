import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ImportDraft } from '@/shared/api';
import { baseCommands, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const draft = (source: string, overrides: Partial<ImportDraft> = {}): ImportDraft => ({
  source,
  name: source,
  connection: {
    kind: 'ssh',
    host: `${source}.example.com`,
    port: 22,
    user: 'deploy',
    socketPath: '/var/run/docker.sock',
    auth: 'imported',
    publicKey: null,
    bastions: [],
  },
  files: [`/home/me/.ssh/${source}`],
  needsPassphrase: false,
  problem: null,
  warnings: [],
  nameTaken: false,
  ...overrides,
});

const loop = {
  code: 'validation' as const,
  key: 'errors.hosts.import.jumpLoop',
  params: { alias: 'a' },
  message: 'Its ProxyJump chain makes a loop through a.',
  hint: null,
  detail: null,
};

function setup(drafts: ImportDraft[], failFor: string[] = []) {
  const sent: unknown[] = [];
  mockCommands({
    ...baseCommands,
    hosts_detect_local: () => [],
    hosts_ssh_config_preview: () => drafts,
    hosts_ssh_config_import: ({ choices }) => {
      sent.push(choices);
      const list = choices as { source: string; name: string }[];
      return {
        created: list.filter((c) => !failFor.includes(c.source)).map((c) => hostView(c.name)),
        failed: list
          .filter((c) => failFor.includes(c.source))
          .map((c) => ({
            source: c.source,
            error: { ...loop, key: '', message: `${c.source} refused.` },
          })),
      };
    },
  });
  renderApp('/hosts/new');
  return sent;
}

describe('import hosts dialog', () => {
  it('previews the SSH config and imports what was picked', async () => {
    const sent = setup([
      draft('prod', {
        connection: {
          ...(draft('prod').connection as Extract<ImportDraft['connection'], { kind: 'ssh' }>),
          bastions: [
            {
              host: 'jump.example.com',
              port: 22,
              user: 'me',
              auth: 'sameAsHost',
              publicKey: null,
              hostKey: null,
            },
          ],
        },
      }),
      draft('a', { connection: null, problem: loop, files: [] }),
      draft('staging', { nameTaken: true }),
    ]);
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Import from ~/.ssh/config' }));
    const dialog = await screen.findByRole('dialog');
    expect(await within(dialog).findByText('deploy@prod.example.com')).toBeInTheDocument();
    expect(within(dialog).getByText('through jump.example.com')).toBeInTheDocument();
    expect(within(dialog).getByText(loop.message)).toBeInTheDocument();
    expect(within(dialog).getByLabelText('Import a')).toBeDisabled();
    expect(within(dialog).getByLabelText('Import staging')).not.toBeChecked();

    const socket = within(dialog).getByLabelText('Remote socket path of prod');
    await user.clear(socket);
    await user.type(socket, '/run/user/1000/docker.sock');
    await user.click(within(dialog).getByRole('button', { name: 'Import 1 host' }));

    await waitFor(() => expect(screen.queryByRole('dialog')).toBeNull());
    expect(sent).toEqual([
      [
        {
          source: 'prod',
          name: 'prod',
          socketPath: '/run/user/1000/docker.sock',
          passphrase: null,
        },
      ],
    ]);
  });

  it('asks for a passphrase and a free name before importing', async () => {
    setup([draft('enc', { needsPassphrase: true }), draft('staging', { nameTaken: true })]);
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Import from ~/.ssh/config' }));
    const dialog = await screen.findByRole('dialog');
    const submit = await within(dialog).findByRole('button', { name: 'Import 1 host' });
    expect(submit).toBeDisabled();
    expect(within(dialog).getByText('Enter the key’s passphrase.')).toBeInTheDocument();
    await user.type(within(dialog).getByLabelText('Passphrase for the key of enc'), 'secret');
    expect(submit).toBeEnabled();

    await user.click(within(dialog).getByLabelText('Import staging'));
    expect(within(dialog).getByText('Pick another name.')).toBeInTheDocument();
    expect(within(dialog).getByRole('button', { name: 'Import 2 hosts' })).toBeDisabled();
    const name = within(dialog).getByLabelText('Host name for staging');
    await user.clear(name);
    await user.type(name, 'staging-2');
    expect(within(dialog).getByRole('button', { name: 'Import 2 hosts' })).toBeEnabled();
  });

  it('keeps what failed in the dialog, with why', async () => {
    const sent = setup([draft('prod'), draft('broken')], ['broken']);
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Import from ~/.ssh/config' }));
    const dialog = await screen.findByRole('dialog');
    await user.click(await within(dialog).findByRole('button', { name: 'Import 2 hosts' }));
    expect(await within(dialog).findByText('broken refused.')).toBeInTheDocument();
    expect(within(dialog).getByText('broken wasn’t imported:')).toBeInTheDocument();
    expect(within(dialog).queryByLabelText('Import prod')).toBeNull();
    expect(sent).toHaveLength(1);
  });

  it('says when the Docker CLI has no context to import', async () => {
    mockCommands({
      ...baseCommands,
      hosts_detect_local: () => [],
      hosts_docker_contexts_preview: () => [],
    });
    renderApp('/hosts/new');
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Import Docker contexts' }));
    expect(
      await screen.findByText('The Docker CLI has no context besides default.'),
    ).toBeInTheDocument();
  });
});
