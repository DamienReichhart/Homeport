import { useQuery } from '@tanstack/react-query';
import { RefreshCwIcon } from 'lucide-react';
import { useNavigate, useParams } from 'react-router-dom';
import { ClusterGate, NamespacePicker, useNamespace } from '@/core/cluster';
import { toAppError, type ClusterView } from '@/shared/api';
import {
  Alert,
  AlertDescription,
  Button,
  ErrorMessage,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import { rbacQueries } from '../api/queries';
import { RBAC_TABS, type RbacTab } from '../model/paths';
import { AccessPanel } from './AccessPanel';
import { BindingsList, RolesList, SubjectsList } from './RbacLists';
import { Refused } from './Refused';
import { t } from '@/shared/i18n';

const LABELS = {
  roles: 'rbac.tab.roles',
  bindings: 'rbac.tab.bindings',
  subjects: 'rbac.tab.subjects',
  access: 'rbac.tab.access',
} as const;

function Rbac({ cluster }: { cluster: ClusterView }) {
  const id = cluster.cluster.id;
  const { tab } = useParams();
  const navigate = useNavigate();
  const [namespace, setNamespace] = useNamespace(cluster);
  const index = useQuery(rbacQueries.index(id, namespace));
  const current: RbacTab = RBAC_TABS.find((x) => x === tab) ?? 'roles';
  const forbidden = index.error && toAppError(index.error).code === 'forbidden';
  return (
    <div className="grid gap-3">
      <PageHeader
        title={t('rbac.title')}
        description={
          index.data &&
          t('rbac.summary', {
            roles: index.data.roles.length,
            bindings: index.data.bindings.length,
          })
        }
        actions={
          <>
            <NamespacePicker clusterId={id} value={namespace} onChange={setNamespace} />
            <Button
              variant="outline"
              size="sm"
              onClick={() => void index.refetch()}
              disabled={index.isFetching}
            >
              <RefreshCwIcon /> {t('rbac.refresh')}
            </Button>
          </>
        }
      />
      {index.isPending ? (
        <Spinner />
      ) : forbidden ? (
        <div className="grid gap-4">
          <Alert>
            <AlertDescription>{t('rbac.forbidden')}</AlertDescription>
          </Alert>
          <ErrorMessage error={index.error} />
          <AccessPanel cluster={cluster} namespace={namespace} />
        </div>
      ) : index.error ? (
        <ErrorMessage error={index.error} onRetry={() => void index.refetch()} />
      ) : (
        <div className="grid gap-3">
          <Refused errors={index.data.refused} />
          <Tabs
            value={current}
            onValueChange={(next) => void navigate(`/rbac/${next}`, { replace: true })}
          >
            <TabsList>
              {RBAC_TABS.map((x) => (
                <TabsTrigger key={x} value={x}>
                  {t(LABELS[x])}
                </TabsTrigger>
              ))}
            </TabsList>
            <TabsContent value="roles" className="pt-3">
              <RolesList index={index.data} />
            </TabsContent>
            <TabsContent value="bindings" className="pt-3">
              <BindingsList bindings={index.data.bindings} />
            </TabsContent>
            <TabsContent value="subjects" className="pt-3">
              <SubjectsList index={index.data} />
            </TabsContent>
            <TabsContent value="access" className="pt-3">
              <AccessPanel cluster={cluster} namespace={namespace} />
            </TabsContent>
          </Tabs>
        </div>
      )}
    </div>
  );
}

export function RbacPage() {
  return (
    <ClusterGate>{(cluster) => <Rbac key={cluster.cluster.id} cluster={cluster} />}</ClusterGate>
  );
}
