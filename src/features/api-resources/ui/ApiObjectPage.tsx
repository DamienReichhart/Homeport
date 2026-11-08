import { useQuery } from '@tanstack/react-query';
import { PencilIcon } from 'lucide-react';
import { useState, type ReactNode } from 'react';
import { Link, useParams } from 'react-router-dom';
import {
  age,
  ClusterGate,
  ConditionList,
  EditYamlDialog,
  kindPath,
  Labels,
  ObjectEvents,
  ResourceTabs,
  useNamespace,
} from '@/core/cluster';
import type { ApiResourceInfo, ClusterView, ObjectDetail, OwnerRef } from '@/shared/api';
import { Badge, Button, Card, CardContent, CopyButton, KeyValue, Spinner } from '@/shared/ui';
import { apiQueries } from '../api/queries';
import { useApiWatch } from '../api/useApiWatch';
import { findType, objectPagePath, typePath } from '../model/types';
import { t } from '@/shared/i18n';

/** An owner's page: Homeport's own for its kinds, else its API type's. */
function ownerPath(
  owner: OwnerRef,
  namespace: string | null,
  types: ApiResourceInfo[],
): string | null {
  const own = kindPath(owner.kind, namespace, owner.name);
  if (own) return own;
  const [group, version] = owner.apiVersion.includes('/')
    ? owner.apiVersion.split('/')
    : ['', owner.apiVersion];
  const type = types.find(
    (x) => x.api.group === group && x.api.version === version && x.api.kind === owner.kind,
  );
  return type ? objectPagePath(type.api, type.api.namespaced ? namespace : null, owner.name) : null;
}

function Overview({ detail, types }: { detail: ObjectDetail; types: ApiResourceInfo[] }) {
  const items: [string, ReactNode][] = [
    [t('apiresources.detail.namespace'), detail.namespace],
    [
      t('apiresources.detail.owners'),
      detail.owners.length > 0 && (
        <span className="grid gap-0.5">
          {detail.owners.map((o) => {
            const path = ownerPath(o, detail.namespace, types);
            const label = `${o.kind} ${o.name}`;
            return path ? (
              <Link key={label} to={path}>
                {label}
              </Link>
            ) : (
              <span key={label}>{label}</span>
            );
          })}
        </span>
      ),
    ],
    [t('apiresources.detail.age'), age(detail.created)],
    [t('apiresources.detail.labels'), <Labels values={detail.labels} />],
    [t('apiresources.detail.annotations'), <Labels values={detail.annotations} />],
  ];
  return (
    <Card>
      <CardContent>
        <KeyValue items={items} />
      </CardContent>
    </Card>
  );
}

function ObjectView({ cluster, info }: { cluster: ClusterView; info: ApiResourceInfo }) {
  const id = cluster.cluster.id;
  const { namespace: segment = '-', name = '' } = useParams();
  const namespace = segment === '-' ? null : segment;
  const api = info.api;
  const detail = useQuery(apiQueries.object(id, api, namespace, name));
  useApiWatch(id, info.verbs.includes('watch') ? api : undefined, namespace, [
    ...apiQueries.object(id, api, namespace, name).queryKey,
  ]);
  const types = useQuery(apiQueries.types(id, null));
  const [editing, setEditing] = useState(false);
  const editable = info.verbs.includes('patch') && !(api.group === '' && api.kind === 'Secret');
  return (
    <>
      <ResourceTabs
        detail={detail}
        base={objectPagePath(api, namespace, name)}
        title={
          <span className="flex items-center gap-2">
            {name} <Badge variant="outline">{api.kind}</Badge>
          </span>
        }
        actions={() =>
          editable && (
            <Button variant="outline" size="sm" onClick={() => setEditing(true)}>
              <PencilIcon /> {t('apiresources.edit')}
            </Button>
          )
        }
        deletable={
          info.verbs.includes('delete')
            ? { cluster, target: { type: { api }, namespace, name }, listPath: typePath(api) }
            : undefined
        }
        tabs={(d) => [
          {
            value: 'overview',
            label: t('apiresources.tab.overview'),
            content: <Overview detail={d} types={types.data ?? []} />,
          },
          ...(d.conditions.length > 0
            ? [
                {
                  value: 'conditions',
                  label: t('apiresources.tab.conditions'),
                  content: <ConditionList conditions={d.conditions} />,
                },
              ]
            : []),
          {
            value: 'events',
            label: t('apiresources.tab.events'),
            content: (
              <ObjectEvents
                clusterId={id}
                kind={{ name: api.kind }}
                namespace={namespace}
                name={name}
              />
            ),
          },
          {
            value: 'yaml',
            label: t('apiresources.tab.source'),
            content: (
              <div className="relative">
                <div className="absolute top-2 right-2">
                  <CopyButton text={d.yaml} label={t('apiresources.copyYaml')} />
                </div>
                <pre className="bg-muted max-h-[70vh] overflow-auto rounded-md p-3 font-mono text-xs">
                  {d.yaml}
                </pre>
              </div>
            ),
          },
        ]}
      />
      {editing && (
        <EditYamlDialog
          clusterId={id}
          clusterName={cluster.cluster.name}
          target={{ api }}
          kindName={api.kind}
          namespace={namespace}
          name={name}
          onClose={() => {
            setEditing(false);
            void detail.refetch();
          }}
        />
      )}
    </>
  );
}

function ApiObject({ cluster }: { cluster: ClusterView }) {
  const { group = '', version = '', plural = '' } = useParams();
  const [namespace] = useNamespace(cluster);
  const types = useQuery(apiQueries.types(cluster.cluster.id, namespace));
  if (types.isPending) return <Spinner />;
  const info = types.data && findType(types.data, group, version, plural);
  if (!info) return <p className="text-muted-foreground text-sm">{t('apiresources.typeGone')}</p>;
  return <ObjectView cluster={cluster} info={info} />;
}

export function ApiObjectPage() {
  return <ClusterGate>{(cluster) => <ApiObject cluster={cluster} />}</ClusterGate>;
}
