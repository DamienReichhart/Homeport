import type { PodmanPod } from '@/shared/api';
import {
  canCreatePod,
  createdSeconds,
  emptyPodForm,
  isUp,
  members,
  podActions,
  podFormErrors,
  podTone,
  runningMembers,
  toPodSpec,
} from './pods';

const pod: PodmanPod = {
  id: 'p1',
  name: 'web',
  status: 'degraded',
  created: '2026-11-08T10:00:00.123456789Z',
  infraId: 'i',
  labels: {},
  networks: ['podman'],
  containers: [
    { id: 'i', name: 'web-infra', state: 'running', infra: true },
    { id: 'a', name: 'web-app', state: 'running', infra: false },
    { id: 'b', name: 'web-db', state: 'exited', infra: false },
  ],
};

describe('pods', () => {
  it('counts the pod’s own containers, not its infra container', () => {
    expect(members(pod).map((c) => c.name)).toEqual(['web-app', 'web-db']);
    expect(runningMembers(pod)).toBe(1);
  });

  it('offers the actions that apply to its status', () => {
    expect(podActions('running')).toEqual(['stop', 'restart', 'pause']);
    expect(podActions('degraded')).toEqual(['stop', 'restart', 'pause']);
    expect(podActions('paused')).toEqual(['unpause', 'stop']);
    expect(podActions('exited')).toEqual(['start']);
    expect(podActions('created')).toEqual(['start']);
    expect(isUp('degraded')).toBe(true);
    expect(isUp('stopped')).toBe(false);
  });

  it('colours a partly running pod as a warning', () => {
    expect(podTone('running')).toBe('success');
    expect(podTone('degraded')).toBe('warning');
    expect(podTone('exited')).toBe('muted');
    expect(podTone('error')).toBe('danger');
  });

  it('reads Podman’s creation time', () => {
    expect(createdSeconds(pod)).toBe(Date.UTC(2026, 10, 8, 10) / 1000);
    expect(createdSeconds({ ...pod, created: '' })).toBe(0);
  });
});

describe('new pod form', () => {
  it('builds the request, dropping empty rows', () => {
    const form = {
      name: ' web ',
      ports: [
        { hostPort: '8080', containerPort: '80', protocol: 'tcp' },
        { hostPort: '', containerPort: '53', protocol: 'udp' },
        { hostPort: '', containerPort: '', protocol: 'tcp' },
      ],
      labels: 'app=web\n\n team = ops ',
    };
    expect(canCreatePod(form)).toBe(true);
    expect(toPodSpec(form)).toEqual({
      name: 'web',
      ports: [
        { hostIp: null, hostPort: 8080, containerPort: 80, protocol: 'tcp' },
        { hostIp: null, hostPort: null, containerPort: 53, protocol: 'udp' },
      ],
      labels: ['app=web', 'team = ops'],
    });
  });

  it('says what’s wrong before sending', () => {
    expect(canCreatePod(emptyPodForm())).toBe(false);
    expect(podFormErrors({ ...emptyPodForm(), name: '-web' }).name).toBe(
      'podmanpods.form.nameInvalid',
    );
    const ports = [{ hostPort: '99999', containerPort: '80', protocol: 'tcp' }];
    expect(podFormErrors({ ...emptyPodForm(), name: 'web', ports }).ports).toBe(
      'podmanpods.form.portInvalid',
    );
    expect(podFormErrors({ ...emptyPodForm(), name: 'web', labels: 'nokey' }).labels).toBe(
      'podmanpods.form.labelInvalid',
    );
  });
});
