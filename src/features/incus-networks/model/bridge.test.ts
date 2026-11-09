import { describe, expect, it } from 'vitest';
import { validAddress, validNetworkName } from './bridge';

describe('bridge settings', () => {
  it('take auto, none or an address with its prefix', () => {
    expect(validAddress('auto', 4)).toBe(true);
    expect(validAddress('10.0.5.1/24', 4)).toBe(true);
    expect(validAddress('10.0.5.1', 4)).toBe(false);
    expect(validAddress('300.0.5.1/24', 4)).toBe(false);
    expect(validAddress('fd42:1::1/64', 6)).toBe(true);
    expect(validAddress('fd42:1::1/129', 6)).toBe(false);
  });

  it('name it like a Linux interface', () => {
    expect(validNetworkName('lab0')).toBe(true);
    expect(validNetworkName('a-very-long-bridge-name')).toBe(false);
    expect(validNetworkName('my lab')).toBe(false);
  });
});
