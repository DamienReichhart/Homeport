import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, connectedHost, container } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const accessRefused = {
  code: 'permission_denied',
  message: 'Homeport isn’t allowed to use the Docker Engine at /var/run/docker.sock.',
  hint: {
    text: 'Add your user to the docker group, then log out and back in.',
    commands: ['sudo usermod -aG docker "$USER"'],
  },
  detail: 'Permission denied (os error 13)',
};

const appInfo = () => ({ name: 'Homeport', version: '0.3.0', os: 'linux', arch: 'x86_64' });

describe('error handling', () => {
  it('explains a list that failed to load, shows its details and retries', async () => {
    let calls = 0;
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      usage_subscribe: () => 1,
      containers_list: () => {
        calls += 1;
        // Queries retry once before giving up.
        if (calls <= 2) throw accessRefused;
        return [container('redis', null)];
      },
    });
    const user = userEvent.setup();
    renderApp('/containers');

    const alert = await screen.findByRole('alert', {}, { timeout: 4000 });
    expect(within(alert).getByText(accessRefused.message)).toBeInTheDocument();
    expect(within(alert).getByText(accessRefused.hint.text)).toBeInTheDocument();
    expect(within(alert).getByText('sudo usermod -aG docker "$USER"')).toBeInTheDocument();

    await user.click(within(alert).getByRole('button', { name: 'Details' }));
    const details = await screen.findByRole('dialog');
    expect(within(details).getByText('permission_denied')).toBeInTheDocument();
    expect(within(details).getByText('Permission denied (os error 13)')).toBeInTheDocument();
    expect(within(details).getByText('containersList')).toBeInTheDocument();
    expect(await within(details).findByText(/Homeport 0\.3\.0/)).toBeInTheDocument();

    await user.click(within(details).getByRole('button', { name: 'Copy report' }));
    const report = await navigator.clipboard.readText();
    expect(report).toContain(accessRefused.message);
    expect(report).toContain('Code: permission_denied');
    expect(report).toContain('Permission denied (os error 13)');
    expect(report).toContain('sudo usermod -aG docker "$USER"');

    await user.keyboard('{Escape}');
    await user.click(within(alert).getByRole('button', { name: 'Retry' }));
    expect(await screen.findByText('redis')).toBeInTheDocument();
  });

  it('turns a failure nobody handles into a toast with details', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      hosts_list: () => [
        {
          ...connectedHost,
          status: {
            state: 'unreachable',
            retryInSecs: 30,
            error: {
              code: 'connection_refused',
              message: 'prod.example:2375 refused the connection.',
              hint: null,
              detail: 'Connection refused (os error 111)',
            },
          },
        },
      ],
      hosts_reconnect: () => {
        throw {
          code: 'db_locked',
          message: 'Homeport’s database is busy.',
          hint: { text: 'Another Homeport may be running.', commands: [] },
          detail: 'database is locked',
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/containers');

    await user.click(await screen.findByRole('button', { name: /Retry now/ }));
    const message = await screen.findByText('Homeport’s database is busy.');
    const toast = message.closest<HTMLElement>('[data-sonner-toast]')!;
    expect(within(toast).getByText('Another Homeport may be running.')).toBeInTheDocument();

    await user.click(within(toast).getByRole('button', { name: 'Details' }));
    const details = await screen.findByRole('dialog');
    expect(within(details).getByText('db_locked')).toBeInTheDocument();
    expect(within(details).getByText('hostsReconnect')).toBeInTheDocument();
  });

  it('says why a host is down and what to do about it', async () => {
    mockCommands({
      ...baseCommands,
      get_app_info: appInfo,
      hosts_list: () => [
        { ...connectedHost, status: { state: 'unauthorized', error: accessRefused } },
      ],
    });
    const user = userEvent.setup();
    renderApp('/containers');

    expect(await screen.findByText('local: Access refused')).toBeInTheDocument();
    expect(screen.getByText(accessRefused.message)).toBeInTheDocument();
    expect(screen.getByText(accessRefused.hint.text)).toBeInTheDocument();
    expect(screen.getByText('sudo usermod -aG docker "$USER"')).toBeInTheDocument();

    await user.click(screen.getByRole('button', { name: 'Details' }));
    const details = await screen.findByRole('dialog');
    expect(within(details).getByText('Permission denied (os error 13)')).toBeInTheDocument();
  });
});
