import { containerdContainer as ctrContainer } from '@/test/containerd';
import { isUp, managedByKubernetes, taskActions, visibleContainers } from './containers';

describe('containerd containers', () => {
  it('offer the task actions their state allows', () => {
    expect(taskActions('running')).toEqual(['stop', 'pause']);
    expect(taskActions('paused')).toEqual(['resume', 'stop']);
    expect(taskActions('stopped')).toEqual(['start']);
    expect(taskActions('none')).toEqual(['start']);
    expect(isUp('paused')).toBe(true);
    expect(isUp('created')).toBe(false);
  });

  it('hide Pods’ pause containers unless asked, and tell the kubelet’s apart', () => {
    const list = [ctrContainer('web'), ctrContainer('pause', { sandbox: true })];
    expect(visibleContainers(list, false).map((c) => c.id)).toEqual(['web']);
    expect(visibleContainers(list, true)).toHaveLength(2);
    expect(managedByKubernetes(ctrContainer('dns', { createdBy: 'kubernetes' }))).toBe(true);
    expect(managedByKubernetes(ctrContainer('web'))).toBe(false);
  });
});
