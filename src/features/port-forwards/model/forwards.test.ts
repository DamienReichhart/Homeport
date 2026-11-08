import { describe, expect, it } from 'vitest';
import { parseLocalPort, targetLabel } from './forwards';

describe('forwards', () => {
  it('names targets the way kubectl does', () => {
    expect(targetLabel({ kind: 'service', namespace: 'shop', name: 'api' })).toBe('svc/api');
    expect(targetLabel({ kind: 'pod', namespace: 'shop', name: 'api-1' })).toBe('pod/api-1');
  });

  it('reads a local port, empty meaning any free one', () => {
    expect(parseLocalPort(' ')).toBeNull();
    expect(parseLocalPort('8080')).toBe(8080);
    expect(parseLocalPort('0')).toBeUndefined();
    expect(parseLocalPort('70000')).toBeUndefined();
    expect(parseLocalPort('80a')).toBeUndefined();
  });
});
