import { useQuery } from '@tanstack/react-query';
import { TriangleAlertIcon } from 'lucide-react';
import { Link } from 'react-router-dom';
import type { ClusterView, Subject } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  Card,
  CardContent,
  ErrorMessage,
  KeyValue,
  Spinner,
} from '@/shared/ui';
import { rbacQueries } from '../api/queries';
import { subjectPath } from '../model/paths';
import { CanIBox } from './CanIBox';
import { PermissionMatrix } from './PermissionMatrix';
import { t } from '@/shared/i18n';

const SERVICE_ACCOUNT = /^system:serviceaccount:([^:]+):([^:]+)$/;

/** The subject a user name stands for: a ServiceAccount's is `system:serviceaccount:<ns>:<name>`. */
function subjectOfUser(username: string): Subject {
  const sa = SERVICE_ACCOUNT.exec(username);
  return sa
    ? { kind: 'ServiceAccount', namespace: sa[1] ?? null, name: sa[2] ?? '' }
    : { kind: 'User', name: username, namespace: null };
}

/**
 * "What can I do here?": the API server's own answer for Homeport's
 * credentials in a Namespace, with its caveat, and a "Can I…?" box.
 */
export function AccessPanel({
  cluster,
  namespace,
}: {
  cluster: ClusterView;
  namespace: string | null;
}) {
  const id = cluster.cluster.id;
  const own = useQuery(rbacQueries.own(id, namespace));
  if (own.isPending) return <Spinner />;
  if (own.error) return <ErrorMessage error={own.error} onRetry={() => void own.refetch()} />;
  const { user, grants, incomplete, evaluationError } = own.data;
  return (
    <div className="grid gap-4">
      <p className="text-muted-foreground text-sm">
        {t('rbac.access.about', { name: cluster.cluster.name, namespace: own.data.namespace })}
      </p>
      {user && (
        <Card>
          <CardContent>
            <KeyValue
              items={[
                [
                  t('rbac.access.user'),
                  <span className="flex flex-wrap items-center gap-3">
                    <span className="font-mono">{user.username}</span>
                    <Link
                      className="text-primary text-sm"
                      to={subjectPath(subjectOfUser(user.username))}
                    >
                      {t('rbac.access.myBindings')}
                    </Link>
                  </span>,
                ],
                [t('rbac.access.groups'), user.groups.join(', ') || null],
              ]}
            />
          </CardContent>
        </Card>
      )}
      {(incomplete || evaluationError) && (
        <Alert>
          <TriangleAlertIcon />
          <AlertDescription className="grid gap-1">
            <p>{t('rbac.access.incomplete')}</p>
            {evaluationError && <p className="font-mono text-xs">{evaluationError}</p>}
          </AlertDescription>
        </Alert>
      )}
      <Card>
        <CardContent>
          <PermissionMatrix grants={grants} />
        </CardContent>
      </Card>
      <CanIBox key={own.data.namespace} clusterId={id} namespace={own.data.namespace} />
    </div>
  );
}
