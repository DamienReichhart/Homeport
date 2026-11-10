import { useQuery } from '@tanstack/react-query';
import { InfoIcon } from 'lucide-react';
import { Link, useParams } from 'react-router-dom';
import { ClusterGate, NamespacePicker, useNamespace } from '@/core/cluster';
import type {
  AppliedBinding,
  ClusterView,
  Subject,
  SubjectKind,
  SubjectPermissions,
} from '@/shared/api';
import {
  Alert,
  AlertDescription,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  DataTable,
  ErrorMessage,
  PageHeader,
  Spinner,
  type ColumnDef,
} from '@/shared/ui';
import { rbacQueries } from '../api/queries';
import { subjectName, subjectPath } from '../model/paths';
import { PermissionMatrix } from './PermissionMatrix';
import { BoundRole } from './RbacLists';
import { Refused } from './Refused';
import { t } from '@/shared/i18n';

const bindingName = (a: AppliedBinding) =>
  a.binding.namespace ? `${a.binding.namespace}/${a.binding.name}` : a.binding.name;

const columns: ColumnDef<AppliedBinding, unknown>[] = [
  { id: 'name', header: t('rbac.column.name'), accessorFn: (a) => a.binding.name },
  { id: 'kind', header: t('rbac.column.kind'), accessorFn: (a) => a.binding.kind },
  {
    id: 'namespace',
    header: t('rbac.column.namespace'),
    accessorFn: (a) => a.binding.namespace ?? t('rbac.clusterWide'),
  },
  {
    id: 'role',
    header: t('rbac.column.role'),
    accessorFn: (a) => a.binding.roleName,
    cell: ({ row }) => <BoundRole binding={row.original.binding} />,
  },
  {
    id: 'how',
    header: t('rbac.column.how'),
    accessorFn: (a) => a.throughGroup ?? '',
    cell: ({ row }) => {
      const group = row.original.throughGroup;
      if (!group) return t('rbac.subject.direct');
      return (
        <Link
          className="text-primary"
          to={subjectPath({ kind: 'Group', name: group, namespace: null })}
        >
          {t('rbac.subject.through', { group })}
        </Link>
      );
    },
  },
];

function Permissions({ data }: { data: SubjectPermissions }) {
  const via = (indexes: number[]) =>
    t('rbac.subject.via', {
      bindings: indexes
        .map((i) => data.bindings[i])
        .filter((a) => a !== undefined)
        .map(bindingName)
        .join(', '),
    });
  return (
    <div className="grid gap-4">
      <Alert>
        <InfoIcon />
        <AlertDescription className="grid gap-1">
          <p>{t('rbac.subject.bestEffort')}</p>
          {data.groups.length > 0 && (
            <p>
              {t('rbac.subject.groups')} <code>{data.groups.join(', ')}</code>
            </p>
          )}
          {data.namespace && (
            <p>{t('rbac.subject.onlyNamespace', { namespace: data.namespace })}</p>
          )}
        </AlertDescription>
      </Alert>
      <Refused errors={data.refused} />
      {data.scopes.length === 0 && (
        <p className="text-muted-foreground text-sm">{t('rbac.subject.none')}</p>
      )}
      {data.scopes.map((scope) => (
        <Card key={scope.namespace ?? ''}>
          <CardHeader>
            <CardTitle>
              {scope.namespace
                ? t('rbac.subject.inNamespace', { namespace: scope.namespace })
                : t('rbac.subject.cluster')}
            </CardTitle>
            {!scope.namespace && <CardDescription>{t('rbac.subject.clusterHint')}</CardDescription>}
          </CardHeader>
          <CardContent>
            <PermissionMatrix grants={scope.grants} via={via} />
          </CardContent>
        </Card>
      ))}
      <Card>
        <CardHeader>
          <CardTitle>{t('rbac.subject.bindings')}</CardTitle>
        </CardHeader>
        <CardContent>
          <DataTable
            columns={columns}
            data={data.bindings}
            getRowId={(a) => `${a.binding.kind}/${a.binding.namespace ?? ''}/${a.binding.name}`}
            empty={t('rbac.empty')}
          />
        </CardContent>
      </Card>
    </div>
  );
}

function SubjectDetail({ cluster, kind }: { cluster: ClusterView; kind: SubjectKind }) {
  const { namespace: subjectNamespace = null, name = '' } = useParams();
  const [namespace, setNamespace] = useNamespace(cluster);
  const subject: Subject = {
    kind,
    name,
    namespace: kind === 'ServiceAccount' ? subjectNamespace : null,
  };
  const id = cluster.cluster.id;
  const permissions = useQuery(rbacQueries.subject(id, subject, namespace));
  return (
    <div className="grid gap-3">
      <PageHeader
        title={subjectName(subject)}
        description={kind}
        actions={<NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />}
      />
      {permissions.isPending ? (
        <Spinner />
      ) : permissions.error ? (
        <ErrorMessage error={permissions.error} onRetry={() => void permissions.refetch()} />
      ) : (
        <Permissions data={permissions.data} />
      )}
    </div>
  );
}

function SubjectPage({ kind }: { kind: SubjectKind }) {
  return <ClusterGate>{(cluster) => <SubjectDetail cluster={cluster} kind={kind} />}</ClusterGate>;
}

export function UserPage() {
  return <SubjectPage kind="User" />;
}

export function GroupPage() {
  return <SubjectPage kind="Group" />;
}

export function ServiceAccountPage() {
  return <SubjectPage kind="ServiceAccount" />;
}
