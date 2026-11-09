import { effectiveNamespace } from './namespace';

const listing = {
  listable: true,
  namespaces: [
    { name: 'default', description: null },
    { name: 'shop', description: null },
  ],
};

describe('the namespace the Jobs page asks for', () => {
  it('is the one chosen, or every one', () => {
    expect(effectiveNamespace(listing, 'shop')).toBe('shop');
    expect(effectiveNamespace(listing, null)).toBe('*');
  });

  it('falls back to default for a namespace that’s gone', () => {
    expect(effectiveNamespace(listing, 'billing')).toBe('default');
  });

  it('is the token’s default while the namespaces can’t be listed', () => {
    expect(effectiveNamespace({ listable: false, namespaces: [] }, 'shop')).toBeNull();
    expect(effectiveNamespace(undefined, null)).toBeNull();
  });
});
