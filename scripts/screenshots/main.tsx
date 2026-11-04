/**
 * Renders the real Homeport frontend against mocked IPC, for the website's
 * screenshots. The query string picks what to show:
 *
 *   route=/containers   the page (default `/`)
 *   theme=dark          `light` or `dark` (default `light`)
 *   host=<id>           the selected Host (default prod-eu-1)
 *   step=click:Label    click the button or tab with that text (repeatable)
 *   step=fill:Placeholder=value   type into the field with that placeholder
 *   step=wait:500       pause between steps
 *   clock=fast          run time 25× faster, so stats charts fill quickly
 *   hold=3000           keep the page loading this long, so `firefox
 *                       --screenshot` waits for the data to render
 */
const params = new URLSearchParams(location.search);

if (params.get('clock') === 'fast') {
  const real = Date.now.bind(Date);
  const start = real();
  Date.now = () => start + (real() - start) * 25;
}

// `firefox --screenshot` captures on the load event; this image delays it.
const hold = document.createElement('img');
hold.src = `/__hold?ms=${Number(params.get('hold') ?? 2500)}`;
hold.alt = '';
hold.style.display = 'none';
document.body.append(hold);

// Imported after the clock patch, so module-level timestamps use it.
const [{ StrictMode }, { createRoot }, { RouterProvider }, providers, router, mocks, data] =
  await Promise.all([
    import('react'),
    import('react-dom/client'),
    import('react-router-dom'),
    import('@/app/providers/AppProviders'),
    import('@/app/router/router'),
    import('./mocks'),
    import('./data'),
  ]);
await import('@/shared/styles/global.css');

const theme = params.get('theme') === 'dark' ? 'dark' : 'light';
document.documentElement.classList.toggle('dark', theme === 'dark');
mocks.installMocks({ theme, hostId: params.get('host') ?? data.PROD });

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <providers.AppProviders>
      <RouterProvider router={router.createAppRouter([params.get('route') ?? '/'])} />
    </providers.AppProviders>
  </StrictMode>,
);

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

function byText(text: string): HTMLElement | undefined {
  const candidates = document.querySelectorAll<HTMLElement>(
    'button, [role="tab"], [role="menuitem"], [role="option"], a',
  );
  return [...candidates].find((el) => el.textContent?.trim() === text);
}

function click(text: string) {
  const el = byText(text);
  if (!el) return console.warn(`[screenshots] nothing to click labelled "${text}"`);
  // Radix triggers react to pointer/mouse down, plain buttons to click.
  const init = { bubbles: true, cancelable: true, button: 0, pointerType: 'mouse' };
  el.dispatchEvent(new PointerEvent('pointerdown', init));
  el.dispatchEvent(new MouseEvent('mousedown', init));
  el.dispatchEvent(new PointerEvent('pointerup', init));
  el.dispatchEvent(new MouseEvent('mouseup', init));
  el.click();
}

function fill(placeholder: string, value: string) {
  const el = document.querySelector<HTMLInputElement | HTMLTextAreaElement>(
    `input[placeholder="${placeholder}"], textarea[placeholder="${placeholder}"]`,
  );
  if (!el) return console.warn(`[screenshots] no field with placeholder "${placeholder}"`);
  const proto =
    el instanceof HTMLTextAreaElement ? HTMLTextAreaElement.prototype : HTMLInputElement.prototype;
  Object.getOwnPropertyDescriptor(proto, 'value')?.set?.call(el, value);
  el.dispatchEvent(new Event('input', { bubbles: true }));
}

async function runSteps() {
  await sleep(700);
  for (const step of params.getAll('step')) {
    const [kind, ...rest] = step.split(':');
    const arg = rest.join(':');
    if (kind === 'click') click(arg);
    else if (kind === 'fill') {
      const [placeholder = '', ...value] = arg.split('=');
      fill(placeholder, value.join('='));
    } else if (kind === 'wait') await sleep(Number(arg));
    await sleep(150);
  }
  (document.activeElement as HTMLElement | null)?.blur?.();
}

void runSteps();
