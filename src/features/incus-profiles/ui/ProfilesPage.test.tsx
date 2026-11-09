import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { IncusProfile } from '@/shared/api';
import { baseCommands } from '@/test/fixtures';
import { incusHost, incusProject } from '@/test/incus';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const profile = (name: string, over: Partial<IncusProfile> = {}): IncusProfile => ({
  name,
  description: '',
  config: {},
  devices: {},
  usedBy: [],
  ...over,
});

type Call = { cmd: string; args: Record<string, unknown> };

function setup(calls: Call[] = []) {
  const record = (cmd: string, result: () => unknown) => (args: Record<string, unknown>) => {
    calls.push({ cmd, args });
    return result();
  };
  const web = profile('web', {
    description: 'Web servers',
    config: { 'limits.cpu': '2' },
    usedBy: ['nginx'],
  });
  mockCommands({
    ...baseCommands,
    hosts_list: () => [incusHost],
    incus_projects: () => [incusProject('default')],
    incus_profiles: () => [profile('default', { usedBy: ['nginx'] }), web, profile('spare')],
    incus_profile: ({ name }) => ({
      profile: name === 'web' ? web : profile(String(name)),
      yaml: "description: Web servers\nconfig:\n  limits.cpu: '2'\ndevices: {}\n",
      etag: 'abc',
    }),
    incus_profile_delete: record('incus_profile_delete', () => null),
    incus_profile_update: record('incus_profile_update', () => null),
  });
  return userEvent.setup();
}

describe('incus profiles', () => {
  it('keep default and profiles in use, and delete the others', async () => {
    const calls: Call[] = [];
    const user = setup(calls);
    renderApp('/incus/profiles');
    expect(await screen.findByText('Web servers')).toBeInTheDocument();
    expect(screen.getByRole('button', { name: 'Delete default' })).toBeDisabled();
    expect(screen.getByRole('button', { name: 'Delete web' })).toBeDisabled();
    await user.click(screen.getByRole('button', { name: 'Delete spare' }));
    await user.click(await screen.findByRole('button', { name: 'Delete' }));
    await waitFor(() =>
      expect(calls).toContainEqual({
        cmd: 'incus_profile_delete',
        args: { hostId: 'i1', project: 'default', name: 'spare' },
      }),
    );
  });

  it('show who uses a profile, keeping Save off until the YAML changes', async () => {
    setup();
    renderApp('/incus/profiles/web');
    expect(await screen.findByRole('link', { name: 'nginx' })).toHaveAttribute(
      'href',
      '/incus/instances/nginx',
    );
    expect(screen.getByRole('button', { name: 'Save' })).toBeDisabled();
  });
});
