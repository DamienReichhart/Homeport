import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { nomadClusterView } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

function setup(extra: Record<string, (args: Record<string, unknown>) => unknown> = {}) {
  const calls: { command: string; args: Record<string, unknown> }[] = [];
  const record = (command: string, answer: unknown) => (args: Record<string, unknown>) => {
    calls.push({ command, args });
    return answer;
  };
  mockCommands({
    ...baseCommands,
    clusters_kubeconfig_contexts: () => ({ paths: [], contexts: [] }),
    clusters_list: () => [nomadClusterView('nomad-dev')],
    clusters_nomad_test: record('test', {
      version: '1.10.3',
      region: 'global',
      datacenter: 'dc1',
      leader: '10.0.0.5:4647',
    }),
    clusters_nomad_add: record('add', nomadClusterView('nomad-dev')),
    clusters_nomad_update: record('update', nomadClusterView('nomad-dev')),
    ...extra,
  });
  return calls;
}

describe('adding a Nomad cluster', () => {
  it('tests the connection, then adds it with its token', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp('/clusters/new');
    await user.click(await screen.findByRole('tab', { name: 'Nomad' }));
    await user.type(screen.getByLabelText('Name'), 'nomad-dev');
    await user.type(screen.getByLabelText('Address'), '127.0.0.1');
    await user.type(screen.getByLabelText('ACL token'), 'secret-id');
    await user.click(screen.getByRole('button', { name: 'Test connection' }));
    expect(
      await screen.findByText('Connected to Nomad 1.10.3, region global.'),
    ).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Add cluster' }));
    await waitFor(() => expect(calls.some((c) => c.command === 'add')).toBe(true));
    expect(calls.find((c) => c.command === 'add')?.args.form).toMatchObject({
      name: 'nomad-dev',
      address: '127.0.0.1',
      token: 'secret-id',
      certificateAuthority: null,
    });
  });

  it('shows why a test failed', async () => {
    setup({
      clusters_nomad_test: () => {
        throw {
          code: 'nomad_token_invalid',
          key: 'errors.nomad.tokenInvalid',
          params: { name: 'nomad-dev' },
          message: "nomad-dev doesn't know the ACL token Homeport sent.",
          hint: null,
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/clusters/new');
    await user.click(await screen.findByRole('tab', { name: 'Nomad' }));
    await user.type(screen.getByLabelText('Name'), 'nomad-dev');
    await user.type(screen.getByLabelText('Address'), '127.0.0.1');
    await user.click(screen.getByRole('button', { name: 'Test connection' }));
    expect(
      await screen.findByText("nomad-dev doesn't know the ACL token Homeport sent."),
    ).toBeInTheDocument();
  });
});

describe('editing a Nomad cluster', () => {
  it('keeps the stored token unless it is replaced or removed', async () => {
    const calls = setup();
    const user = userEvent.setup();
    renderApp('/clusters/n1');
    expect(
      await screen.findByText('http://127.0.0.1:4646', { selector: 'span' }),
    ).toBeInTheDocument();
    const token = screen.getByLabelText('ACL token');
    expect(token).toHaveValue('');
    expect(token).toHaveAttribute('placeholder', 'Saved; leave empty to keep it');
    await user.click(screen.getByRole('button', { name: 'Save changes' }));
    await waitFor(() => expect(calls.some((c) => c.command === 'update')).toBe(true));
    expect(calls.find((c) => c.command === 'update')?.args).toMatchObject({
      id: 'n1',
      form: { name: 'nomad-dev', token: null },
    });

    await user.click(screen.getByRole('button', { name: 'Remove' }));
    await user.click(screen.getByRole('button', { name: 'Save changes' }));
    await waitFor(() => expect(calls.filter((c) => c.command === 'update')).toHaveLength(2));
    expect(calls.filter((c) => c.command === 'update')[1]?.args).toMatchObject({
      form: { token: '' },
    });
  });
});
