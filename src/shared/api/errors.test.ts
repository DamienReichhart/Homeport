import { AppError, toAppError } from './errors';

describe('toAppError', () => {
  it('passes AppError through', () => {
    const e = new AppError({ code: 'x', message: 'boom' });
    expect(toAppError(e)).toBe(e);
  });

  it('wraps backend payloads', () => {
    const e = toAppError({ code: 'settings_io', message: 'disk full' });
    expect(e).toBeInstanceOf(AppError);
    expect(e.code).toBe('settings_io');
    expect(e.message).toBe('disk full');
  });

  it('wraps unknown values', () => {
    expect(toAppError('nope').code).toBe('unknown');
    expect(toAppError(new Error('bad')).message).toBe('bad');
  });
});
