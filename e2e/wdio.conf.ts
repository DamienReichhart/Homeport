/**
 * WebdriverIO drives the real app (Rust backend, real webview, real IPC)
 * through tauri-driver, which speaks WebDriver to WebKitGTK. `run.sh` sets up
 * the display, the keyring and a fresh data folder, then starts this.
 */
import { spawn, type ChildProcess } from 'node:child_process';
import { cpSync, existsSync, mkdirSync, rmSync } from 'node:fs';
import { homedir } from 'node:os';
import { join } from 'node:path';

const HERE = import.meta.dirname;
const APP = process.env.HOMEPORT_APP ?? join(HERE, '..', 'target', 'debug', 'homeport');
const DRIVER = process.env.TAURI_DRIVER ?? join(homedir(), '.cargo', 'bin', 'tauri-driver');
const ARTIFACTS = join(HERE, 'artifacts');
const PORT = 4444;

let driver: ChildProcess | undefined;

/** Resolves once tauri-driver answers on its port. */
async function driverReady(): Promise<void> {
  for (let attempt = 0; attempt < 100; attempt++) {
    try {
      await fetch(`http://127.0.0.1:${PORT}/status`);
      return;
    } catch {
      await new Promise((resolve) => setTimeout(resolve, 100));
    }
  }
  throw new Error(`tauri-driver didn't start on port ${PORT}`);
}

/** The app's log folder of this run, for the artifacts of a failure. */
const logDir = () => join(process.env.XDG_DATA_HOME ?? '', 'com.homeport.app', 'logs');

export const config: WebdriverIO.Config = {
  runner: 'local',
  hostname: '127.0.0.1',
  port: PORT,
  specs: ['./specs/**/*.e2e.ts'],
  // One app at a time: the specs share the Engine and the data folder.
  maxInstances: 1,
  capabilities: [
    {
      // @ts-expect-error tauri-driver's own capability
      'tauri:options': { application: APP },
      // WebKitWebDriver speaks classic WebDriver only, not BiDi.
      'wdio:enforceWebDriverClassic': true,
    },
  ],
  logLevel: 'warn',
  bail: 0,
  waitforTimeout: 20_000,
  connectionRetryTimeout: 120_000,
  connectionRetryCount: 1,
  framework: 'mocha',
  reporters: ['spec'],
  mochaOpts: { ui: 'bdd', timeout: 180_000 },

  // One driver for the run. It starts the app for each session and stops it
  // with the session, so a new session is a fresh launch on the same data.
  async onPrepare() {
    if (!existsSync(APP)) {
      throw new Error(`${APP} isn't built: run "bun tauri build --debug --no-bundle" first`);
    }
    rmSync(ARTIFACTS, { recursive: true, force: true });
    mkdirSync(ARTIFACTS, { recursive: true });
    driver = spawn(DRIVER, ['--port', String(PORT)], { stdio: ['ignore', 'inherit', 'inherit'] });
    driver.on('error', (e) => {
      throw new Error(`tauri-driver didn't start (${DRIVER}): ${e.message}`);
    });
    process.on('exit', () => driver?.kill());
    await driverReady();
  },

  async afterTest(test, _context, { passed }) {
    if (passed) return;
    const name = `${test.parent} ${test.title}`.replace(/[^a-z0-9]+/gi, '-').toLowerCase();
    await browser.saveScreenshot(join(ARTIFACTS, `${name}.png`)).catch(() => undefined);
  },

  onComplete() {
    driver?.kill();
    // The app's own log, next to the screenshots.
    if (existsSync(logDir())) cpSync(logDir(), join(ARTIFACTS, 'logs'), { recursive: true });
  },
};
