import { useQuery } from '@tanstack/react-query';
import { BadgeCheckIcon, StarIcon } from 'lucide-react';
import { toAppError } from '@/shared/api';
import { useDebounced } from '@/shared/hooks';
import { Spinner } from '@/shared/ui';
import { imageQueries } from '../../api/queries';
import { shortRepository } from '../../model/hub';
import { t } from '@/shared/i18n';

/** Docker Hub repositories matching what's typed, searched from the Host. */
export function HubSearch({
  hostId,
  term,
  onChoose,
}: {
  hostId: string;
  term: string;
  onChoose: (repository: string) => void;
}) {
  const debounced = useDebounced(term, 300);
  const results = useQuery(imageQueries.search(hostId, debounced));
  if (results.isFetching && !results.data) return <Spinner />;
  if (results.error) {
    return (
      <p className="text-muted-foreground text-xs">
        {t('images.hub.unavailable', { error: toAppError(results.error).message })}
      </p>
    );
  }
  if (!results.data) return null;
  if (results.data.length === 0) {
    return (
      <p className="text-muted-foreground text-xs">
        {t('images.hub.noMatch', { term: debounced })}
      </p>
    );
  }
  return (
    <ul
      className="divide-border grid max-h-64 divide-y overflow-y-auto"
      aria-label={t('images.hub.results')}
    >
      {results.data.map((r) => (
        <li key={r.name}>
          <button
            type="button"
            className="hover:bg-muted grid w-full gap-0.5 px-2 py-1.5 text-left"
            aria-label={t(r.official ? 'images.hub.resultOfficial' : 'images.hub.result', {
              name: shortRepository(r.name),
              stars: r.stars.toLocaleString(),
            })}
            onClick={() => onChoose(shortRepository(r.name))}
          >
            <span className="flex items-center gap-1.5 text-sm font-medium">
              {shortRepository(r.name)}
              {r.official && (
                <span className="text-muted-foreground inline-flex items-center gap-0.5 text-xs font-normal">
                  <BadgeCheckIcon className="size-3.5" /> {t('images.hub.official')}
                </span>
              )}
              <span className="text-muted-foreground ml-auto inline-flex items-center gap-0.5 text-xs font-normal">
                <StarIcon className="size-3" /> {r.stars.toLocaleString()}
              </span>
            </span>
            {r.description && (
              <span className="text-muted-foreground truncate text-xs">{r.description}</span>
            )}
          </button>
        </li>
      ))}
    </ul>
  );
}
