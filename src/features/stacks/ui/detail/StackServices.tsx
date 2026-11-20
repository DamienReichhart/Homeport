import { Link, useNavigate } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import {
  containerFreshness,
  indexFreshness,
  NewerImage,
  newerLabel,
  resourceQueries,
  useContainers,
} from '@/core/host';
import { containerTone, statusText } from '@/features/containers';
import type { Stack } from '@/shared/api';
import { serviceFreshness, stackFreshness } from '../../model/freshness';
import { t } from '@/shared/i18n';
import {
  Badge,
  StatusDot,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';

/**
 * Each service of a Stack with the state of its containers, one row per
 * container; a container's row opens its page.
 */
export function StackServices({ hostId, stack }: { hostId: string; stack: Stack }) {
  const navigate = useNavigate();
  const containers = useContainers(hostId);
  const freshness = indexFreshness(useQuery(resourceQueries.freshness(hostId)).data);
  const byId = new Map(containers.data?.map((c) => [c.id, c]));
  if (stack.services.length === 0)
    return <p className="text-muted-foreground text-sm">{t('stacks.services.notDeployed')}</p>;
  const { newer } = stackFreshness(serviceFreshness(stack, byId, freshness));
  return (
    <div className="grid gap-3">
      {newer.length > 0 && (
        <p className="text-sm">
          {t('stacks.services.outdated', {
            count: newer.length,
            services: newer.map((s) => s.service).join(', '),
          })}{' '}
          <span className="text-muted-foreground">
            {stack.kind === 'external'
              ? t('stacks.services.adoptToUpdate')
              : t('stacks.services.howToUpdate')}
          </span>
        </p>
      )}
      <div className="bg-card rounded-md border">
        <Table className="table-fixed">
          <TableHeader>
            <TableRow>
              <TableHead className="w-[18%]">{t('stacks.services.column.service')}</TableHead>
              <TableHead className="w-[26%]">{t('stacks.services.column.container')}</TableHead>
              <TableHead>{t('stacks.services.column.image')}</TableHead>
              <TableHead className="w-[24%]">{t('stacks.services.column.status')}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {stack.services.flatMap((s) =>
              s.containerIds.length === 0 ? (
                <TableRow key={`service:${s.name}`}>
                  <TableCell className="truncate font-medium">{s.name}</TableCell>
                  <TableCell className="text-muted-foreground">—</TableCell>
                  <TableCell />
                  <TableCell>
                    <span className="text-muted-foreground flex items-center gap-1.5 text-sm">
                      <StatusDot tone="muted" /> {t('stacks.services.noContainer')}
                    </span>
                  </TableCell>
                </TableRow>
              ) : (
                s.containerIds.map((id, i) => {
                  const c = byId.get(id);
                  const to = `/containers/${id}/logs`;
                  return (
                    <TableRow key={id} className="cursor-pointer" onClick={() => navigate(to)}>
                      {/* The service is named once, on its first container. */}
                      <TableCell className="truncate font-medium" title={s.name}>
                        {i === 0 ? s.name : ''}
                      </TableCell>
                      <TableCell>
                        <span className="flex min-w-0 items-center gap-2">
                          <StatusDot tone={c ? containerTone(c.state, c.health) : 'muted'} />
                          <Link
                            to={to}
                            className="focus-visible:ring-ring/50 truncate rounded-sm font-mono text-xs outline-none hover:underline focus-visible:ring-[3px]"
                            title={c?.name}
                            onClick={(e) => e.stopPropagation()}
                          >
                            {c?.name ?? id.slice(0, 12)}
                          </Link>
                        </span>
                      </TableCell>
                      <TableCell>
                        <span className="flex min-w-0 items-center gap-1.5">
                          <span className="truncate font-mono text-xs" title={c?.image}>
                            {c?.image}
                          </span>
                          {c && containerFreshness(freshness, c) === 'newer' && (
                            <NewerImage label={newerLabel([c.image])} />
                          )}
                        </span>
                      </TableCell>
                      <TableCell>
                        {c && (
                          <span className="flex min-w-0 items-center gap-1.5 text-sm">
                            <span className="text-muted-foreground truncate">{statusText(c)}</span>
                            {c.health && (
                              <Badge variant={c.health === 'unhealthy' ? 'destructive' : 'outline'}>
                                {c.health}
                              </Badge>
                            )}
                          </span>
                        )}
                      </TableCell>
                    </TableRow>
                  );
                })
              ),
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
