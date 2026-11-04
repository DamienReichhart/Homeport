import { searchTerm, shortRepository } from './hub';

describe('Docker Hub search', () => {
  it('searches only bare Docker Hub names', () => {
    expect(searchTerm('postg')).toBe('postg');
    expect(searchTerm(' bitnami/red ')).toBe('bitnami/red');
    expect(searchTerm('p')).toBeNull();
    expect(searchTerm('postgres:17')).toBeNull();
    expect(searchTerm('app@sha256:abc')).toBeNull();
    expect(searchTerm('ghcr.io/org/app')).toBeNull();
    expect(searchTerm('localhost:5000/app')).toBeNull();
  });

  it('shows official repositories without library/', () => {
    expect(shortRepository('library/postgres')).toBe('postgres');
    expect(shortRepository('bitnami/redis')).toBe('bitnami/redis');
  });
});
