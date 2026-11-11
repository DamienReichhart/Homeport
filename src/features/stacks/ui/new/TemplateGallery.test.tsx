import { screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp } from '@/test/render';

function mock() {
  mockCommands({
    ...baseCommands,
    stacks_list: () => [],
    stacks_check: () => ({ errors: [], warnings: [], profiles: [] }),
  });
}

describe('starting a Stack from a template', () => {
  it('fills the name, compose file and environment from the picked card', async () => {
    mock();
    renderApp('/stacks/new');
    const card = await screen.findByRole('button', { name: /PostgreSQL and Adminer/ });
    expect(card).toHaveAccessibleDescription(
      'A PostgreSQL database with Adminer to browse it in a web page.',
    );
    await userEvent.click(card);
    expect(screen.getByLabelText('Name')).toHaveValue('postgres');
    expect((screen.getByLabelText('Environment') as HTMLTextAreaElement).value).toContain(
      'POSTGRES_PASSWORD=',
    );
  });

  it('is usable with the keyboard alone', async () => {
    mock();
    renderApp('/stacks/new');
    await screen.findByRole('button', { name: /Static site/ });
    // The source choice comes first, then the cards.
    await userEvent.tab();
    while (!(document.activeElement?.textContent ?? '').includes('Static site'))
      await userEvent.tab();
    await userEvent.keyboard('{Enter}');
    expect(screen.getByLabelText('Name')).toHaveValue('site');
  });

  it('takes over an External Stack only when adopting it', async () => {
    const created: unknown[] = [];
    const record = (args: unknown) => {
      created.push(args);
      throw { code: 'name_in_use', key: 'errors.stacks.nameRunsExternal', message: 'refused' };
    };
    mockCommands({
      ...baseCommands,
      stacks_list: () => [],
      stacks_check: () => ({ errors: [], warnings: [], profiles: [] }),
      stacks_create: record,
    });
    const page = renderApp('/stacks/new');
    await userEvent.click(await screen.findByRole('button', { name: /Static site/ }));
    await userEvent.click(screen.getByRole('button', { name: 'Save' }));
    await screen.findByText(/refused|already runs/);
    expect(created).toEqual([expect.objectContaining({ name: 'site', adopt: false })]);
    page.unmount();

    renderApp('/stacks/new?adopt=blog');
    await screen.findByLabelText('Name');
    await userEvent.click(screen.getByRole('button', { name: 'Save' }));
    await vi.waitFor(() =>
      expect(created).toEqual([
        expect.objectContaining({ adopt: false }),
        expect.objectContaining({ name: 'blog', adopt: true }),
      ]),
    );
  });

  it('opens an empty editor when adopting', async () => {
    mock();
    renderApp('/stacks/new?adopt=blog');
    expect(await screen.findByLabelText('Name')).toHaveValue('blog');
    expect(screen.queryByRole('button', { name: /Static site/ })).toBeNull();
  });
});
