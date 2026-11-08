import { serviceView } from '@/test/fixtures';
import { canScale, parseReplicas, portsLabel, underReplicated } from './services';

describe('services', () => {
  it('tells an under-replicated service', () => {
    expect(underReplicated(serviceView({ running: 1 }))).toBe(true);
    expect(underReplicated(serviceView())).toBe(false);
  });

  it('only scales replicated services', () => {
    expect(canScale(serviceView())).toBe(true);
    expect(canScale(serviceView({ mode: 'global', replicas: null }))).toBe(false);
  });

  it('reads a number of replicas', () => {
    expect(parseReplicas(' 3 ')).toBe(3);
    expect(parseReplicas('0')).toBe(0);
    expect(parseReplicas('-1')).toBeNull();
    expect(parseReplicas('2.5')).toBeNull();
    expect(parseReplicas('10001')).toBeNull();
  });

  it('labels published and unpublished ports', () => {
    expect(
      portsLabel(
        serviceView({
          ports: [
            { target: 80, published: 8080, protocol: 'tcp', mode: 'ingress' },
            { target: 53, published: null, protocol: 'udp', mode: 'host' },
          ],
        }),
      ),
    ).toBe('8080→80/tcp, 53/udp');
  });
});
