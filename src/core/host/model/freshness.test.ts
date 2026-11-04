import type { Container, Image } from '@/shared/api';
import {
  aggregateFreshness,
  containerFreshness,
  indexFreshness,
  newerLabel,
  newerTags,
} from './freshness';

const index = indexFreshness([
  { reference: 'nginx:1.27', imageId: 'sha256:old', freshness: 'newer' },
  { reference: 'nginx:1.27', imageId: 'sha256:new', freshness: 'upToDate' },
  { reference: 'nginx:latest', imageId: 'sha256:new', freshness: 'newer' },
  { reference: 'app:dev', imageId: 'sha256:app', freshness: 'unknown' },
]);

describe('freshness', () => {
  it('reads a container by its reference and the image it runs', () => {
    const c = (image: string, imageId: string) => ({ image, imageId }) as Container;
    expect(containerFreshness(index, c('nginx:1.27', 'sha256:old'))).toBe('newer');
    expect(containerFreshness(index, c('nginx:1.27', 'sha256:new'))).toBe('upToDate');
    expect(containerFreshness(index, c('redis', 'sha256:r'))).toBe('unknown');
  });

  it('lists the tags of an image that moved on', () => {
    const image = { id: 'sha256:new', tags: ['nginx:1.27', 'nginx:latest'] } as Image;
    expect(newerTags(index, image)).toEqual(['nginx:latest']);
  });

  it('aggregates a Stack from its services', () => {
    expect(aggregateFreshness(['upToDate', 'newer', 'unknown'])).toBe('newer');
    expect(aggregateFreshness(['upToDate', 'unknown'])).toBe('upToDate');
    expect(aggregateFreshness(['unknown', 'unknown'])).toBe('unknown');
    expect(aggregateFreshness([])).toBe('unknown');
  });

  it('says what was published', () => {
    expect(newerLabel(['nginx:1.27'])).toBe('A newer nginx:1.27 was published');
    expect(newerLabel(['a:1', 'b:2'])).toBe('Newer images were published: a:1, b:2');
  });
});
