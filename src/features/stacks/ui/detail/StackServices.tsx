import { Link } from 'react-router-dom';
import { useQuery } from '@tanstack/react-query';
import {
  containerFreshness,
  indexFreshness,
  NewerImage,
  newerLabel,
  resourceQueries,
  useContainers,
} from '@/core/host';
import { containerTone } from '@/features/containers';
import type { Stack } from '@/shared/api';
import { serviceFreshness, stackFreshness } from '../../model/freshness';
import { t } from '@/shared/i18n';
import {
  StatusDot,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';

/** Each service of a Stack with its containers and their status. */
export function StackServices({ hostId, stack }: { hostId: string; stack: Stack }) {
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
      <div className="rounded-md border">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t('stacks.services.column.service')}</TableHead>
              <TableHead>{t('stacks.services.column.container')}</TableHead>
              <TableHead>{t('stacks.services.column.image')}</TableHead>
              <TableHead>{t('stacks.services.column.status')}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {stack.services.flatMap((s) =>
              s.containerIds.map((id) => {
                const c = byId.get(id);
                return (
                  <TableRow key={id}>
                    <TableCell className="font-medium">{s.name}</TableCell>
                    <TableCell>
                      <Link to={`/containers/${id}/logs`} className="text-primary hover:underline">
                        {c?.name ?? id.slice(0, 12)}
                      </Link>
                    </TableCell>
                    <TableCell>
                      <span className="flex items-center gap-1.5">
                        {c?.image}
                        {c && containerFreshness(freshness, c) === 'newer' && (
                          <NewerImage label={newerLabel([c.image])} />
                        )}
                      </span>
                    </TableCell>
                    <TableCell>
                      <span className="flex items-center gap-1.5">
                        {c && <StatusDot tone={containerTone(c.state, c.health)} />}
                        {c?.status}
                      </span>
                    </TableCell>
                  </TableRow>
                );
              }),
            )}
          </TableBody>
        </Table>
      </div>
    </div>
  );
}
