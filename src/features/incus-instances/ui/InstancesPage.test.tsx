import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { incusHost, incusInstance, incusProject } from '@/test/incus';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const web = incusInstance('web', { snapshots: 2 });
const db = incusInstance('db', { status: 'stopped', memoryBytes: null, cpuUsageNs: null });
const vm = incusInstance('vm1', { type: 'virtual-machine', status: 'frozen' });

type Call = { cmd: string; args: Record<string, unknown> };

function setup(calls: Call[] = []) {
  const record = (cmd: string, result: unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result;
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [incusHost],
    incus_projects: () => [incusProject('default', 3), incusProject('lab')],
    incus_instances: () => [web, db, vm],
    incus_instance: ({ name }) => ({
      instance: [web, db, vm].find((i) => i.name === name),
      config: { 'limits.cpu': '2' },
      expandedConfig: { 'limits.cpu': '2', 'security.nesting': 'true' },
      devices: {},
      expandedDevices: { root: { type: 'disk', path: '/', pool: 'default' } },
      snapshotNames: ['before-upgrade', 'snap1'],
      statefulSnapshots: false,
      raw: { name },
    }),
    incus_instance_act: record('incus_instance_act', null),
    incus_instance_delete: record('incus_instance_delete', null),
  });
  return userEvent.setup();
}

const nav = () => screen.getByRole('navigation', { name: 'Main' });

describe('incus instances', () => {
  it('list the project’s instances, filtered by type, without Docker pages', async () => {
    const user = setup();
    renderApp('/incus/instances');
    expect(await screen.findByText('web')).toBeInTheDocument();
    expect(screen.getByText(/3 instances, 1 running, in the project default/)).toBeInTheDocument();
    await user.click(screen.getByRole('radio', { name: 'Virtual machine' }));
    expect(screen.queryByText('web')).toBeNull();
    expect(screen.getByText('vm1')).toBeInTheDocument();
    expect(within(nav()).getByRole('link', { name: 'Instances' })).toBeInTheDocument();
    expect(within(nav()).queryByRole('link', { name: 'Containers' })).toBeNull();
    expect(within(nav()).queryByRole('link', { name: 'Volumes' })).toBeNull();
  });

  it('start, stop and force stop from the list', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/instances');
    await user.click(await screen.findByRole('button', { name: 'Stop web' }));
    await user.click(screen.getByRole('button', { name: 'Start db' }));
    await user.click(screen.getByRole('button', { name: 'More actions for web' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Force stop' }));
    await waitFor(() =>
      expect(calls.filter((c) => c.cmd === 'incus_instance_act').map((c) => c.args)).toEqual([
        { hostId: 'i1', project: 'default', name: 'web', action: 'stop', force: false },
        { hostId: 'i1', project: 'default', name: 'db', action: 'start', force: false },
        { hostId: 'i1', project: 'default', name: 'web', action: 'stop', force: true },
      ]),
    );
  });

  it('delete a running instance only once the user agrees to stop it first', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/instances/web');
    expect(await screen.findByText('before-upgrade, snap1')).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'More actions for web' }));
    await user.click(await screen.findByRole('menuitem', { name: 'Delete' }));
    const dialog = await screen.findByRole('dialog');
    expect(within(dialog).getByText(/before-upgrade, snap1 are deleted/)).toBeInTheDocument();
    const confirm = within(dialog).getByRole('button', { name: 'Stop and delete' });
    expect(confirm).toBeDisabled();
    await user.click(within(dialog).getByRole('checkbox'));
    await user.click(confirm);
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'incus_instance_delete',
        args: { hostId: 'i1', project: 'default', name: 'web', stopFirst: true },
      }),
    );
  });

  it('show an instance’s configuration, marking what its profiles set', async () => {
    const user = setup();
    renderApp('/incus/instances/web');
    await user.click(await screen.findByRole('tab', { name: 'Configuration' }));
    expect(await screen.findByText('security.nesting')).toBeInTheDocument();
    expect(screen.getAllByText('From a profile')).toHaveLength(1);
    await user.click(screen.getByRole('tab', { name: 'Devices' }));
    expect(await screen.findByText('root')).toBeInTheDocument();
  });
});
