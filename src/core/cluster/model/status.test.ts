import type { ClusterStatus } from '@/shared/api';
import { clusterStatusError, describeClusterStatus } from './status';

const error = { code: 'cluster_unauthorized', message: 'kind-dev refused the credentials' };

describe('describeClusterStatus', () => {
  it('names the API server’s version once connected', () => {
    const s = describeClusterStatus({
      state: 'connected',
      serverVersion: 'v1.33.1',
      platform: 'linux/arm64',
    });
    expect(s).toMatchObject({ label: 'Connected', tone: 'success', detail: 'Kubernetes v1.33.1' });
  });

  it('says when it retries on its own', () => {
    const status = { state: 'unreachable', error, retryInSecs: 4 } as unknown as ClusterStatus;
    expect(describeClusterStatus(status).detail).toBe(
      'kind-dev refused the credentials — retrying in 4s',
    );
  });

  it('gives the cause of a status that needs the user', () => {
    const status = { state: 'unauthorized', error } as unknown as ClusterStatus;
    expect(describeClusterStatus(status).tone).toBe('danger');
    expect(clusterStatusError(status)).toBe(error);
    expect(clusterStatusError({ state: 'connecting' })).toBeNull();
  });
});
