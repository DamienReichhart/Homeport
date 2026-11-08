import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { clusterCommands } from '@/test/clusters';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

const bitnami = { name: 'bitnami', url: 'https://charts.bitnami.com/bitnami' };

describe('helm chart repositories', () => {
  it('checks the Add form before helm sees it, then adds the repository', async () => {
    const repos: { name: string; url: string }[] = [];
    const added: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      helm_repos_list: () => [...repos],
      helm_repos_add: (args) => {
        added.push(args);
        repos.push(args as { name: string; url: string });
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/helm/repositories');
    expect(await screen.findByText(/helm has no chart repository yet/)).toBeInTheDocument();
    await user.click(screen.getByRole('button', { name: 'Add repository' }));
    await user.type(screen.getByLabelText('Name'), 'bitnami');
    await user.type(screen.getByLabelText('URL'), 'oci://registry-1.docker.io/bitnamicharts');
    await user.click(screen.getAllByRole('button', { name: 'Add repository' }).at(-1)!);
    expect(screen.getByRole('alert')).toHaveTextContent(/install its chart with its oci:\/\//);
    expect(added).toEqual([]);

    await user.clear(screen.getByLabelText('URL'));
    await user.type(screen.getByLabelText('URL'), bitnami.url);
    await user.click(screen.getAllByRole('button', { name: 'Add repository' }).at(-1)!);
    await waitFor(() => expect(added).toEqual([bitnami]));
    expect(await screen.findByText(bitnami.url)).toBeInTheDocument();
  });

  it('removes a repository once confirmed', async () => {
    const removed: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      helm_repos_list: () => [bitnami],
      helm_repos_remove: ({ name }) => {
        removed.push(name);
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/helm/repositories');
    await user.click(await screen.findByRole('button', { name: 'Remove bitnami' }));
    expect(
      screen.getByText(/forgets the repository https:\/\/charts.bitnami.com/),
    ).toBeInTheDocument();
    expect(removed).toEqual([]);
    await user.click(screen.getByRole('button', { name: 'Remove' }));
    await waitFor(() => expect(removed).toEqual(['bitnami']));
  });

  it('searches the repositories’ charts', async () => {
    const searched: unknown[] = [];
    mockCommands({
      ...baseCommands,
      ...clusterCommands,
      helm_repos_list: () => [bitnami],
      helm_charts_search: (args) => {
        searched.push(args);
        return [
          {
            name: 'bitnami/postgresql',
            version: '16.2.1',
            appVersion: '17.2.0',
            description: 'PostgreSQL (Postgres) is an open source object-relational database',
          },
        ];
      },
    });
    const user = userEvent.setup();
    renderApp('/helm/repositories');
    await user.type(
      await screen.findByPlaceholderText('Search charts, such as postgres'),
      'postgres',
    );
    expect(await screen.findByText('bitnami/postgresql')).toBeInTheDocument();
    expect(screen.getByText('16.2.1')).toBeInTheDocument();
    expect(searched.at(-1)).toEqual({ query: 'postgres', versions: false });
  });
});
