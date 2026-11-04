import { defaultPruneSelection, toPruneRequest } from './prune';

describe('toPruneRequest', () => {
  it('removes dangling images when all unused images go', () => {
    expect(
      toPruneRequest({ ...defaultPruneSelection, images: false, allImages: true }),
    ).toMatchObject({ images: true, allImages: true });
  });

  it('keeps the selection as chosen otherwise', () => {
    expect(toPruneRequest(defaultPruneSelection)).toEqual(defaultPruneSelection);
  });
});
