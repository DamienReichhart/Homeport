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
