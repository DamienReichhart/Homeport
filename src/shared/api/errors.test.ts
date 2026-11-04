import { errorMessages } from '@/shared/i18n/en';
import { errorReport, toAppError } from './errors';

const [someKey, someTemplate] = Object.entries(errorMessages)[0] ?? [];

describe('errors from the backend', () => {
  it('show their English message when they have no key', () => {
    const e = toAppError({
      code: 'validation',
      key: '',
      params: {},
      message: 'Enter a name.',
      hint: { key: '', params: {}, text: 'Names are short.', commands: ['ls'] },
      detail: null,
    });
    expect(e.message).toBe('Enter a name.');
    expect(e.hint).toMatchObject({ text: 'Names are short.', commands: ['ls'] });
  });

  it('fall back to the English message for a key the catalogue lacks, and say so', () => {
    const warn = vi.spyOn(console, 'warn').mockImplementation(() => {});
    const e = toAppError({
      code: 'port_in_use',
      key: 'errors.test.fromANewerBackend',
      params: { port: 80 },
      message: 'Port 80 is already in use.',
      hint: null,
      detail: null,
    });
    expect(e.message).toBe('Port 80 is already in use.');
    expect(warn).toHaveBeenCalledWith(expect.stringContaining('errors.test.fromANewerBackend'));
    warn.mockRestore();
  });

  it.skipIf(!someKey)(
    'render a keyed message from the catalogue, keeping the English for reports',
    () => {
      const e = toAppError({
        code: 'validation',
        key: someKey ?? '',
        params: {},
        message: 'English from the backend',
        hint: null,
        detail: null,
      });
      expect(e.message).toBe(someTemplate?.replace(/\{\w+\}/g, (m) => m));
      expect(e.english).toBe('English from the backend');
      expect(errorReport(e)).toContain('English from the backend');
    },
  );

  it('keep their key when tagged with a context', () => {
    const e = toAppError({
      code: 'io',
      key: '',
      params: {},
      message: 'Failed.',
      hint: null,
      detail: null,
    });
    const tagged = e.withContext({ command: 'containersAct' });
    expect(tagged.message).toBe('Failed.');
    expect(tagged.context.command).toBe('containersAct');
  });
});
