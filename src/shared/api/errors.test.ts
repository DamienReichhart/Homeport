import { errorMessages } from '@/shared/i18n/en';
import { AppError, errorReport, toAppError, UNEXPECTED } from './errors';

describe('toAppError', () => {
  it('passes AppError through', () => {
    const e = new AppError({ code: 'validation', message: 'boom' });
    expect(toAppError(e)).toBe(e);
  });

  it('wraps backend payloads', () => {
    const e = toAppError({
      code: 'permission_denied',
      message: 'Access refused.',
      hint: { text: 'Join the docker group.', commands: ['sudo usermod -aG docker "$USER"'] },
      detail: 'Permission denied (os error 13)',
    });
    expect(e).toBeInstanceOf(AppError);
    expect(e.code).toBe('permission_denied');
    expect(e.message).toBe('Access refused.');
    expect(e.hint?.commands).toEqual(['sudo usermod -aG docker "$USER"']);
    expect(e.detail).toBe('Permission denied (os error 13)');
  });

  it('tolerates payloads without hint or detail', () => {
    const e = toAppError({ code: 'validation', message: 'Enter a name.' });
    expect(e.hint).toBeNull();
    expect(e.detail).toBeNull();
  });

  it('keeps raw text of unexpected failures out of the message', () => {
    const fromString = toAppError('command not found');
    expect(fromString.code).toBe('unknown');
    expect(fromString.message).toBe(UNEXPECTED);
    expect(fromString.detail).toBe('command not found');

    const crash = toAppError(new TypeError("Cannot read properties of null (reading 'filter')"));
    expect(crash.message).toBe(UNEXPECTED);
    expect(crash.detail).toContain("TypeError: Cannot read properties of null (reading 'filter')");
  });
});

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
