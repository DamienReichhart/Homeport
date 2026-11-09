import { allocationTone, jobTone, shortId, statusLabel } from './status';

describe('Nomad statuses', () => {
  it('colours a job by its status, muting a stopped one', () => {
    expect(jobTone('running', false)).toBe('success');
    expect(jobTone('pending', false)).toBe('info');
    expect(jobTone('running', true)).toBe('muted');
    expect(jobTone('dead', false)).toBe('muted');
  });

  it('colours an allocation by its client status', () => {
    expect(allocationTone('running')).toBe('success');
    expect(allocationTone('failed')).toBe('danger');
    expect(allocationTone('lost')).toBe('danger');
    expect(allocationTone('complete')).toBe('muted');
    expect(allocationTone('something-new')).toBe('warning');
  });

  it('names known statuses and shortens IDs as Nomad does', () => {
    expect(statusLabel('running')).toBe('nomad.status.running');
    expect(statusLabel('something-new')).toBeNull();
    expect(shortId('a1b2c3d4-0000-0000-0000-000000000001')).toBe('a1b2c3d4');
  });
});
