import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands, connectedHost, hostView } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { clientWithoutRetries, renderApp } from '@/test/render';

describe('hosts page', () => {
  it('lists every Host with its kind, address and status', async () => {
    mockCommands({
      ...baseCommands,
      hosts_list: () => [
        connectedHost,
        hostView('prod', {
          status: {
            state: 'unreachable',
            error: {
              code: 'timeout',
              key: '',
              params: {},
              message: 'No answer.',
              hint: null,
              detail: null,
            },
            retryInSecs: 4,
          },
        }),
        hostView('edge', {
          connection: { kind: 'tls', address: 'edge.example.com:2376' },
          status: { state: 'credentialsLost' },
        }),
      ],
    });
    renderApp('/hosts');
    expect(await screen.findByText('prod')).toBeInTheDocument();
    expect(screen.getByText('deploy@prod.example.com')).toBeInTheDocument();
    expect(screen.getByText('edge.example.com:2376')).toBeInTheDocument();
    expect(screen.getAllByText('Connected').length).toBeGreaterThan(0);
    expect(screen.getByText('Unreachable')).toBeInTheDocument();
    expect(screen.getByText('Credentials lost')).toBeInTheDocument();
  });

  it('shows a list that fails to load in place', async () => {
    mockCommands({
      ...baseCommands,
      hosts_list: () => {
        throw { code: 'internal', message: 'The database is locked.', hint: null, detail: null };
      },
    });
    renderApp('/hosts', clientWithoutRetries());
    expect((await screen.findAllByText('The database is locked.')).length).toBeGreaterThan(0);
  });
});

describe('add host page', () => {
  it('adds an Engine found on this machine in one click', async () => {
    const created: unknown[] = [];
    mockCommands({
      ...baseCommands,
      hosts_list: () => [connectedHost],
      hosts_detect_local: () => [
        {
          label: 'Docker Engine',
          socketPath: '/var/run/docker.sock',
          engine: 'docker',
          engineVersion: '29.1',
          problem: null,
        },
        {
          label: 'Podman',
          socketPath: '/run/user/1000/podman/podman.sock',
          engine: 'docker',
          engineVersion: '5.4',
          problem: null,
        },
      ],
      hosts_create: ({ input }) => {
        created.push(input);
        return hostView('Podman', {
          connection: { kind: 'local', socketPath: '/run/user/1000/podman/podman.sock' },
        });
      },
    });
    const user = userEvent.setup();
    renderApp('/hosts/new');
    expect(await screen.findByText('Found on this machine')).toBeInTheDocument();
    // The socket that is already a Host isn't offered again.
    expect(screen.queryByText('/var/run/docker.sock')).toBeNull();
    await user.click(screen.getByRole('button', { name: 'Add' }));
    await waitFor(() =>
      expect(created).toEqual([
        {
          name: 'Podman',
          connection: { kind: 'local', socketPath: '/run/user/1000/podman/podman.sock' },
          engine: 'docker',
          credentials: null,
        },
      ]),
    );
  });

  it('shows why a Host can’t be created in place', async () => {
    mockCommands({
      ...baseCommands,
      hosts_detect_local: () => [],
      hosts_create: () => {
        throw {
          code: 'validation',
          message: 'A host named "prod" already exists.',
          hint: { text: 'Pick another name.', commands: [] },
          detail: null,
        };
      },
    });
    const user = userEvent.setup();
    renderApp('/hosts/new');
    await user.click(await screen.findByRole('radio', { name: 'TCP' }));
    await user.click(screen.getByRole('button', { name: 'Add host' }));
    expect(await screen.findByText('A host named "prod" already exists.')).toBeInTheDocument();
    expect(screen.getByText('Pick another name.')).toBeInTheDocument();
  });
});
