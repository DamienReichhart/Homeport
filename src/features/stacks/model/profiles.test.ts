import { splitProfiles, toggle } from './profiles';

describe('a Stack’s profiles', () => {
  it('drops the profiles the definition no longer declares', () => {
    expect(splitProfiles(['debug', 'old'], ['debug', 'tools'])).toEqual({
      kept: ['debug'],
      dropped: ['old'],
    });
    expect(splitProfiles([], ['debug'])).toEqual({ kept: [], dropped: [] });
  });

  it('turns profiles on and off in the definition’s order', () => {
    const declared = ['debug', 'tools', 'workers'];
    expect(toggle(declared, ['workers'], 'debug', true)).toEqual(['debug', 'workers']);
    expect(toggle(declared, ['debug', 'workers'], 'debug', false)).toEqual(['workers']);
  });
});
