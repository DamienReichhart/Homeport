/**
 * The nonce the app's Content Security Policy allows inline styles with.
 *
 * Built apps serve `index.html` through Tauri, which puts a fresh nonce on
 * the page's own `<style>` and adds it to `style-src`. A `style-src` with a
 * nonce makes the webview ignore `'unsafe-inline'`, so a library that adds a
 * `<style>` of its own (CodeMirror does) must carry the same nonce, or its
 * rules never apply. `undefined` in development and tests, where there is
 * no nonce and inline styles are allowed.
 */
export function styleNonce(root: ParentNode = document): string | undefined {
  const styled = root.querySelector<HTMLElement>('style[nonce], link[nonce]');
  // The browser hides the attribute once the page loads; the property keeps it.
  const nonce = styled?.nonce || styled?.getAttribute('nonce');
  return nonce || undefined;
}
