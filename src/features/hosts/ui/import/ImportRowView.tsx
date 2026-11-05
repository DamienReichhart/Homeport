import { describeAddress } from '@/core/host';
import { toAppError } from '@/shared/api';
import { Checkbox, Input, TableCell, TableRow } from '@/shared/ui';
import { authSummary, canPick, rowProblem, type ImportRow } from '../../model/importRows';
import { t } from '@/shared/i18n';

/** One draft of the import preview: pick it, rename it, and see what to expect. */
export function ImportRowView({
  row,
  rows,
  onChange,
}: {
  row: ImportRow;
  rows: ImportRow[];
  onChange: (row: ImportRow) => void;
}) {
  const { draft } = row;
  const c = draft.connection;
  const auth = authSummary(draft);
  const problem = rowProblem(row, rows);
  return (
    <TableRow className="align-top" aria-label={draft.source}>
      <TableCell>
        <Checkbox
          checked={row.selected}
          disabled={!canPick(row)}
          onCheckedChange={(v) => onChange({ ...row, selected: v === true })}
          aria-label={t('hosts.import.pick', { name: draft.source })}
        />
      </TableCell>
      <TableCell className="grid min-w-40 gap-1.5 whitespace-normal">
        <Input
          value={row.name}
          disabled={!canPick(row)}
          onChange={(e) => onChange({ ...row, name: e.target.value })}
          aria-label={t('hosts.import.name', { name: draft.source })}
        />
        {draft.needsPassphrase && row.selected && (
          <Input
            type="password"
            value={row.passphrase}
            placeholder={t('hosts.ssh.passphrase')}
            onChange={(e) => onChange({ ...row, passphrase: e.target.value })}
            aria-label={t('hosts.import.passphrase', { name: draft.source })}
          />
        )}
      </TableCell>
      <TableCell className="min-w-48 whitespace-normal">
        {c ? (
          <div className="grid gap-1.5">
            <span className="font-mono text-xs break-all">{describeAddress(c)}</span>
            {c.kind === 'ssh' && c.bastions.length > 0 && (
              <span className="text-muted-foreground text-xs">
                {t('hosts.import.via', { bastions: c.bastions.map((b) => b.host).join(' → ') })}
              </span>
            )}
            {c.kind === 'ssh' && (
              <Input
                value={row.socketPath}
                disabled={!canPick(row)}
                onChange={(e) => onChange({ ...row, socketPath: e.target.value })}
                className="h-7 font-mono text-xs"
                aria-label={t('hosts.import.socket', { name: draft.source })}
              />
            )}
          </div>
        ) : (
          <span className="text-muted-foreground">—</span>
        )}
      </TableCell>
      <TableCell className="text-xs whitespace-normal">
        {auth && t(auth)}
        {draft.files.map((file) => (
          <div key={file} className="text-muted-foreground font-mono break-all">
            {file}
          </div>
        ))}
      </TableCell>
      <TableCell className="max-w-64 text-xs whitespace-normal">
        {draft.problem && <p className="text-destructive">{toAppError(draft.problem).message}</p>}
        {draft.nameTaken && <p className="text-warning">{t('hosts.import.nameTaken')}</p>}
        {draft.warnings.map((w, i) => (
          <p key={i} className="text-warning">
            {toAppError(w).message}
          </p>
        ))}
        {problem && <p className="text-destructive">{t(problem)}</p>}
      </TableCell>
    </TableRow>
  );
}
