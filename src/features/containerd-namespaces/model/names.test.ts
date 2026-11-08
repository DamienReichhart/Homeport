import { namespaceNameValid } from './names';

describe('namespace names', () => {
  it('are letters and digits separated by single dots, dashes or underscores', () => {
    for (const good of ['default', 'k8s.io', 'my-ns_2'])
      expect(namespaceNameValid(good)).toBe(true);
    for (const bad of ['', 'two words', '-lead', 'a..b', 'x'.repeat(77)])
      expect(namespaceNameValid(bad)).toBe(false);
  });
});
