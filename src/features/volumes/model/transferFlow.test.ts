import { reduceFlow, startFlow, volumeNameFromFile, type FlowEvent } from './transferFlow';

const run = (...events: FlowEvent[]) => events.reduce(reduceFlow, startFlow);

describe('volume transfer flow', () => {
  it('goes straight to the image when nothing running uses the volume', () => {
    expect(run({ type: 'checked', users: [] })).toEqual({ kind: 'image', stop: [] });
    expect(run({ type: 'checked', users: [] }, { type: 'imageReady', pulledImage: true })).toEqual({
      kind: 'transfer',
      stop: [],
      pulledImage: true,
      replace: false,
    });
  });

  it('asks about running users, and stops the ones chosen', () => {
    const asked = run({ type: 'checked', users: ['db-1'] });
    expect(asked).toEqual({ kind: 'running', users: ['db-1'] });
    expect(reduceFlow(asked, { type: 'proceed', stop: ['db-1'] })).toEqual({
      kind: 'image',
      stop: ['db-1'],
    });
  });

  it('asks before replacing a volume that holds files, once', () => {
    const found = run(
      { type: 'checked', users: [] },
      { type: 'imageReady', pulledImage: false },
      { type: 'notEmpty' },
    );
    expect(found).toEqual({ kind: 'replace', stop: [], pulledImage: false });
    const replacing = reduceFlow(found, { type: 'replace' });
    expect(replacing).toEqual({ kind: 'transfer', stop: [], pulledImage: false, replace: true });
    expect(reduceFlow(replacing, { type: 'notEmpty' })).toBe(replacing);
  });

  it('names a new volume after the backup file', () => {
    expect(volumeNameFromFile('/home/me/shop_db.tar')).toBe('shop_db');
    expect(volumeNameFromFile('C:\\backups\\my db (1).TAR')).toBe('my-db--1-');
    expect(volumeNameFromFile('/x/.tar')).toBe('');
  });
});
