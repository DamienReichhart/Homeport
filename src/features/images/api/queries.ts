import { queryOptions } from '@tanstack/react-query';
import { hostKeys } from '@/core/host';
import { commands } from '@/shared/api';

export const imageQueries = {
  /** The layers an image was built from. */
  history: (hostId: string, imageId: string) =>
    queryOptions({
      queryKey: [...hostKeys.images(hostId), imageId, 'history'],
      queryFn: () => commands.imagesHistory(hostId, imageId),
    }),
  inspect: (hostId: string, imageId: string) =>
    queryOptions({
      queryKey: [...hostKeys.images(hostId), imageId, 'inspect'],
      queryFn: () => commands.imagesInspect(hostId, imageId),
    }),
};
