import { isAccepted } from './spdx';

const ACCEPTED = ['MIT', 'Apache-2.0', 'Apache-2.0 WITH LLVM-exception', 'ISC'];

describe('isAccepted', () => {
  it.each([
    'MIT',
    'MIT OR GPL-3.0',
    'GPL-3.0 or MIT',
    'MIT AND ISC',
    '(MIT OR GPL-3.0) AND ISC',
    'Apache-2.0 WITH LLVM-exception',
    'MIT/GPL-3.0',
  ])('accepts %s', (expression) => {
    expect(isAccepted(expression, ACCEPTED)).toBe(true);
  });

  it.each([
    '',
    'GPL-3.0',
    'MIT AND GPL-3.0',
    '(MIT OR Apache-2.0) AND GPL-3.0',
    'GPL-3.0 WITH Classpath-exception-2.0',
    'MIT WITH Classpath-exception-2.0',
    '(MIT',
    'MIT)',
  ])('rejects %s', (expression) => {
    expect(isAccepted(expression, ACCEPTED)).toBe(false);
  });
});
