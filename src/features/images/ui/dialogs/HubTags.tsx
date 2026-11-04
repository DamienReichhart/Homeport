import { useInfiniteQuery } from '@tanstack/react-query';
import { ArrowLeftIcon } from 'lucide-react';
import { useState } from 'react';
import { hubTagsQuery } from '@/features/registries';
import { useDebounced } from '@/shared/hooks';
import { formatBytes, formatRelative } from '@/shared/lib';
import { Button, ErrorMessage, SearchInput, Spinner } from '@/shared/ui';
import { t } from '@/shared/i18n';

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
          <ArrowLeftIcon /> {t('images.hub.backToSearch')}
        </Button>
        <span className="text-sm font-medium">{repository}</span>
        {tags.data && (
          <span className="text-muted-foreground text-xs">
            {t('images.hub.tagCount', { count: tags.data.pages[0]?.count ?? 0 })}
          </span>
        )}
      </div>
      <SearchInput value={filter} onChange={setFilter} placeholder={t('images.hub.filterTags')} />
      {tags.isPending && <Spinner />}
      {tags.error && <ErrorMessage error={tags.error} onRetry={() => void tags.refetch()} />}
      {list.length > 0 && (
        <ul
          className="divide-border grid max-h-64 divide-y overflow-y-auto"
          aria-label={t('images.hub.tags')}
        >
          {list.map((tag) => (
            <li key={tag.name}>
              <button
                type="button"
                className="hover:bg-muted flex w-full items-center gap-3 px-2 py-1.5 text-left"
                aria-label={tag.name}
                onClick={() => onChoose(tag.name)}
              >
                <span className="min-w-0 flex-1 truncate font-mono text-xs">{tag.name}</span>
                {tag.size !== null && (
                  <span className="text-muted-foreground text-xs">{formatBytes(tag.size)}</span>
                )}
                {tag.lastPushed && (
                  <span className="text-muted-foreground w-24 text-right text-xs">
                    {formatRelative(Date.parse(tag.lastPushed) / 1000)}
                  </span>
                )}
              </button>
            </li>
          ))}
        </ul>
      )}
      {tags.data && list.length === 0 && (
        <p className="text-muted-foreground text-xs">
          {t('images.hub.noTagMatch', { filter: debounced })}
        </p>
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
          {tags.isFetchingNextPage ? t('common.loading') : t('images.hub.moreTags')}
        </Button>
      )}
    </div>
  );
}
