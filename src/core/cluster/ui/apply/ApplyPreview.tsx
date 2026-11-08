import type { ApplyDocument, ApplyResult } from '@/shared/api';
import { Badge, ErrorMessage, YamlDiff } from '@/shared/ui';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<ApplyDocument['action'], MessageKey> = {
  created: 'core.apply.action.created',
  changed: 'core.apply.action.changed',
  unchanged: 'core.apply.action.unchanged',
  failed: 'core.apply.action.failed',
};

function Document({ doc }: { doc: ApplyDocument }) {
  const where = doc.namespace ? `${doc.namespace}/` : '';
  return (
    <section className="grid gap-2 rounded-md border p-3">
      <header className="flex flex-wrap items-center gap-2">
        <Badge variant={doc.action === 'failed' ? 'destructive' : 'outline'}>
          {t(ACTIONS[doc.action])}
        </Badge>
        <span className="font-mono text-sm break-all">
          {doc.kind} {where}
          {doc.name}
        </span>
      </header>
      {doc.error && <ErrorMessage error={doc.error} />}
      {doc.conflicts.length > 0 && (
        <ul className="text-sm">
          {doc.conflicts.map((c) => (
            <li key={c.field}>
              <code className="text-xs">{c.field}</code>{' '}
              <span className="text-muted-foreground">{c.message}</span>
            </li>
          ))}
        </ul>
      )}
      {(doc.action === 'created' || doc.action === 'changed') && doc.after && (
        <YamlDiff
          before={doc.before ?? ''}
          after={doc.after}
          label={t('core.apply.diffOf', { kind: doc.kind, name: doc.name })}
        />
      )}
    </section>
  );
}

/** What applying would do, or did, to each document. */
export function ApplyPreview({ result }: { result: ApplyResult }) {
  return (
    <div className="grid gap-3">
      {result.documents.map((doc, i) => (
        <Document key={`${i}:${doc.kind}/${doc.name}`} doc={doc} />
      ))}
    </div>
  );
}
