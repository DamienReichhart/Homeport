import { useQuery } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { Link, useParams } from 'react-router-dom';
import { age, ClusterGate, Labels, useNamespace } from '@/core/cluster';
import type { ClusterView, RoleKind, RoleView, Rule } from '@/shared/api';
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  ErrorMessage,
  KeyValue,
  PageHeader,
  Spinner,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/shared/ui';
import { rbacQueries } from '../api/queries';
import { binds, rolePath } from '../model/paths';
import { BindingsList } from './RbacLists';
import { Refused } from './Refused';
import { t } from '@/shared/i18n';

const list = (values: string[]) => values.join(', ');
const groups = (values: string[]) =>
  values.map((g) => (g === '' ? t('rbac.coreGroup') : g)).join(', ');

/** A role's rules, as RBAC writes them. */
function Rules({ rules }: { rules: Rule[] }) {
  if (!rules.length)
    return <p className="text-muted-foreground text-sm">{t('rbac.role.noRules')}</p>;
  return (
    <div className="overflow-x-auto">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>{t('rbac.rule.resources')}</TableHead>
            <TableHead>{t('rbac.rule.apiGroups')}</TableHead>
            <TableHead>{t('rbac.rule.resourceNames')}</TableHead>
            <TableHead>{t('rbac.rule.verbs')}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {rules.map((r, i) => (
            // Rules have no identity of their own; their order is the role's.
            <TableRow key={i}>
              <TableCell className="font-mono text-xs">
                {r.resources.length ? list(r.resources) : list(r.nonResourceUrls)}
              </TableCell>
              <TableCell className="font-mono text-xs">
                {r.resources.length ? groups(r.apiGroups) : t('rbac.rule.urls')}
              </TableCell>
              <TableCell className="font-mono text-xs">{list(r.resourceNames)}</TableCell>
              <TableCell className="font-mono text-xs">{list(r.verbs)}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}

function Overview({ role }: { role: RoleView }) {
  const items: [string, ReactNode][] = [
    [t('rbac.role.kind'), role.kind],
    [t('rbac.role.namespace'), role.namespace ?? t('rbac.clusterWide')],
  ];
  if (role.aggregation.length)
    items.push([t('rbac.role.aggregation'), <code>{role.aggregation.join('; ')}</code>]);
  if (role.aggregatedFrom.length)
    items.push([
      t('rbac.role.aggregatedFrom'),
      <span className="flex flex-wrap gap-x-3">
        {role.aggregatedFrom.map((name) => (
          <Link
            key={name}
            className="text-primary"
            to={rolePath({ kind: 'ClusterRole', namespace: null, name })}
          >
            {name}
          </Link>
        ))}
      </span>,
    ]);
  items.push([t('rbac.role.age'), age(role.created)]);
  items.push([t('rbac.role.labels'), <Labels values={role.labels} />]);
  return <KeyValue items={items} />;
}

function RoleDetail({ cluster, kind }: { cluster: ClusterView; kind: RoleKind }) {
  const { namespace: roleNamespace = null, name = '' } = useParams();
  const [pageNamespace] = useNamespace(cluster);
  // A Role's bindings are in its Namespace; a ClusterRole's anywhere the page shows.
  const namespace = kind === 'Role' ? roleNamespace : pageNamespace;
  const index = useQuery(rbacQueries.index(cluster.cluster.id, namespace));
  if (index.isPending) return <Spinner />;
  if (index.error) return <ErrorMessage error={index.error} onRetry={() => void index.refetch()} />;
  const wanted = { kind, namespace: kind === 'Role' ? roleNamespace : null, name };
  const role = index.data.roles.find(
    (r) => r.kind === kind && r.name === name && r.namespace === wanted.namespace,
  );
  const bindings = index.data.bindings.filter((b) => binds(b, wanted));
  return (
    <div className="grid gap-4">
      <PageHeader
        title={name}
        description={wanted.namespace ? `${kind} · ${wanted.namespace}` : kind}
      />
      <Refused errors={index.data.refused} />
      {role ? (
        <>
          <Card>
            <CardContent>
              <Overview role={role} />
            </CardContent>
          </Card>
          <Card>
            <CardHeader>
              <CardTitle>{t('rbac.role.rules')}</CardTitle>
            </CardHeader>
            <CardContent>
              <Rules rules={role.rules} />
            </CardContent>
          </Card>
        </>
      ) : (
        <p className="text-muted-foreground text-sm">{t('rbac.role.gone')}</p>
      )}
      <Card>
        <CardHeader>
          <CardTitle>{t('rbac.role.bindings')}</CardTitle>
        </CardHeader>
        <CardContent>
          {bindings.length ? (
            <BindingsList bindings={bindings} />
          ) : (
            <p className="text-muted-foreground text-sm">{t('rbac.role.noBindings')}</p>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

export function ClusterRolePage() {
  return (
    <ClusterGate>{(cluster) => <RoleDetail cluster={cluster} kind="ClusterRole" />}</ClusterGate>
  );
}

export function RolePage() {
  return <ClusterGate>{(cluster) => <RoleDetail cluster={cluster} kind="Role" />}</ClusterGate>;
}
