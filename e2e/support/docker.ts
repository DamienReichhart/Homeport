/** The Docker CLI, for what a test sets up and checks outside the app. */
import { execFileSync } from 'node:child_process';

/** Runs `docker` and returns what it printed; throws when it fails. */
export function docker(...args: string[]): string {
  return execFileSync('docker', args, {
    encoding: 'utf8',
    stdio: ['ignore', 'pipe', 'pipe'],
  }).trim();
}

/** Like {@link docker}, but a failure is an answer (`null`), not an error. */
export function tryDocker(...args: string[]): string | null {
  try {
    return docker(...args);
  } catch {
    return null;
  }
}

/** Every container, running or not, whose name starts with `prefix`. */
export function containersNamed(prefix: string): string[] {
  return docker('ps', '--all', '--filter', `name=^${prefix}`, '--format', '{{.Names}}')
    .split('\n')
    .filter(Boolean);
}

/** Whether the container exists and is running. */
export function isRunning(name: string): boolean {
  return tryDocker('inspect', '--format', '{{.State.Running}}', name) === 'true';
}
