import type { ReactNode } from 'react';
import { age, Labels } from '@/core/cluster';
import type { WorkloadDetail } from '@/shared/api';
import { Card, CardContent, CardHeader, CardTitle, DataTable, KeyValue } from '@/shared/ui';
import { readyText } from '../../model/kinds';
import { t } from '@/shared/i18n';

/** A workload's settings, labels and its Pods' containers. */
export function WorkloadOverview({ detail }: { detail: WorkloadDetail }) {
  const v = detail.view;
  const items: [string, ReactNode][] = [[t('workloads.detail.namespace'), v.namespace]];
  items.push([t('workloads.detail.ready'), readyText(v.ready, v.desired, v.kind)]);
  if (v.upToDate !== null) items.push([t('workloads.detail.upToDate'), v.upToDate]);
  if (v.schedule) items.push([t('workloads.detail.schedule'), <code>{v.schedule}</code>]);
  if (v.kind === 'cronJob') items.push([t('workloads.detail.lastSchedule'), age(v.lastSchedule)]);
  if (detail.strategy) items.push([t('workloads.detail.strategy'), detail.strategy]);
  if (detail.selector)
    items.push([
      t('workloads.detail.selector'),
      <code className="text-xs">{detail.selector}</code>,
    ]);
  if (v.owner) items.push([t('workloads.detail.owner'), `${v.owner.kind} ${v.owner.name}`]);
  items.push([t('workloads.detail.age'), age(v.created)]);
  items.push([t('workloads.detail.labels'), <Labels values={detail.labels} />]);
  items.push([t('workloads.detail.annotations'), <Labels values={detail.annotations} />]);
  return (
    <div className="grid gap-4">
      <Card>
        <CardContent>
          <KeyValue items={items} />
        </CardContent>
      </Card>
      <Card>
        <CardHeader>
          <CardTitle>{t('workloads.detail.containers')}</CardTitle>
        </CardHeader>
        <CardContent>
          <DataTable
            columns={[
              { id: 'name', header: t('workloads.container.name'), accessorFn: (c) => c.name },
              {
                id: 'image',
                header: t('workloads.container.image'),
                accessorFn: (c) => c.image,
                cell: ({ row }) => (
                  <span className="font-mono text-xs break-all">{row.original.image}</span>
                ),
              },
              {
                id: 'ports',
                header: t('workloads.container.ports'),
                accessorFn: (c) => c.ports.join(', '),
              },
            ]}
            data={detail.containers}
            getRowId={(c) => c.name}
          />
        </CardContent>
      </Card>
    </div>
  );
}
