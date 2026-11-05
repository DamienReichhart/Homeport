import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { HostStatus } from '@/shared/api';
import { baseCommands, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const PRESENTED = 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPresented';

function renderHost(
  status: HostStatus,
  extra: Record<string, (a: Record<string, unknown>) => unknown> = {},
) {
  const trusted: unknown[] = [];
  const removed: unknown[] = [];
  mockCommands({
    ...baseCommands,
    hosts_list: () => [hostView('prod', { status })],
    hosts_ssh_setup: () => ({
      publicKey: 'ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIHomeport homeport@test',
      fingerprint: 'SHA256:homeport',
      installCommand: "echo 'ssh-ed25519 AAAA' >> ~/.ssh/authorized_keys",
      dockerGroupCommand: 'sudo usermod -aG docker deploy',
    }),
    hosts_trust_key: (args) => {
      trusted.push(args);
      return hostView('prod');
    },
    hosts_delete: ({ id }) => {
      removed.push(id);
      return null;
    },
    ...extra,
  });
  renderApp('/hosts/prod-id');
  return { trusted, removed };
}

describe('trust prompt', () => {
  it('shows an unknown server key’s fingerprint and trusts that key', async () => {
    const { trusted } = renderHost({
      state: 'hostKeyUntrusted',
      key: PRESENTED,
      fingerprint: 'SHA256:presented',
    });
    expect(await screen.findByText('Is this the right server?')).toBeInTheDocument();
    expect(screen.getAllByText('SHA256:presented').length).toBeGreaterThan(0);
    await userEvent.setup().click(screen.getByRole('button', { name: 'Trust this key' }));
    await waitFor(() => expect(trusted).toEqual([{ id: 'prod-id', key: PRESENTED }]));
  });

  it('asks for confirmation before trusting a changed key', async () => {
    const { trusted } = renderHost({
      state: 'hostKeyMismatch',
      key: PRESENTED,
      fingerprint: 'SHA256:presented',
      expectedFingerprint: 'SHA256:expected',
    });
    const user = userEvent.setup();
    expect(await screen.findByText("The server's key changed")).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Trust the new key' }));
    const confirmation = await screen.findByRole('alertdialog');
    expect(trusted).toEqual([]);

    await user.click(within(confirmation).getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(trusted).toEqual([]);

    await user.click(screen.getByRole('button', { name: 'Trust the new key' }));
    await user.click(
      within(await screen.findByRole('alertdialog')).getByRole('button', { name: 'Trust new key' }),
    );
    await waitFor(() => expect(trusted).toEqual([{ id: 'prod-id', key: PRESENTED }]));
  });
});

describe('SSH setup card', () => {
  it('shows the commands that authorize the generated key, ready to copy', async () => {
    // userEvent stands in for the clipboard.
    const user = userEvent.setup();
    renderHost({ state: 'connecting' });
    expect(await screen.findByText('Authorize Homeport on the server')).toBeInTheDocument();
    expect(
      screen.getByText("echo 'ssh-ed25519 AAAA' >> ~/.ssh/authorized_keys"),
    ).toBeInTheDocument();
    expect(screen.getByText('sudo usermod -aG docker deploy')).toBeInTheDocument();
    expect(screen.getByText('SHA256:homeport')).toBeInTheDocument();
    await user.click(screen.getAllByRole('button', { name: 'Copy' })[0]!);
    expect(await navigator.clipboard.readText()).toBe(
      "echo 'ssh-ed25519 AAAA' >> ~/.ssh/authorized_keys",
    );
  });
});

describe('remove host card', () => {
  it('removes the Host only once confirmed', async () => {
    const { removed } = renderHost({ state: 'connecting' });
    const user = userEvent.setup();
    await user.click(await screen.findByRole('button', { name: 'Remove host' }));
    const confirmation = await screen.findByRole('alertdialog');
    expect(within(confirmation).getByText('Remove prod?')).toBeInTheDocument();
    expect(removed).toEqual([]);

    await user.click(within(confirmation).getByRole('button', { name: 'Cancel' }));
    await waitFor(() => expect(screen.queryByRole('alertdialog')).toBeNull());
    expect(removed).toEqual([]);

    await user.click(screen.getByRole('button', { name: 'Remove host' }));
    await user.click(
      within(await screen.findByRole('alertdialog')).getByRole('button', { name: 'Remove host' }),
    );
    await waitFor(() => expect(removed).toEqual(['prod-id']));
  });
});
