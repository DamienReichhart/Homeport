import type { UseQueryResult } from '@tanstack/react-query';
import type { ReactNode } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import type { ClusterView } from '@/shared/api';
import {
  ErrorMessage,
  PageHeader,
  Spinner,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
} from '@/shared/ui';
import type { DeleteTarget } from '../model/deletion';
import { DeleteObjectButton } from './DeleteObjectButton';

export interface ResourceTab {
  value: string;
  label: string;
  content: ReactNode;
}

/**
 * A Cluster object's page: its title and status, then tabs whose choice is
 * in the URL (`<base>/<tab>`), the first one by default.
 */
export function ResourceTabs<T>({
  detail,
  base,
  title,
  description,
  actions,
  deletable,
  tabs,
}: {
  detail: UseQueryResult<T>;
  base: string;
  title: ReactNode;
  description?: (data: T) => ReactNode;
  /** What the object's header offers, such as Delete. */
  actions?: (data: T) => ReactNode;
  /** Offers Delete, then goes back to `listPath`. */
  deletable?: { cluster: ClusterView; target: DeleteTarget; listPath: string };
  tabs: (data: T) => ResourceTab[];
}) {
  const { tab } = useParams();
  const navigate = useNavigate();
  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />;
  const list = tabs(detail.data);
  const value = list.some((x) => x.value === tab) ? tab : list[0]?.value;
  return (
    <div className="flex h-full min-h-0 flex-col">
      <PageHeader
        title={title}
        description={description?.(detail.data)}
        actions={
          (actions || deletable) && (
            <div className="flex items-center gap-2">
              {actions?.(detail.data)}
              {deletable && (
                <DeleteObjectButton
                  cluster={deletable.cluster}
                  target={deletable.target}
                  onDeleted={() => void navigate(deletable.listPath, { replace: true })}
                />
              )}
            </div>
          )
        }
      />
      <Tabs
        value={value}
        onValueChange={(next) => void navigate(`${base}/${next}`, { replace: true })}
      >
        <TabsList>
          {list.map((x) => (
            <TabsTrigger key={x.value} value={x.value}>
              {x.label}
            </TabsTrigger>
          ))}
        </TabsList>
        {list.map((x) => (
          <TabsContent key={x.value} value={x.value} className="pt-3">
            {x.content}
          </TabsContent>
        ))}
      </Tabs>
    </div>
  );
}
