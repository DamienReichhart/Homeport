import { repositoryProblem } from './repository';

describe('adding a chart repository', () => {
  it('takes a name and an http(s) URL', () => {
    expect(repositoryProblem('bitnami', 'https://charts.bitnami.com/bitnami')).toBeNull();
    expect(repositoryProblem(' local ', 'http://127.0.0.1:8879 ')).toBeNull();
  });

  it('says what is wrong otherwise', () => {
    expect(repositoryProblem('', 'https://x')).toBe('helm.repos.problem.noName');
    expect(repositoryProblem('a b', 'https://x')).toBe('helm.repos.problem.name');
    expect(repositoryProblem('a', '')).toBe('helm.repos.problem.noUrl');
    expect(repositoryProblem('a', 'oci://registry-1.docker.io/bitnamicharts')).toBe(
      'helm.repos.problem.oci',
    );
    expect(repositoryProblem('a', 'charts.example.com')).toBe('helm.repos.problem.url');
    expect(repositoryProblem('a', 'https://me:pw@charts.example.com')).toBe(
      'helm.repos.problem.credentials',
    );
  });
});
