/**
 * Whether `value` is what a bridge's address setting takes: `auto`,
 * `none`, or an address with its prefix length.
 */
export function validAddress(value: string, family: 4 | 6): boolean {
  const v = value.trim();
  if (v === '' || v === 'auto' || v === 'none') return true;
  const [address, prefix, ...rest] = v.split('/');
  if (rest.length > 0 || !address || !prefix || !/^\d+$/.test(prefix)) return false;
  const bits = Number(prefix);
  if (family === 4) {
    const parts = address.split('.');
    return (
      bits <= 32 &&
      parts.length === 4 &&
      parts.every((p) => /^\d{1,3}$/.test(p) && Number(p) <= 255)
    );
  }
  return bits <= 128 && /^[0-9a-fA-F:]+$/.test(address) && address.includes(':');
}

/** Linux interface names: up to 15 characters, no slash, colon or space. */
export function validNetworkName(name: string): boolean {
  return /^[a-zA-Z0-9][a-zA-Z0-9_.-]{0,14}$/.test(name.trim());
}
