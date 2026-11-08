import type { HostCapability } from '@/core/feature';
import type { EngineKind, HostView } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** The name of each kind of Engine, as the Host form and pages show it. */
export const ENGINE_LABELS: Record<EngineKind, MessageKey> = {
  docker: 'core.engine.docker',
  containerd: 'core.engine.containerd',
  incus: 'core.engine.incus',
};

/** The Podman version a connected Host runs, `null` for any other Engine or state. */
export function podmanVersion(host: HostView | null | undefined): string | null {
  return host?.status.state === 'connected' ? host.status.podmanVersion : null;
}

/**
 * Whether `host` offers `capability`. The Docker Engine API is known from the
 * stored Engine kind; Podman's own only once the Host is connected, because
 * the same socket can change product.
 */
export function supports(host: HostView | null | undefined, capability: HostCapability): boolean {
  if (!host) return false;
  const docker = host.host.engine === 'docker';
  switch (capability) {
    case 'docker':
      return docker;
    case 'podman':
      return docker && podmanVersion(host) !== null;
  }
}

/** `version`'s leading numbers, so "5.8.1-dev" reads as [5, 8, 1]. */
function numbers(version: string): number[] {
  return (/^v?(\d+(?:\.\d+)*)/.exec(version.trim())?.[1] ?? '').split('.').map(Number);
}

/** Whether `version` is `minimum` or later, comparing their numbers in order. */
export function versionAtLeast(version: string, minimum: string): boolean {
  const have = numbers(version);
  const need = numbers(minimum);
  for (let i = 0; i < Math.max(have.length, need.length); i++) {
    const a = have[i] ?? 0;
    const b = need[i] ?? 0;
    if (a !== b) return a > b;
  }
  return true;
}

/**
 * Whether what `host` offers is known yet: its Engine kind always is, and
 * Podman once it's connected. Until then a page can't say it's unsupported.
 */
export function capabilityKnown(host: HostView, capability: HostCapability): boolean {
  return capability === 'docker' || host.status.state === 'connected';
}

/** Whether a connected Host runs Podman `minimum` or later. */
export function podmanAtLeast(host: HostView | null | undefined, minimum: string): boolean {
  const version = podmanVersion(host);
  return version !== null && versionAtLeast(version, minimum);
}
