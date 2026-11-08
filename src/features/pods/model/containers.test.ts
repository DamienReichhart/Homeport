import { describe, expect, it } from 'vitest';
import type { PodContainer, PodDetail } from '@/shared/api';
import { defaultContainer, hasPrevious } from './containers';

const container = (name: string, extra: Partial<PodContainer> = {}): PodContainer => ({
  name,
  image: 'img',
  init: false,
  state: 'running',
  reason: null,
  message: null,
  started: null,
  exitCode: null,
  ready: true,
  restarts: 0,
  lastReason: null,
  ports: [],
  ...extra,
});

const pod = (containers: PodContainer[], annotations: Record<string, string> = {}) =>
  ({ containers, annotations }) as unknown as PodDetail;

describe('defaultContainer', () => {
  it('picks the container the annotation names', () => {
    const p = pod([container('init', { init: true }), container('api'), container('envoy')], {
      'kubectl.kubernetes.io/default-container': 'envoy',
    });
    expect(defaultContainer(p)).toBe('envoy');
  });

  it('else the first app container, not an init one', () => {
    expect(defaultContainer(pod([container('init', { init: true }), container('api')]))).toBe(
      'api',
    );
    expect(
      defaultContainer(
        pod([container('api')], { 'kubectl.kubernetes.io/default-container': 'gone' }),
      ),
    ).toBe('api');
  });
});

describe('hasPrevious', () => {
  it('is true once a container restarted', () => {
    expect(hasPrevious(container('api', { restarts: 3 }))).toBe(true);
    expect(hasPrevious(container('api'))).toBe(false);
    expect(hasPrevious(undefined)).toBe(false);
  });
});
