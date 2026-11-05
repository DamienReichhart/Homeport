import type { ImportChoice, ImportDraft } from '@/shared/api';
import type { MessageKey } from '@/shared/i18n';

/** Where Hosts are imported from. */
export type ImportSource = 'sshConfig' | 'dockerContexts';

/** One draft of the preview, with what the user picked and changed. */
export interface ImportRow {
  draft: ImportDraft;
  selected: boolean;
  name: string;
  /** The Engine socket on the server, for SSH Hosts. */
  socketPath: string;
  passphrase: string;
}

/** The preview's rows: everything importable is picked, unless its name is taken. */
export function rowsFrom(drafts: ImportDraft[]): ImportRow[] {
  return drafts.map((draft) => ({
    draft,
    selected: !draft.problem && !draft.nameTaken,
    name: draft.name,
    socketPath: draft.connection?.kind === 'ssh' ? draft.connection.socketPath : '',
    passphrase: '',
  }));
}

/** Whether a row can be picked at all. */
export const canPick = (row: ImportRow) => !row.draft.problem;

/** What keeps a picked row from being imported, if anything. */
export function rowProblem(row: ImportRow, rows: ImportRow[]): MessageKey | null {
  if (!row.selected) return null;
  const name = row.name.trim().toLowerCase();
  if (!name) return 'hosts.import.problem.nameEmpty';
  const twice = rows.filter((r) => r.selected && r.name.trim().toLowerCase() === name).length > 1;
  if (twice) return 'hosts.import.problem.nameTwice';
  if (row.draft.nameTaken && row.name.trim() === row.draft.name) {
    return 'hosts.import.problem.nameTaken';
  }
  if (row.draft.needsPassphrase && !row.passphrase) return 'hosts.import.problem.passphrase';
  if (row.draft.connection?.kind === 'ssh' && !row.socketPath.trim().startsWith('/')) {
    return 'hosts.import.problem.socket';
  }
  return null;
}

/** The picked rows, ready to send, or `null` while one of them has a problem. */
export function toChoices(rows: ImportRow[]): ImportChoice[] | null {
  const picked = rows.filter((r) => r.selected && canPick(r));
  if (picked.length === 0 || picked.some((r) => rowProblem(r, rows))) return null;
  return picked.map((r) => ({
    source: r.draft.source,
    name: r.name.trim(),
    socketPath: r.draft.connection?.kind === 'ssh' ? r.socketPath.trim() : null,
    passphrase: r.draft.needsPassphrase ? r.passphrase : null,
  }));
}

/** How a draft logs in, in a few words. */
export function authSummary(draft: ImportDraft): MessageKey | null {
  const c = draft.connection;
  if (!c) return null;
  switch (c.kind) {
    case 'ssh':
      return c.auth === 'agent' ? 'hosts.import.auth.agent' : 'hosts.import.auth.key';
    case 'tls':
      return 'hosts.import.auth.certificates';
    default:
      return 'hosts.import.auth.none';
  }
}
