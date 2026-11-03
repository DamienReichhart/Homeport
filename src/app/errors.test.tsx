import { screen, within } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, container } from '@/test/fixtures';
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
});
