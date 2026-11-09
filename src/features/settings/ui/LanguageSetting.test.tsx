import { screen, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import type { Settings } from '@/shared/api';
import { DEFAULT_LOCALE, setLocale } from '@/shared/i18n';
import { AppError } from '@/shared/api';
import { ErrorMessage } from '@/shared/ui';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { renderApp, renderWithProviders } from '@/test/render';

describe('choosing the app’s language', () => {
  afterEach(() => setLocale(DEFAULT_LOCALE));

  it('switches the whole window at once, saves it and hands it to the tray', async () => {
    let settings = baseCommands.get_settings() as Settings;
    const native: Record<string, string>[] = [];
    mockCommands({
      ...baseCommands,
      get_settings: () => settings,
      update_settings: ({ settings: next }: Record<string, unknown>) => {
        settings = next as Settings;
        return settings;
      },
      set_native_text: ({ templates }: Record<string, unknown>) => {
        native.push(templates as Record<string, string>);
        return null;
      },
    });
    const user = userEvent.setup();
    renderApp('/settings');
    expect(await screen.findByRole('heading', { name: 'Settings' })).toBeInTheDocument();
    await user.click(screen.getByRole('combobox', { name: 'Language' }));
    await user.click(await screen.findByRole('option', { name: 'Français' }));

    expect(await screen.findByRole('heading', { name: 'Paramètres' })).toBeInTheDocument();
    expect(screen.getByRole('combobox', { name: 'Langue' })).toBeInTheDocument();
    expect(document.documentElement.lang).toBe('fr');
    expect(settings.language).toBe('fr');
    await waitFor(() => expect(native.at(-1)?.['native.tray.quit']).toBe('Quitter'));
  });

  it('re-renders an error already on screen in the new language', async () => {
    const error = new AppError({
      code: 'unknown',
      key: 'errors.hosts.notConnected',
      params: { name: 'prod' },
      message: 'prod isn’t connected.',
    });
    mockCommands(baseCommands);
    renderWithProviders(<ErrorMessage error={error} />);
    expect(await screen.findByText(/prod isn.t connected/)).toBeInTheDocument();
    await setLocale('fr');
    await waitFor(() => expect(screen.queryByText(/isn.t connected/)).not.toBeInTheDocument());
    expect(screen.getByText(/prod/)).toBeInTheDocument();
    expect(error.english).toBe('prod isn’t connected.');
  });
});
