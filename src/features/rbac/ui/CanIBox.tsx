import { CircleCheckIcon, CircleXIcon } from 'lucide-react';
import { useId, useState, type FormEvent } from 'react';
import {
  Badge,
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  CopyButton,
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { useCanI } from '../api/queries';
import { ALL, STANDARD_VERBS } from '../model/matrix';
import { t } from '@/shared/i18n';

/** Verbs offered as you type; any other can be typed. */
const VERBS = [...STANDARD_VERBS, 'bind', 'escalate', 'impersonate', 'use', ALL];

function Field({
  label,
  value,
  onChange,
  placeholder,
  list,
}: {
  label: string;
  value: string;
  onChange: (value: string) => void;
  placeholder?: string;
  list?: string;
}) {
  const id = useId();
  return (
    <div className="grid gap-1">
      <Label htmlFor={id}>{label}</Label>
      <Input
        id={id}
        value={value}
        list={list}
        placeholder={placeholder}
        className="font-mono"
        spellCheck={false}
        autoCapitalize="off"
        onChange={(e) => onChange(e.target.value)}
      />
    </div>
  );
}

const optional = (value: string) => value.trim() || null;

/**
 * "Can I …?": a verb on a resource in a Namespace, asked of the API server
 * as `kubectl auth can-i` does, with the same command to copy.
 */
export function CanIBox({ clusterId, namespace }: { clusterId: string; namespace: string }) {
  const canI = useCanI(clusterId);
  const verbs = useId();
  const [verb, setVerb] = useState('get');
  const [resource, setResource] = useState('');
  const [ns, setNs] = useState(namespace);
  const [group, setGroup] = useState('');
  const [subresource, setSubresource] = useState('');
  const [name, setName] = useState('');
  const ask = (e: FormEvent) => {
    e.preventDefault();
    canI.mutate({
      verb,
      resource,
      namespace: optional(ns),
      group: optional(group),
      subresource: optional(subresource),
      name: optional(name),
    });
  };
  const answer = canI.data;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('rbac.canI.title')}</CardTitle>
      </CardHeader>
      <CardContent className="grid gap-4">
        <form onSubmit={ask} className="grid gap-3">
          <datalist id={verbs}>
            {VERBS.map((v) => (
              <option key={v} value={v} />
            ))}
          </datalist>
          <div className="grid gap-3 sm:grid-cols-3">
            <Field label={t('rbac.canI.verb')} value={verb} onChange={setVerb} list={verbs} />
            <Field
              label={t('rbac.canI.resource')}
              value={resource}
              onChange={setResource}
              placeholder={t('rbac.canI.resourcePlaceholder')}
            />
            <Field
              label={t('rbac.canI.namespace')}
              value={ns}
              onChange={setNs}
              placeholder={t('rbac.canI.allNamespaces')}
            />
          </div>
          <div className="grid gap-3 sm:grid-cols-3">
            <Field
              label={t('rbac.canI.group')}
              value={group}
              onChange={setGroup}
              placeholder={t('rbac.canI.optional')}
            />
            <Field
              label={t('rbac.canI.subresource')}
              value={subresource}
              onChange={setSubresource}
              placeholder={t('rbac.canI.optional')}
            />
            <Field
              label={t('rbac.canI.name')}
              value={name}
              onChange={setName}
              placeholder={t('rbac.canI.optional')}
            />
          </div>
          <div>
            <Button type="submit" disabled={canI.isPending}>
              {t('rbac.canI.ask')}
            </Button>
          </div>
        </form>
        {canI.error && <ErrorMessage error={canI.error} />}
        {answer && (
          <div className="grid gap-2 text-sm" role="status">
            <div className="flex flex-wrap items-center gap-2">
              {answer.allowed ? (
                <Badge>
                  <CircleCheckIcon /> {t('rbac.canI.yes')}
                </Badge>
              ) : (
                <Badge variant="destructive">
                  <CircleXIcon /> {t('rbac.canI.no')}
                </Badge>
              )}
              {answer.denied && <Badge variant="outline">{t('rbac.canI.denied')}</Badge>}
              <span className="text-muted-foreground">
                {t('rbac.canI.checked', { resource: answer.resource })}
              </span>
            </div>
            {!answer.known && (
              <p className="text-muted-foreground">
                {t('rbac.canI.unknown', { resource: answer.resource })}
              </p>
            )}
            {answer.reason && <p className="text-muted-foreground">{answer.reason}</p>}
            {answer.evaluationError && (
              <p className="text-destructive font-mono text-xs">{answer.evaluationError}</p>
            )}
            <div className="flex flex-wrap items-center gap-2">
              <code className="bg-muted rounded px-2 py-1 font-mono text-xs">{answer.command}</code>
              <CopyButton text={answer.command} />
            </div>
          </div>
        )}
      </CardContent>
    </Card>
  );
}
