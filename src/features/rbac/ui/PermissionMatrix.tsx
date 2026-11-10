import { CheckCheckIcon, CheckIcon } from 'lucide-react';
import type { Grants } from '@/shared/api';
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/shared/ui';
import { matrixOf, urlRows, type Cell } from '../model/matrix';
import { t } from '@/shared/i18n';

function Mark({ cell }: { cell: Cell }) {
  if (cell === 'yes')
    return <CheckIcon className="text-primary mx-auto size-4" aria-label={t('rbac.matrix.yes')} />;
  if (cell === 'all')
    return (
      <CheckCheckIcon
        className="text-primary/70 mx-auto size-4"
        aria-label={t('rbac.matrix.all')}
      />
    );
  return null;
}

/**
 * What grants allow, as a resource × verb matrix, and the non-resource URLs
 * after it. `via` names the bindings behind a row, shown on hover.
 */
export function PermissionMatrix({
  grants,
  via,
}: {
  grants: Grants;
  via?: (bindings: number[]) => string;
}) {
  const matrix = matrixOf(grants.resources);
  const urls = urlRows(grants.urls);
  if (!matrix.rows.length && !urls.length)
    return <p className="text-muted-foreground text-sm">{t('rbac.matrix.empty')}</p>;
  return (
    <div className="grid gap-4">
      {matrix.rows.length > 0 && (
        <div className="overflow-x-auto">
          <Table>
            <TableHeader>
              <TableRow>
                <TableHead>{t('rbac.matrix.resource')}</TableHead>
                <TableHead>{t('rbac.matrix.names')}</TableHead>
                {matrix.columns.map((verb) => (
                  <TableHead key={verb} className="text-center font-mono text-xs">
                    {verb}
                  </TableHead>
                ))}
              </TableRow>
            </TableHeader>
            <TableBody>
              {matrix.rows.map((row) => (
                <TableRow key={row.id} title={via?.(row.bindings)}>
                  <TableCell className="font-mono text-xs">{row.resource}</TableCell>
                  <TableCell className="font-mono text-xs">{row.names.join(', ')}</TableCell>
                  {row.cells.map((cell, i) => (
                    <TableCell key={matrix.columns[i]} className="text-center">
                      <Mark cell={cell} />
                    </TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </div>
      )}
      {urls.length > 0 && (
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t('rbac.matrix.url')}</TableHead>
              <TableHead>{t('rbac.matrix.verbs')}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {urls.map((u) => (
              <TableRow key={u.url} title={via?.(u.bindings)}>
                <TableCell className="font-mono text-xs">{u.url}</TableCell>
                <TableCell className="font-mono text-xs">{u.verbs.join(', ')}</TableCell>
              </TableRow>
            ))}
          </TableBody>
        </Table>
      )}
    </div>
  );
}
