import { formatBytes } from '@/shared/lib';

const BINARY: Record<string, number> = { Ki: 1024, Mi: 1024 ** 2, Gi: 1024 ** 3, Ti: 1024 ** 4 };
const DECIMAL: Record<string, number> = { k: 1e3, M: 1e6, G: 1e9, T: 1e12 };

/** A Kubernetes byte quantity (`16302048Ki`, `512M`, `76224326324`) in bytes, `null` if it isn't one. */
export function bytesOf(quantity: string): number | null {
  const m = /^(\d+(?:\.\d+)?)(Ki|Mi|Gi|Ti|k|M|G|T)?$/.exec(quantity.trim());
  if (!m) return null;
  const unit = m[2] ?? '';
  return Number(m[1]) * (BINARY[unit] ?? DECIMAL[unit] ?? 1);
}

/** A CPU quantity (`3920m`, `4`) in cores, `null` if it isn't one. */
export function coresOf(quantity: string): number | null {
  const m = /^(\d+(?:\.\d+)?)(m)?$/.exec(quantity.trim());
  if (!m) return null;
  return Number(m[1]) / (m[2] ? 1000 : 1);
}

/**
 * A Node's resource as people read it: CPUs in cores, memory and storage in
 * GB; anything else (pods, hugepages counts) as Kubernetes wrote it.
 */
export function readableQuantity(resource: string, quantity: string | null | undefined): string {
  if (!quantity) return '—';
  if (resource === 'cpu') {
    const cores = coresOf(quantity);
    return cores === null ? quantity : String(Math.round(cores * 100) / 100);
  }
  if (resource === 'memory' || resource.includes('storage') || resource.startsWith('hugepages')) {
    const bytes = bytesOf(quantity);
    return bytes === null ? quantity : formatBytes(bytes);
  }
  return quantity;
}
