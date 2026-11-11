import { ShieldCheckIcon } from 'lucide-react';
import { defineFeature } from '@/core/feature';
import { RbacPage } from './ui/RbacPage';
import { ClusterRolePage, RolePage } from './ui/RoleDetailPage';
import { GroupPage, ServiceAccountPage, UserPage } from './ui/SubjectDetailPage';

export const rbacFeature = defineFeature({
  routes: [
    { path: 'rbac/:tab?', Component: RbacPage },
    { path: 'rbac/clusterroles/:name', Component: ClusterRolePage },
    { path: 'rbac/roles/:namespace/:name', Component: RolePage },
    { path: 'rbac/users/:name', Component: UserPage },
    { path: 'rbac/groups/:name', Component: GroupPage },
    { path: 'rbac/serviceaccounts/:namespace/:name', Component: ServiceAccountPage },
  ],
  nav: { path: '/rbac', label: 'rbac.nav', icon: ShieldCheckIcon, scope: 'cluster' },
  clusterKind: 'kubernetes',
});
