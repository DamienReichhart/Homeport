import type { NetworkInput } from '@/shared/api';

/** A new network with the Engine's defaults. */
export const emptyNetwork: NetworkInput = {
  name: '',
  driver: null,
  subnet: null,
  gateway: null,
  internal: false,
  attachable: false,
};

/** Whether the form can be sent: the Engine needs a name, everything else has a default. */
export const canCreateNetwork = (input: NetworkInput) => input.name.trim() !== '';

/**
 * The request the form sends: text trimmed, and blank optional fields left
 * to the Engine (`null`) rather than sent as empty strings it would reject.
 */
export function toNetworkInput(input: NetworkInput): NetworkInput {
  const optional = (value: string | null) => value?.trim() || null;
  return {
    ...input,
    name: input.name.trim(),
    driver: optional(input.driver),
    subnet: optional(input.subnet),
    gateway: optional(input.gateway),
  };
}
