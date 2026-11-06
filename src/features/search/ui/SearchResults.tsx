import {
  BoxesIcon,
  BoxIcon,
  HardDriveIcon,
  LayersIcon,
  NetworkIcon,
  type LucideIcon,
} from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useHostSelection } from '@/core/host';
import { CommandGroup, CommandItem } from '@/shared/ui';
import { useResourceSearch } from '../api/useResourceSearch';
import { hitPath, KINDS, MIN_QUERY, type ResourceKind, type SearchHit } from '../model/search';
import { t, type MessageKey } from '@/shared/i18n';

/** At most this many results per kind until **Show all**. */
const PER_KIND = 5;

const KIND: Record<ResourceKind, { heading: MessageKey; icon: LucideIcon }> = {
  container: { heading: 'search.kind.container', icon: BoxIcon },
  stack: { heading: 'search.kind.stack', icon: BoxesIcon },
  image: { heading: 'search.kind.image', icon: LayersIcon },
  volume: { heading: 'search.kind.volume', icon: HardDriveIcon },
  network: { heading: 'search.kind.network', icon: NetworkIcon },
};

/**
 * The command palette's results from every connected Host, grouped by kind.
 * Choosing one selects its Host and opens it; `onChosen` closes the palette.
 */
export function SearchResults({
  query,
  active,
  onChosen,
}: {
  query: string;
  active: boolean;
  onChosen: () => void;
}) {
  const { hits, skipped } = useResourceSearch(query, active);
  const [expanded, setExpanded] = useState(new Set<ResourceKind>());
  const navigate = useNavigate();
  const { select } = useHostSelection();

  const open = (hit: SearchHit) => {
    onChosen();
    select(hit.hostId);
    void navigate(hitPath(hit));
  };
  return (
    <>
      {KINDS.map((kind) => {
        const all = hits.filter((h) => h.kind === kind);
        if (all.length === 0) return null;
        const shown = expanded.has(kind) ? all : all.slice(0, PER_KIND);
        const Icon = KIND[kind].icon;
        return (
          <CommandGroup key={kind} heading={t(KIND[kind].heading)}>
            {shown.map((hit) => (
              <CommandItem
                key={`${hit.hostId}/${hit.id}`}
                value={`search:${kind}:${hit.hostId}:${hit.id}`}
                onSelect={() => open(hit)}
              >
                <Icon />
                <span className="truncate">{hit.label}</span>
                <span className="text-muted-foreground ml-auto shrink-0 text-xs">
                  {hit.state
                    ? t('search.onHostState', { host: hit.hostName, state: hit.state })
                    : hit.hostName}
                </span>
              </CommandItem>
            ))}
            {shown.length < all.length && (
              <CommandItem
                value={`search:${kind}:all`}
                onSelect={() => setExpanded((was) => new Set(was).add(kind))}
                className="text-muted-foreground"
              >
                {t('search.showAll', { count: all.length })}
              </CommandItem>
            )}
          </CommandGroup>
        );
      })}
      {skipped > 0 && query.trim().length >= MIN_QUERY && (
        <p className="text-muted-foreground px-3 py-2 text-xs">
          {t('search.skipped', { count: skipped })}
        </p>
      )}
    </>
  );
}
