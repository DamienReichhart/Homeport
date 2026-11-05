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

  it('opens an empty editor when adopting', async () => {
    mock();
    renderApp('/stacks/new?adopt=blog');
    expect(await screen.findByLabelText('Name')).toHaveValue('blog');
    expect(screen.queryByRole('button', { name: /Static site/ })).toBeNull();
  });
});
