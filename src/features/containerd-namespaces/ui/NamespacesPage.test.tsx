import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { ContainerdNamespace } from '@/shared/api';
import { baseCommands, connectedHost } from '@/test/fixtures';
import { containerdHost } from '@/test/containerd';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const ns = (name: string, containers = 0, images = 0): ContainerdNamespace => ({
  name,
  labels: {},
  containers,
  images,
});

function setup(
  calls: { cmd: string; args: Record<string, unknown> }[] = [],
  host = containerdHost,
) {
  const saved: Record<string, unknown>[] = [];
  const record = (cmd: string, result: unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [host],
    containerd_namespaces: () => [ns('default'), ns('k8s.io', 12, 30), ns('lab')],
    containerd_namespace_create: record('containerd_namespace_create', null),
    containerd_namespace_remove: record('containerd_namespace_remove', null),
    update_settings: ({ settings }) => {
      saved.push(settings as Record<string, unknown>);
      return settings;
    },
  });
  return { user: userEvent.setup(), saved };
}

const nav = () => screen.getByRole('navigation', { name: 'Main' });

describe('containerd namespaces', () => {
  it('aren’t offered on a Docker Engine', async () => {
    setup([], connectedHost);
    renderApp('/containerd/namespaces');
    expect(await screen.findByText('local can’t show this page')).toBeInTheDocument();
    expect(within(nav()).queryByRole('link', { name: 'Namespaces' })).toBeNull();
    expect(screen.queryByRole('combobox', { name: 'Namespace' })).toBeNull();
  });

  it('show k8s.io first when only it holds something, and remember a pick', async () => {
    const { user, saved } = setup();
    renderApp('/containerd/namespaces');
    const picker = await screen.findByRole('combobox', { name: 'Namespace' });
    await waitFor(() => expect(picker).toHaveTextContent('k8s.io'));
    expect(within(nav()).getByRole('link', { name: 'Namespaces' })).toBeInTheDocument();
    const row = (await screen.findByText('lab')).closest('tr')!;
    await user.click(within(row).getByRole('button', { name: 'Show' }));
    await waitFor(() => expect(saved.at(-1)?.containerdNamespaces).toEqual({ c1: 'lab' }));
    expect(picker).toHaveTextContent('lab');
  });

  it('remove only empty namespaces, and create new ones', async () => {
    const calls: { cmd: string; args: Record<string, unknown> }[] = [];
    const { user } = setup(calls);
    renderApp('/containerd/namespaces');
    expect(await screen.findByRole('button', { name: 'Remove k8s.io' })).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Remove lab' }));
    await user.click(await screen.findByRole('button', { name: 'Remove' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'containerd_namespace_remove',
        args: { hostId: 'c1', name: 'lab' },
      }),
    );

    await user.click(screen.getByRole('button', { name: 'New namespace' }));
    const name = await screen.findByLabelText('Name');
    await user.type(name, 'bad name');
    expect(screen.getByRole('button', { name: 'Create' })).toBeDisabled();
    await user.clear(name);
    await user.type(name, 'ci');
    await user.click(screen.getByRole('button', { name: 'Create' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'containerd_namespace_create',
        args: { hostId: 'c1', name: 'ci' },
      }),
    );
  });
});
