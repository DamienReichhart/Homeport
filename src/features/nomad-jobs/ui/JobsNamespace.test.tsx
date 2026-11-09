import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { nomadCommands, nomadJob } from '@/test/nomad';
import { renderApp } from '@/test/render';

function setup(namespaces: unknown, remembered: Record<string, string> = {}) {
  const asked: unknown[] = [];
  const saved: unknown[] = [];
  mockCommands({
    ...baseCommands,
    ...nomadCommands,
    get_settings: () => ({ ...nomadCommands.get_settings(), clusterNamespaces: remembered }),
    update_settings: ({ settings }) => {
      saved.push((settings as { clusterNamespaces: unknown }).clusterNamespaces);
      return settings;
    },
    clusters_nomad_namespaces: () => namespaces,
    clusters_nomad_jobs: ({ namespace }) => {
      asked.push(namespace);
      return [nomadJob('web', { namespace: 'shop' })];
    },
  });
  return { asked, saved };
}

const listable = {
  listable: true,
  namespaces: [
    { name: 'default', description: null },
    { name: 'shop', description: null },
  ],
};

describe('the namespace of the Jobs page', () => {
  it('starts at default, and remembers every namespace when picked, showing each job’s', async () => {
    const { asked, saved } = setup(listable);
    const user = userEvent.setup();
    renderApp('/nomad/jobs');
    const picker = await screen.findByRole('combobox', { name: 'Namespace' });
    expect(picker).toHaveTextContent('default');
    await waitFor(() => expect(asked).toEqual(['default']));
    expect(screen.queryByRole('columnheader', { name: 'Namespace' })).not.toBeInTheDocument();

    await user.click(picker);
    await user.click(await screen.findByRole('option', { name: 'All namespaces' }));
    await waitFor(() => expect(asked).toContain('*'));
    expect(await screen.findByRole('columnheader', { name: 'Namespace' })).toBeInTheDocument();
    await waitFor(() => expect(saved).toContainEqual({ n1: '' }));
  });

  it('keeps the namespace remembered for the cluster', async () => {
    const { asked } = setup(listable, { n1: 'shop' });
    renderApp('/nomad/jobs');
    expect(await screen.findByRole('combobox', { name: 'Namespace' })).toHaveTextContent('shop');
    await waitFor(() => expect(asked).toContain('shop'));
  });

  it('shows the token’s default namespace, with no picker, when it can’t list them', async () => {
    const { asked } = setup({ listable: false, namespaces: [] }, { n1: 'shop' });
    renderApp('/nomad/jobs');
    expect(await screen.findByText('web')).toBeInTheDocument();
    expect(screen.queryByRole('combobox', { name: 'Namespace' })).not.toBeInTheDocument();
    expect(asked.at(-1)).toBeNull();
  });
});
