import { styleNonce } from './csp';

describe('styleNonce', () => {
  it('reads the nonce the page’s own style was served with', () => {
    const root = document.createElement('div');
    root.innerHTML = '<style>html{}</style><style nonce="8475">html{}</style>';
    expect(styleNonce(root)).toBe('8475');
  });

  it('is undefined when nothing carries one, as in development', () => {
    const root = document.createElement('div');
    root.innerHTML = '<style>html{}</style>';
    expect(styleNonce(root)).toBeUndefined();
  });
});
