import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { mockCommands } from '@/test/ipc';
import { App } from './App';

const newerDatabase = {
  error: {
    code: 'db_from_newer_version',
    key: 'errors.storage.dbFromNewer',
    params: { version: '1.2.0' },
    message: 'This database was last used by Homeport 1.2.0 or later.',
    hint: {
      key: 'errors.storage.dbFromNewer.hint',
      params: { version: '1.2.0' },
      text: 'Install Homeport 1.2.0 or newer from the download page. Your data hasn’t been changed.',
      commands: [],
    },
    detail: 'the database is at schema version 21; Homeport 1.0.0 knows up to 18',
  },
  databasePath: '/home/me/.local/share/com.homeport.app/homeport.db',
};

describe('a database that can’t be opened', () => {
  it('is shown instead of the app, with where the file is', async () => {
    const opened = vi.fn(() => null);
    // Any other command would throw: nothing else may start.
    mockCommands({ startup_problem: () => newerDatabase, open_data_folder: opened });
    render(<App />);

    expect(
      await screen.findByText('This database was last used by Homeport 1.2.0 or later.'),
    ).toBeInTheDocument();
    expect(screen.getByText(/Install Homeport 1\.2\.0 or newer/)).toBeInTheDocument();
    expect(screen.getByText(newerDatabase.databasePath)).toBeInTheDocument();
    expect(screen.queryByRole('navigation')).not.toBeInTheDocument();

    await userEvent.click(screen.getByRole('button', { name: 'Open folder' }));
    expect(opened).toHaveBeenCalled();
  });
});
