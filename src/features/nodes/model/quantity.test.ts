import { bytesOf, coresOf, readableQuantity } from './quantity';

describe('Kubernetes quantities', () => {
  it('reads bytes in binary and decimal units', () => {
    expect(bytesOf('16Ki')).toBe(16384);
    expect(bytesOf('1Gi')).toBe(1024 ** 3);
    expect(bytesOf('512M')).toBe(512e6);
    expect(bytesOf('76224326324')).toBe(76224326324);
    expect(bytesOf('lots')).toBeNull();
  });

  it('reads CPUs in cores and millicores', () => {
    expect(coresOf('3920m')).toBe(3.92);
    expect(coresOf('4')).toBe(4);
    expect(coresOf('x')).toBeNull();
  });

  it('shows each resource as people read it', () => {
    expect(readableQuantity('memory', '15335132Ki')).toBe('14.6 GB');
    expect(readableQuantity('cpu', '3920m')).toBe('3.92');
    expect(readableQuantity('ephemeral-storage', '83873772Ki')).toBe('80.0 GB');
    expect(readableQuantity('pods', '58')).toBe('58');
    expect(readableQuantity('memory', null)).toBe('—');
  });
});
