import { browser, expect } from '@wdio/globals';
import {
  button,
  dialog,
  dialogButton,
  eventually,
  field,
  fillEditor,
  goTo,
  link,
  menuItem,
  press,
  text,
  gone,
  shown,
} from '../support/app';
import { docker, isRunning, tryDocker } from '../support/docker';

const IMAGE = process.env.HOMEPORT_E2E_IMAGE ?? 'mirror.gcr.io/library/alpine:3.22';
const CONTAINER = 'homeport-e2e-web';
const PULLED = process.env.HOMEPORT_E2E_PULL ?? 'mirror.gcr.io/library/busybox:1.37';
const STACK = 'homeport-e2e-stack';
const GREETING = 'sealed-hello';

/** The environment of the Stack's container, as the Engine has it. */
function stackEnv(): string {
  const id = tryDocker('ps', '--quiet', '--filter', `label=com.docker.compose.project=${STACK}`);
  if (!id) return '';
  return (
    tryDocker('inspect', '--format', '{{join .Config.Env "\\n"}}', id.split('\n')[0] ?? '') ?? ''
  );
}

describe('Homeport', () => {
  before(() => {
    tryDocker('rm', '--force', CONTAINER);
    docker(
      'run',
      '--detach',
      '--name',
      CONTAINER,
      IMAGE,
      'sh',
      '-c',
      'while true; do echo hello-from-e2e; sleep 1; done',
    );
  });
  after(() => {
    tryDocker('rm', '--force', CONTAINER);
    tryDocker('compose', '--project-name', STACK, 'down', '--remove-orphans');
  });

  it('starts and connects to the local Engine', async () => {
    await shown(text('Found on this machine'), 60_000);
    await press(button('Add'));
    await shown(text('Connected'));
  });

  it('lists a container, streams its logs and stops it', async () => {
    await goTo('Containers');
    await shown(link(CONTAINER));
    await press(link(CONTAINER));
    await shown(text('hello-from-e2e'));
    await press(button('Stop'));
    await eventually(() => !isRunning(CONTAINER), `${CONTAINER} is still running`);
    await expect(text('Exited')).toBeDisplayed();
  });

  it('pulls an image', async () => {
    tryDocker('rmi', PULLED);
    await goTo('Images');
    await press(button('Pull image'));
    await shown(dialog());
    await field('Image reference').setValue(PULLED);
    await press(button('Pull'));
    await eventually(
      () => tryDocker('image', 'inspect', '--format', '{{.Id}}', PULLED) !== null,
      `${PULLED} wasn't pulled`,
      120_000,
    );
    await shown(text('Done.'));
    // Toasts keep their layout under the CSP: Sonner's stylesheet is bundled.
    await shown(text('Pulled '));
    expect(
      await browser.execute(
        () => getComputedStyle(document.querySelector('[data-sonner-toaster]')!).position,
      ),
    ).toBe('fixed');
    await press(button('Close'));
    await gone(dialog());
    await expect(text('busybox')).toBeDisplayed();
  });

  it('deploys a Managed Stack with a sealed environment and shows it running', async () => {
    await goTo('Stacks');
    await press(button('New stack'));
    await press(text('A services: section to fill in yourself.'));
    await shown(field('Name'));
    // The editor's own styles apply under the built app's CSP (styleNonce).
    expect(
      await browser.execute(() => getComputedStyle(document.querySelector('.cm-editor')!).display),
    ).toBe('flex');
    await field('Name').setValue(STACK);
    await fillEditor(
      [
        'services:',
        '  web:',
        `    image: ${IMAGE}`,
        '    command: ["sleep", "600"]',
        '    environment:',
        '      GREETING: ${GREETING}',
        '',
      ].join('\n'),
    );
    // Stored sealed under the keychain's key, and opened again to deploy.
    await field('Environment').setValue(`GREETING=${GREETING}`);
    await press(button('Save and deploy'));
    await eventually(
      () => stackEnv().includes(`GREETING=${GREETING}`),
      `${STACK} isn't deployed with its environment`,
      120_000,
    );
    await press(button('Close'));
    await shown(text('Running'));
  });

  it('keeps its Hosts, Stacks and secrets across a restart', async () => {
    // A new session is a new launch of the app, on the same data folder.
    await browser.reloadSession();
    await shown(text('Connected'), 60_000);
    await goTo('Stacks');
    await shown(text(STACK));
    await shown(text('Running'));

    await press(button('More stack actions'));
    await press(menuItem('Take down'));
    await press(dialogButton('Take down'));
    await eventually(() => stackEnv() === '', `${STACK} still has a container`, 120_000);
    await press(dialogButton('Close'));
    await shown(text('Not deployed'));

    // Deploying again needs the environment, sealed before the restart.
    await press(button('Deploy'));
    await eventually(
      () => stackEnv().includes(`GREETING=${GREETING}`),
      `${STACK} isn't redeployed with its environment`,
      120_000,
    );
    await press(dialogButton('Close'));
    await shown(text('Running'));
  });
});
