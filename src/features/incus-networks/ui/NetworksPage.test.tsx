import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { IncusNetwork } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { incusHost, incusProject } from '@/test/incus';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

type Call = { cmd: string; args: Record<string, unknown> };

const network = (name: string, over: Partial<IncusNetwork> = {}): IncusNetwork => ({
  name,
  type: 'bridge',
  managed: true,
  description: '',
  status: 'Created',
  ipv4: '10.42.71.1/24',
  ipv6: null,
  nat: true,
  config: {},
  usedBy: [],
  ...over,
});

function setup(calls: Call[] = []) {
  const record = (cmd: string) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return null;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [incusHost],
    incus_projects: () => [incusProject('default')],
    incus_networks: () => [
      network('incusbr0', {
        usedBy: [
          { kind: 'profile', name: 'default', project: null },
          { kind: 'instance', name: 't1', project: null },
        ],
      }),
      network('lab0', { ipv4: '10.0.5.1/24' }),
      network('eth0', { type: 'physical', managed: false, ipv4: null, nat: false }),
    ],
    incus_network_state: () => ({
      state: 'up',
      hwaddr: '00:16:3e:bb:e1:6e',
      mtu: 1500,
      addresses: ['10.42.71.1/24'],
      leases: [
        { hostname: 'incusbr0.gw', hwaddr: '', address: '10.42.71.1', type: 'gateway' },
        { hostname: 't1', hwaddr: '00:16:3e:97:58:32', address: '10.42.71.245', type: 'dynamic' },
      ],
    }),
    incus_bridge_create: record('create'),
    incus_network_delete: record('delete'),
  });
  return userEvent.setup();
}

describe('incus networks', () => {
  it('list networks, refusing to delete those in use or the machine’s own', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/networks');
    expect(await screen.findByText('lab0')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Delete incusbr0' })).toBeDisabled();
    expect(screen.queryByRole('button', { name: 'Delete eth0' })).toBeNull();
    await user.click(screen.getByRole('button', { name: 'Delete lab0' }));
    await user.click(await screen.findByRole('button', { name: 'Delete' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'delete',
        args: { hostId: 'i1', project: 'default', name: 'lab0' },
      }),
    );
  });

  it('make a bridge, checking its addresses first', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/networks');
    await user.click(await screen.findByRole('button', { name: 'New bridge' }));
    const dialog = await screen.findByRole('dialog');
    const create = within(dialog).getByRole('button', { name: 'Create' });
    await user.type(within(dialog).getByLabelText('Name'), 'lab1');
    const ipv4 = within(dialog).getByLabelText('IPv4');
    await user.clear(ipv4);
    await user.type(ipv4, '10.0.6.1');
    expect(create).toBeDisabled();
    await user.type(ipv4, '/24');
    await user.click(create);
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'create',
        args: {
          hostId: 'i1',
          project: 'default',
          request: { name: 'lab1', ipv4: '10.0.6.1/24', ipv6: 'auto', nat: true, description: '' },
        },
      }),
    );
  });

  it('show a network’s users and DHCP leases', async () => {
    setup();
    renderApp('/incus/networks/incusbr0');
    expect(await screen.findByRole('link', { name: 't1' })).toHaveAttribute(
      'href',
      '/incus/instances/t1',
    );
    expect(screen.getByText('profile default')).toBeInTheDocument();
    expect(await screen.findByText('10.42.71.245')).toBeInTheDocument();
    expect(screen.getByText('gateway')).toBeInTheDocument();
  });
});
