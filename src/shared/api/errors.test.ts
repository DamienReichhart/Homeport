import { AppError, toAppError } from './errors';

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

  it('wraps unknown values', () => {
    expect(toAppError('nope').code).toBe('unknown');
    expect(toAppError(new Error('bad')).message).toBe('bad');
  });
});
