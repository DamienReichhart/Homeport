import { useInfiniteQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useState } from 'react';
import { hubTagsQuery } from '@/features/registries';
import { useDebounced } from '@/shared/hooks';
import { formatBytes, formatRelative } from '@/shared/lib';
import { Button, ErrorMessage, SearchInput, Spinner } from '@/shared/ui';

/** A Docker Hub repository's tags, newest first, to pick one to pull. */
export function HubTags({
  repository,
  onChoose,
  onBack,
}: {
  repository: string;
  onChoose: (tag: string) => void;
  onBack: () => void;
}) {
  const [filter, setFilter] = useState('');
  const debounced = useDebounced(filter.trim(), 300);
  const tags = useInfiniteQuery(hubTagsQuery(repository, debounced));
  const list = tags.data?.pages.flatMap((p) => p.tags) ?? [];

  return (
    <div className="grid gap-2">
      <div className="flex items-center gap-2">
        <Button type="button" variant="ghost" size="sm" onClick={onBack}>
          <ArrowLeftIcon /> Search
        </Button>
        <span className="text-sm font-medium">{repository}</span>
        {tags.data && (
          <span className="text-muted-foreground text-xs">{tags.data.pages[0]?.count} tags</span>
        )}
      </div>
      <SearchInput value={filter} onChange={setFilter} placeholder="Filter tags" />
      {tags.isPending && <Spinner />}
      {tags.error && <ErrorMessage error={tags.error} onRetry={() => void tags.refetch()} />}
      {list.length > 0 && (
        <ul className="divide-border grid max-h-64 divide-y overflow-y-auto" aria-label="Tags">
          {list.map((t) => (
            <li key={t.name}>
              <button
                type="button"
                className="hover:bg-muted flex w-full items-center gap-3 px-2 py-1.5 text-left"
                aria-label={t.name}
                onClick={() => onChoose(t.name)}
              >
                <span className="min-w-0 flex-1 truncate font-mono text-xs">{t.name}</span>
                {t.size !== null && (
                  <span className="text-muted-foreground text-xs">{formatBytes(t.size)}</span>
                )}
                {t.lastPushed && (
                  <span className="text-muted-foreground w-24 text-right text-xs">
                    {formatRelative(Date.parse(t.lastPushed) / 1000)}
                  </span>
                )}
              </button>
            </li>
          ))}
        </ul>
      )}
      {tags.data && list.length === 0 && (
        <p className="text-muted-foreground text-xs">No tag matches “{debounced}”.</p>
      )}
      {tags.hasNextPage && (
        <Button
          type="button"
          variant="outline"
          size="sm"
          className="justify-self-start"
          disabled={tags.isFetchingNextPage}
          onClick={() => void tags.fetchNextPage()}
        >
          {tags.isFetchingNextPage ? 'Loading…' : 'More tags'}
        </Button>
      )}
    </div>
  );
}
