import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { InstanceType, RemoteImage } from '@/shared/api';
import { formatBytes } from '@/shared/lib';
import { cn } from '@/shared/lib';
import { ErrorMessage, Input, SearchInput, Spinner } from '@/shared/ui';
import { incusImageQueries } from '../api/queries';
import { filterRemote, remoteLabel } from '../model/remote';
import { t } from '@/shared/i18n';

/** How many matches the list shows; the search narrows the rest. */
const SHOWN = 60;

/**
 * Picks an image from a remote server's catalogue, filtered to those that
 * come as `type`. The alias can also be typed, for a server Homeport can't
 * read or an image it doesn't list.
 */
export function RemoteImagePicker({
  hostId,
  server,
  type,
  alias,
  onAlias,
}: {
  hostId: string;
  server: string;
  type: InstanceType;
  alias: string;
  onAlias: (alias: string) => void;
}) {
  const catalogue = useQuery({ ...incusImageQueries.remote(hostId, server), enabled: !!server });
  const [query, setQuery] = useState('');
  const matches = catalogue.data ? filterRemote(catalogue.data, query, type) : [];
  const size = (image: RemoteImage) =>
    type === 'container' ? image.containerSize : image.virtualMachineSize;
  return (
    <div className="grid gap-2">
      <SearchInput value={query} onChange={setQuery} placeholder={t('incus.remote.search')} />
      {catalogue.isPending && server ? (
        <Spinner />
      ) : catalogue.error ? (
        <ErrorMessage error={catalogue.error} onRetry={() => void catalogue.refetch()} />
      ) : (
        <ul
          className="max-h-56 overflow-auto rounded-md border"
          aria-label={t('incus.remote.list')}
        >
          {matches.length === 0 && (
            <li className="text-muted-foreground p-3 text-sm">{t('incus.remote.none')}</li>
          )}
          {matches.slice(0, SHOWN).map((image) => {
            const bytes = size(image);
            return (
              <li key={`${image.alias}/${image.variant}`}>
                <button
                  type="button"
                  aria-pressed={alias === image.alias}
                  onClick={() => onAlias(image.alias)}
                  className={cn(
                    'hover:bg-muted flex w-full items-center justify-between gap-3 px-3 py-1.5 text-left text-sm',
                    alias === image.alias && 'bg-accent text-accent-foreground',
                  )}
                >
                  <span className="min-w-0">
                    <span className="block truncate">{remoteLabel(image)}</span>
                    <span className="text-muted-foreground font-mono text-xs">{image.alias}</span>
                  </span>
                  {bytes !== null && (
                    <span className="text-muted-foreground shrink-0 text-xs">
                      {formatBytes(bytes)}
                    </span>
                  )}
                </button>
              </li>
            );
          })}
        </ul>
      )}
      <Input
        aria-label={t('incus.remote.alias')}
        value={alias}
        onChange={(e) => onAlias(e.target.value)}
        placeholder="debian/12"
        className="font-mono"
      />
    </div>
  );
}
