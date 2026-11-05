import { initialTransfer, reduceTransfer } from './transfer';

describe('file transfers', () => {
  it('follows bytes, lines and the names at the end', () => {
    let s = reduceTransfer(initialTransfer, { type: 'progress', bytes: 1024, total: 4096 });
    s = reduceTransfer(s, { type: 'progress', bytes: 2048, total: null });
    expect([s.bytes, s.total]).toEqual([2048, 4096]);
    s = reduceTransfer(s, { type: 'line', text: 'Loaded image: nginx:1.27' });
    s = reduceTransfer(s, { type: 'done', images: ['nginx:1.27'] });
    expect(s).toMatchObject({
      lines: ['Loaded image: nginx:1.27'],
      images: ['nginx:1.27'],
      done: true,
    });
  });
});
