/** The home page's departure board, in the browser (ui/sections/Board.astro). */
const marks: Record<string, string> = { success: '●', warning: '◆', danger: '●' };

/**
 * Makes the departure boards on the page live: a row opens when picked, and a
 * row with a `data-flip` status changes to it and back while its board is in
 * view (not when the visitor prefers reduced motion).
 */
export function enhanceBoards() {
  for (const board of document.querySelectorAll<HTMLElement>('[data-board]')) {
    const rows = [...board.querySelectorAll<HTMLElement>('[data-row]')];

    for (const row of rows) {
      row.querySelector('button')?.addEventListener('click', () => {
        for (const other of rows) {
          const open = other === row;
          other.classList.toggle('is-open', open);
          other.querySelector('button')?.setAttribute('aria-expanded', String(open));
        }
      });
    }

    // A row that changes status while the board is in view, like a real one.
    const live = rows.filter((row) => row.dataset.flip);
    if (live.length === 0 || matchMedia('(prefers-reduced-motion: reduce)').matches) continue;
    const set = (row: HTMLElement, status: string, tone: string) => {
      const chip = row.querySelector<HTMLElement>('.board__status');
      const text = row.querySelector('[data-status-text]');
      const mark = row.querySelector('[data-mark]');
      if (!chip || !text || !mark) return;
      chip.dataset.tone = tone;
      chip.classList.remove('is-changing');
      void chip.offsetWidth;
      chip.classList.add('is-changing');
      text.textContent = status;
      mark.textContent = marks[tone] ?? '●';
    };
    let timer: number | undefined;
    let flipped = false;
    const tick = () => {
      flipped = !flipped;
      for (const row of live) {
        const flip = JSON.parse(row.dataset.flip ?? '{}') as { status: string; tone: string };
        if (flipped) set(row, flip.status, flip.tone);
        else set(row, row.dataset.status ?? '', row.dataset.tone ?? 'success');
      }
    };
    new IntersectionObserver(([entry]) => {
      if (entry?.isIntersecting) timer ??= window.setInterval(tick, 3200);
      else if (timer !== undefined) {
        window.clearInterval(timer);
        timer = undefined;
      }
    }).observe(board);
  }
}
