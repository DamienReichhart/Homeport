import { canCreateNetwork, emptyNetwork, toNetworkInput } from './networkInput';

describe('network input', () => {
  it('needs a name, and nothing else', () => {
    expect(canCreateNetwork(emptyNetwork)).toBe(false);
    expect(canCreateNetwork({ ...emptyNetwork, name: '   ' })).toBe(false);
    expect(canCreateNetwork({ ...emptyNetwork, name: 'backend' })).toBe(true);
  });

  it('leaves the Engine’s defaults alone', () => {
    expect(toNetworkInput({ ...emptyNetwork, name: 'backend' })).toEqual({
      name: 'backend',
      driver: null,
      subnet: null,
      gateway: null,
      internal: false,
      attachable: false,
    });
  });

  it('trims what was typed and treats blank fields as automatic', () => {
    expect(
      toNetworkInput({
        name: ' backend ',
        driver: ' overlay ',
        subnet: ' 10.10.0.0/16 ',
        gateway: '  ',
        internal: true,
        attachable: true,
      }),
    ).toEqual({
      name: 'backend',
      driver: 'overlay',
      subnet: '10.10.0.0/16',
      gateway: null,
      internal: true,
      attachable: true,
    });
  });
});
