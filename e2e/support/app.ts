/**
 * Finding things in the app the way a user does: by role and accessible
 * name, or by visible text. Never by CSS classes, which are styling.
 */
import { $, browser } from '@wdio/globals';

/** An XPath string literal for `text`, whatever quotes it holds. */
function literal(text: string): string {
  if (!text.includes("'")) return `'${text}'`;
  return `concat(${text
    .split("'")
    .map((part) => `'${part}'`)
    .join(`, "'", `)})`;
}

/** The element's accessible name: its `aria-label`, else its text. */
const named = (name: string) =>
  `[@aria-label=${literal(name)} or normalize-space(.)=${literal(name)}]`;

/** Something to press: a button, or a link drawn as one. */
export const button = (name: string) =>
  $(`//*[self::button or self::a or @role='button']${named(name)}`);

/** Clicks once the element can be clicked, as a user would wait for it. */
export async function press(element: ReturnType<typeof $>): Promise<void> {
  await shown(element);
  await element.waitForClickable();
  await element.click();
}
/** A link, including a sidebar entry. */
export const link = (name: string) => $(`//a${named(name)}`);
export const tab = (name: string) => $(`//*[@role='tab']${named(name)}`);
export const menuItem = (name: string) => $(`//*[@role='menuitem']${named(name)}`);
export const dialog = () => $(`//*[@role='dialog' or @role='alertdialog']`);
/** A button of the open dialog, which may share its name with one behind it. */
export const dialogButton = (name: string) =>
  $(`//*[@role='dialog' or @role='alertdialog']//button${named(name)}`);
/** The first element whose own text contains `text`. */
export const text = (value: string) => $(`//*[text()[contains(., ${literal(value)})]]`);
/** The field labelled `label`, through its `<label for>` or its `aria-label`. */
export const field = (label: string) =>
  $(
    `//*[self::input or self::textarea][@aria-label=${literal(label)} or @id=//label[normalize-space(.)=${literal(label)}]/@for]`,
  );

/** Goes to a page through the sidebar, as a user would. */
export async function goTo(page: string): Promise<void> {
  await press($(`//nav//a${named(page)}`));
}

/** Waits until `check` holds, for things that happen outside the window. */
export async function eventually(
  check: () => boolean | Promise<boolean>,
  message: string,
  timeout = 60_000,
): Promise<void> {
  await browser.waitUntil(check, { timeout, interval: 500, timeoutMsg: message });
}

/** Replaces what a CodeMirror editor holds, the way a paste does. */
export async function fillEditor(content: string): Promise<void> {
  const editor = $(`//*[@role='textbox' and @contenteditable='true']`);
  await editor.waitForExist();
  await editor.click();
  await browser.execute((value) => {
    document.execCommand('selectAll');
    document.execCommand('insertText', false, value);
  }, content);
}

/**
 * Pastes `content` into a text field, line breaks included: WebKit's driver
 * drops them when it types, as no user pasting a key would see.
 */
export async function paste(element: ReturnType<typeof $>, content: string): Promise<void> {
  await element.waitForDisplayed();
  await element.click();
  await browser.execute((value) => {
    document.execCommand('selectAll');
    document.execCommand('insertText', false, value);
  }, content);
}

type Element = ReturnType<typeof $>;

/**
 * Whether `element` is on screen now. While React redraws the part of the
 * page being searched, WebKitWebDriver can answer "stale element" instead of
 * searching again; that means "not settled yet", not a failure.
 */
async function displayed(element: Element): Promise<boolean> {
  try {
    return await element.isDisplayed();
  } catch {
    return false;
  }
}

/** Waits until `element` is shown. */
export async function shown(element: Element, timeout = 20_000): Promise<void> {
  await browser.waitUntil(() => displayed(element), {
    timeout,
    interval: 250,
    timeoutMsg: `${await element.selector} still not displayed after ${timeout}ms`,
  });
}

/** Waits until `element` is gone or hidden. */
export async function gone(element: Element, timeout = 20_000): Promise<void> {
  await browser.waitUntil(async () => !(await displayed(element)), {
    timeout,
    interval: 250,
    timeoutMsg: `${await element.selector} still displayed after ${timeout}ms`,
  });
}
