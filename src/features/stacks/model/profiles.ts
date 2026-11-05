/**
 * The Stack's turned-on profiles its definition still declares, and those
 * that disappeared from it: they're dropped at the next save.
 */
export function splitProfiles(
  active: string[],
  declared: string[],
): { kept: string[]; dropped: string[] } {
  return {
    kept: active.filter((p) => declared.includes(p)),
    dropped: active.filter((p) => !declared.includes(p)),
  };
}

/** `profiles` with `profile` turned on or off, in the definition's order. */
export function toggle(
  declared: string[],
  profiles: string[],
  profile: string,
  on: boolean,
): string[] {
  return declared.filter((p) => (p === profile ? on : profiles.includes(p)));
}
