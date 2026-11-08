import {
  isOci,
  namespaceProblem,
  ociProblem,
  releaseNameProblem,
  suggestedName,
  timeoutSeconds,
} from './install';

describe('installing a chart', () => {
  it('suggests the chart’s name as the release’s', () => {
    expect(suggestedName('bitnami/nginx')).toBe('nginx');
    expect(suggestedName('oci://registry-1.docker.io/bitnamicharts/Redis_HA')).toBe('redis-ha');
  });

  it('checks release names like helm', () => {
    expect(releaseNameProblem('web')).toBeNull();
    expect(releaseNameProblem('web.v2')).toBeNull();
    expect(releaseNameProblem('')).toBe('helm.install.problem.noName');
    expect(releaseNameProblem('Web')).toBe('helm.install.problem.name');
    expect(releaseNameProblem('web-')).toBe('helm.install.problem.name');
    expect(releaseNameProblem('a'.repeat(54))).toBe('helm.install.problem.nameLong');
  });

  it('checks Namespaces', () => {
    expect(namespaceProblem('shop')).toBeNull();
    expect(namespaceProblem('')).toBe('helm.install.problem.noNamespace');
    expect(namespaceProblem('shop.eu')).toBe('helm.install.problem.namespace');
  });

  it('takes OCI references with a registry and a path', () => {
    expect(isOci('OCI://ghcr.io/x/y')).toBe(true);
    expect(ociProblem('oci://registry-1.docker.io/bitnamicharts/nginx')).toBeNull();
    expect(ociProblem('oci://registry-1.docker.io')).toBe('helm.install.problem.oci');
    expect(ociProblem('bitnami/nginx')).toBe('helm.install.problem.oci');
  });

  it('turns the minutes typed into helm’s timeout', () => {
    expect(timeoutSeconds('5')).toBe(300);
    expect(timeoutSeconds('0.5')).toBe(30);
    expect(timeoutSeconds('')).toBeNull();
    expect(timeoutSeconds('x')).toBeNull();
  });
});
