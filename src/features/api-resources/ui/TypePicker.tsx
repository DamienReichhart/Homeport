import { LockIcon } from 'lucide-react';
import { useState } from 'react';
import { Link } from 'react-router-dom';
import type { ApiResourceInfo } from '@/shared/api';
import { cn } from '@/shared/lib';
import { SearchInput } from '@/shared/ui';
import { byGroup, matches, notListable, typePath } from '../model/types';
import { t } from '@/shared/i18n';

/** Every type the Cluster serves, grouped by API group, searchable by kind or short name. */
export function TypePicker({
  types,
  current,
}: {
  types: ApiResourceInfo[];
  current: ApiResourceInfo | undefined;
}) {
  const [query, setQuery] = useState('');
  const groups = byGroup(types.filter((x) => matches(x, query)));
  return (
    <nav aria-label={t('apiresources.types')} className="grid content-start gap-3">
      <SearchInput value={query} onChange={setQuery} placeholder={t('apiresources.search')} />
      {groups.length === 0 && (
        <p className="text-muted-foreground text-sm">{t('apiresources.noMatch')}</p>
      )}
      <div className="grid max-h-[70vh] gap-3 overflow-y-auto pr-1">
        {groups.map(({ group, types: list }) => (
          <section key={group} className="grid gap-0.5">
            <h2 className="text-muted-foreground px-2 text-xs font-medium">
              {group || t('apiresources.coreGroup')}
            </h2>
            {list.map((info) => {
              const blocked = notListable(info);
              const selected =
                current?.api.group === info.api.group && current.api.plural === info.api.plural;
              return (
                <Link
                  key={`${info.api.group}/${info.api.plural}`}
                  to={typePath(info.api)}
                  aria-current={selected ? 'page' : undefined}
                  className={cn(
                    'hover:bg-accent flex items-center gap-2 rounded-md px-2 py-1 text-sm',
                    selected && 'bg-accent font-medium',
                    blocked && 'text-muted-foreground',
                  )}
                >
                  <span className="truncate">{info.api.kind}</span>
                  {info.shortNames.length > 0 && (
                    <span className="text-muted-foreground font-mono text-xs">
                      {info.shortNames.join(', ')}
                    </span>
                  )}
                  {blocked && (
                    <LockIcon
                      className="ml-auto size-3"
                      aria-label={
                        blocked === 'verb'
                          ? t('apiresources.cantList')
                          : t('apiresources.notAllowed')
                      }
                    />
                  )}
                </Link>
              );
            })}
          </section>
        ))}
      </div>
    </nav>
  );
}
