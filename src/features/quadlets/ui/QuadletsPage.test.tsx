import { screen, waitFor, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Container, QuadletUnits } from '@/shared/api';
import { baseCommands, connectedHost, container, podmanHost } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

// CodeMirror doesn't run in jsdom: the editor is a text area.
vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  UnitFileEditor: ({
    value,
    label,
    readOnly = false,
    onChange,
  }: {
    value: string;
    label: string;
    readOnly?: boolean;
    onChange?: (value: string) => void;
  }) => (
    <textarea
      aria-label={label}
      value={value}
      readOnly={readOnly}
      onChange={(e) => onChange?.(e.target.value)}
    />
  ),
}));

const listed = (patch: Partial<QuadletUnits> = {}): QuadletUnits => ({
  units: [
    {
      name: 'web.container',
      unitName: 'web.service',
      path: '/home/me/.config/containers/systemd/shop/web.container',
      status: 'active/running',
      app: 'shop',
      pod: null,
      scope: 'user',
      containers: [{ id: 'c1-0123456789abcdef', name: 'systemd-web', state: 'running' }],
    },
    {
      name: 'cache.volume',
      unitName: 'cache-volume.service',
      path: '/home/me/.config/containers/systemd/cache.volume',
      status: 'inactive/dead',
      app: null,
      pod: null,
      scope: 'user',
      containers: [],
    },
  ],
  source: 'api',
  podmanVersion: '5.8.1',
  managesFiles: true,
  runsSystemctl: true,
  ...patch,
});

type Call = { cmd: string; args: Record<string, unknown> };

function setup(calls: Call[] = [], units = listed(), host = podmanHost) {
  const record = (cmd: string, result: unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result;
  };
  const web: Container = {
    ...container('systemd-web', null),
    labels: { PODMAN_SYSTEMD_UNIT: 'web.service' },
  };
  mockCommands({
    ...baseCommands,
    hosts_list: () => [host],
    quadlets_list: () => units,
    quadlets_file: () => '[Container]\nImage=nginx\n',
    quadlets_act: record('quadlets_act', null),
    quadlets_install: record('quadlets_install', ['/home/me/x']),
    quadlets_remove: record('quadlets_remove', null),
    pods_list: () => [],
    containers_list: () => [web, container('redis', null)],
    usage_subscribe: () => 1,
    images_list: () => [],
    networks_list: () => [],
  });
  return userEvent.setup();
}

const nav = () => screen.getByRole('navigation', { name: 'Main' });

describe('Quadlet units', () => {
  it('aren’t offered on a Docker Engine', async () => {
    setup([], listed(), connectedHost);
    renderApp('/quadlets');
    expect(await screen.findByText('local can’t show this page')).toBeInTheDocument();
    expect(within(nav()).queryByRole('link', { name: 'Quadlet' })).toBeNull();
  });

  it('are listed on a Podman Host, and stop from their row', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/quadlets');
    expect(await screen.findByText('web.container')).toBeInTheDocument();
    expect(within(nav()).getByRole('link', { name: 'Quadlet' })).toBeInTheDocument();
    const row = screen.getByRole('row', { name: /web\.container/ });
    expect(within(row).getByText('Running')).toBeInTheDocument();
    await user.click(within(row).getByRole('button', { name: 'Stop web.service' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'quadlets_act',
        args: { hostId: 'h1', unit: 'web.service', scope: 'user', action: 'stop' },
      }),
    );
  });

  it('can’t be started where Homeport can’t run systemctl, and say why', async () => {
    setup([], listed({ runsSystemctl: false }));
    renderApp('/quadlets');
    const row = (await screen.findByText('cache.volume')).closest('tr')!;
    expect(within(row).getByRole('button', { name: 'Start cache-volume.service' })).toBeDisabled();
    expect(screen.getByText(/needs this host to be reached over SSH/)).toBeInTheDocument();
  });

  it('come from their containers on older Podman, which can’t install any', async () => {
    setup([], listed({ source: 'labels', podmanVersion: '5.6.2', managesFiles: false }));
    renderApp('/quadlets');
    expect(await screen.findByText(/Podman 5.6.2 can’t list Quadlet units/)).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'New unit' })).toBeDisabled();
  });

  it('are installed from a typed file', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/quadlets');
    await user.click(await screen.findByRole('button', { name: 'New unit' }));
    const dialog = await screen.findByRole('dialog', { name: 'New Quadlet unit' });
    const name = within(dialog).getByLabelText('File name');
    await user.clear(name);
    await user.type(name, 'api.container');
    await user.click(within(dialog).getByRole('button', { name: 'Install' }));
    await waitFor(() => expect(calls.map((c) => c.cmd)).toContain('quadlets_install'));
    const install = calls.find((c) => c.cmd === 'quadlets_install')!.args;
    expect(install.replace).toBe(false);
    expect(install.install).toMatchObject({
      unit: { name: 'api.container' },
      extra: [],
      app: null,
    });
    expect((install.install as { unit: { content: string } }).unit.content).toContain(
      '[Container]',
    );
  });

  it('show their file and containers, and are edited in place', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/quadlets/web.container');
    const file = await screen.findByRole('textbox', { name: 'Unit file' });
    await waitFor(() => expect(file).toHaveValue('[Container]\nImage=nginx\n'));
    expect(screen.getByRole('link', { name: /systemd-web/ })).toHaveAttribute(
      'href',
      '/containers/c1-0123456789abcdef/logs',
    );
    expect(screen.getByText('A user unit (systemctl --user)')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'Edit' }));
    const dialog = await screen.findByRole('dialog', { name: 'Edit web.container' });
    expect(within(dialog).getByLabelText('File name')).toBeDisabled();
    await user.click(within(dialog).getByRole('button', { name: 'Save' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'quadlets_install',
        args: {
          hostId: 'h1',
          install: {
            unit: { name: 'web.container', content: '[Container]\nImage=nginx\n' },
            extra: [],
            app: 'shop',
          },
          replace: true,
        },
      }),
    );
  });

  it('are removed, stopping their service when asked', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/quadlets/web.container');
    await user.click(await screen.findByRole('button', { name: 'Remove' }));
    const dialog = await screen.findByRole('alertdialog');
    expect(
      within(dialog).getByRole('switch', { name: 'Stop its running service first' }),
    ).toBeChecked();
    await user.click(within(dialog).getByRole('button', { name: 'Remove' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'quadlets_remove',
        args: { hostId: 'h1', name: 'web.container', force: true },
      }),
    );
  });

  it('are named on the containers they created', async () => {
    setup();
    renderApp('/containers');
    const row = (await screen.findByText('systemd-web')).closest('tr')!;
    expect(within(row).getByRole('link', { name: 'web.service' })).toHaveAttribute(
      'href',
      '/quadlets/web.service',
    );
  });
});
