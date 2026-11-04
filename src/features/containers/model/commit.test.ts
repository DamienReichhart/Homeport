import type { Image } from '@/shared/api';
import { commitName, emptyCommitForm, imageNamed, toCommitSpec } from './commit';

const image = (tags: string[]): Image => ({
  id: 'sha256:a',
  tags,
  digests: [],
  created: 0,
  size: 0,
  containers: 0,
  labels: {},
});

describe('committing', () => {
  it('suggests a dated, lower-case name', () => {
    expect(commitName('Shop_Web-1', new Date('2026-11-04T10:00:00Z'))).toBe(
      'shop_web-1:2026-11-04',
    );
  });

  it('sends the changes one per line, without blanks', () => {
    const spec = toCommitSpec({
      ...emptyCommitForm('web'),
      reference: ' web:backup ',
      comment: 'Before the upgrade',
      changes: 'ENV MODE=prod\n\nEXPOSE 8080\n',
    });
    expect(spec).toEqual({
      reference: 'web:backup',
      comment: 'Before the upgrade',
      author: null,
      pause: true,
      changes: ['ENV MODE=prod', 'EXPOSE 8080'],
    });
  });

  it('finds the image whose tag the commit would move', () => {
    const images = [image(['web:backup']), image(['registry.home:5000/app:latest'])];
    expect(imageNamed('web:backup', images)).toBe(images[0]);
    expect(imageNamed('registry.home:5000/app', images)).toBe(images[1]);
    expect(imageNamed('web:other', images)).toBeUndefined();
  });
});
