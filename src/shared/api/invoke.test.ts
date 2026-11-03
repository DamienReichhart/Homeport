import { mockCommands } from '@/test/ipc';
import { AppError } from './errors';
import { commands } from './invoke';

describe('commands', () => {
  it('rejects with an AppError that names the failed command', async () => {
    mockCommands({
      get_settings: () => {
        throw {
          code: 'db_locked',
          message: 'Homeport’s database is busy.',
          hint: { text: 'Close the other Homeport.', commands: [] },
          detail: 'database is locked',
        };
      },
    });
    const failure = await commands.getSettings().catch((e: unknown) => e);
    expect(failure).toBeInstanceOf(AppError);
    expect(failure).toMatchObject({
      code: 'db_locked',
      hint: { text: 'Close the other Homeport.' },
      detail: 'database is locked',
      context: { command: 'getSettings' },
    });
  });
});
