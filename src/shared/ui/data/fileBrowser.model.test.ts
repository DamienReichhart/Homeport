import { crumbs } from './fileBrowser.model';

describe('crumbs', () => {
  it('splits a path into clickable parents', () => {
    expect(crumbs('/')).toEqual([['/', '/']]);
    expect(crumbs('/usr/local/bin')).toEqual([
      ['/', '/'],
      ['usr', '/usr'],
      ['local', '/usr/local'],
      ['bin', '/usr/local/bin'],
    ]);
  });
});
