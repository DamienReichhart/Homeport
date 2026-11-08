import { describe, expect, it } from 'vitest';
import { incusInstance } from '@/test/incus';
import { cpuSeconds, filterByType, instanceActions, isUp, statusTone } from './instances';

describe('instance model', () => {
  it('offers the actions that make sense in each state, the main one first', () => {
    expect(instanceActions('running')).toEqual(['stop', 'restart', 'freeze']);
    expect(instanceActions('frozen')).toEqual(['unfreeze', 'stop']);
    expect(instanceActions('stopped')).toEqual(['start']);
    expect(instanceActions('starting')).toEqual([]);
  });

  it('tells running Instances apart for deleting them', () => {
    expect(isUp('frozen')).toBe(true);
    expect(isUp('stopped')).toBe(false);
    expect(statusTone('running')).toBe('success');
    expect(statusTone('error')).toBe('danger');
  });

  it('filters by type', () => {
    const list = [incusInstance('web'), incusInstance('vm', { type: 'virtual-machine' })];
    expect(filterByType(list, 'all')).toHaveLength(2);
    expect(filterByType(list, 'virtual-machine').map((i) => i.name)).toEqual(['vm']);
  });

  it('shows CPU time in seconds', () => {
    expect(cpuSeconds(270_905_168)).toBe('0.3');
  });
});
