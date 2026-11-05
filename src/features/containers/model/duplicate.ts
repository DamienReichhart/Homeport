import type { RunForm } from './runForm';

/** `<name>-copy`, or `-copy-2`, `-copy-3`… when that is taken. */
export function copyName(name: string, taken: readonly string[]): string {
  const used = new Set(taken);
  if (!used.has(`${name}-copy`)) return `${name}-copy`;
  let n = 2;
  while (used.has(`${name}-copy-${n}`)) n += 1;
  return `${name}-copy-${n}`;
}

/** Fixed host ports in `form` that the original container publishes too. */
export function sharedHostPorts(form: RunForm, original: RunForm): string[] {
  const published = new Set(
    original.ports
      .filter((p) => p.hostPort.trim() !== '')
      .map((p) => `${p.hostPort.trim()}/${p.protocol}`),
  );
  return form.ports
    .filter((p) => p.hostPort.trim() !== '' && published.has(`${p.hostPort.trim()}/${p.protocol}`))
    .map((p) => p.hostPort.trim());
}
