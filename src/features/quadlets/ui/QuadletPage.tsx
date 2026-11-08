import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, PencilIcon, ScrollTextIcon, Trash2Icon } from 'lucide-react';
import { useState, type ReactNode } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { HostGate } from '@/core/host';
import type { QuadletUnit, QuadletUnits } from '@/shared/api';
import { shortId } from '@/shared/lib';
import {
  Button,
  Card,
  CardContent,
  CardHeader,
  CardTitle,
  EmptyState,
  ErrorMessage,
  PageHeader,
  Spinner,
  StatusDot,
  UnitFileEditor,
} from '@/shared/ui';
import { quadletQueries } from '../api/queries';
import { filesBlocked, findUnit, unitStatusLabel, unitTone } from '../model/units';
import { RemoveUnitDialog } from './dialogs/RemoveUnitDialog';
import { UnitDialog } from './dialogs/UnitDialog';
import { UnitActions } from './UnitActions';
import { t } from '@/shared/i18n';

function Row({ label, children }: { label: string; children: ReactNode }) {
  return (
    <div className="grid grid-cols-[10rem_1fr] gap-2 py-1 text-sm">
      <dt className="text-muted-foreground">{label}</dt>
      <dd className="min-w-0 break-all">{children}</dd>
    </div>
  );
}

/** The unit's file, read-only, as Podman reads it on the Host. */
function UnitFile({
  hostId,
  units,
  unit,
}: {
  hostId: string;
  units: QuadletUnits;
  unit: QuadletUnit;
}) {
  const readable = units.managesFiles && unit.name !== null;
  const file = useQuery({ ...quadletQueries.file(hostId, unit.name ?? ''), enabled: readable });
  let body: ReactNode;
  if (!unit.name)
    body = <p className="text-muted-foreground text-sm">{t('quadlets.file.unknown')}</p>;
  else if (!units.managesFiles)
    body = (
      <p className="text-muted-foreground text-sm">
        {t('quadlets.file.tooOld', { version: units.podmanVersion })}
      </p>
    );
  else if (file.isPending) body = <Spinner />;
  else if (file.error)
    body = <ErrorMessage error={file.error} onRetry={() => void file.refetch()} />;
  else body = <UnitFileEditor label={t('quadlets.file.title')} value={file.data} readOnly />;
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('quadlets.file.title')}</CardTitle>
      </CardHeader>
      <CardContent>{body}</CardContent>
    </Card>
  );
}

function UnitDetail({ hostId, unitKey }: { hostId: string; unitKey: string }) {
  const navigate = useNavigate();
  const units = useQuery(quadletQueries.list(hostId));
  const unit = units.data ? findUnit(units.data.units, unitKey) : null;
  const file = useQuery({
    ...quadletQueries.file(hostId, unit?.name ?? ''),
    enabled: Boolean(units.data?.managesFiles && unit?.name),
  });
  const [editing, setEditing] = useState(false);
  const [removing, setRemoving] = useState<QuadletUnit | null>(null);
  const back = () => void navigate('/quadlets');

  if (units.isPending) return <Spinner />;
  if (units.error) return <ErrorMessage error={units.error} onRetry={() => void units.refetch()} />;
  if (!unit)
    return (
      <EmptyState
        icon={ScrollTextIcon}
        title={t('quadlets.gone', { name: unitKey })}
        action={<Button onClick={back}>{t('quadlets.title')}</Button>}
      />
    );
  const blocked = filesBlocked(units.data) ?? (unit.name ? null : 'quadlets.blocked.noFile');
  const reason = blocked ? t(blocked, { version: units.data.podmanVersion }) : undefined;
  return (
    <div className="grid gap-4">
      <div>
        <Button variant="ghost" size="sm" className="-ml-2 mb-2" onClick={back}>
          <ArrowLeftIcon /> {t('quadlets.title')}
        </Button>
        <PageHeader
          title={
            <span className="flex items-center gap-2 font-mono">
              <StatusDot tone={unitTone(unit.status)} />
              {unit.name ?? unit.unitName}
            </span>
          }
          description={t(unitStatusLabel(unit.status))}
          actions={
            <>
              <UnitActions hostId={hostId} units={units.data} unit={unit} />
              <span title={reason}>
                <Button
                  variant="outline"
                  disabled={blocked !== null || !file.data}
                  onClick={() => setEditing(true)}
                >
                  <PencilIcon /> {t('quadlets.edit')}
                </Button>
              </span>
              <span title={reason}>
                <Button
                  variant="destructive"
                  disabled={blocked !== null}
                  onClick={() => setRemoving(unit)}
                >
                  <Trash2Icon /> {t('common.remove')}
                </Button>
              </span>
            </>
          }
        />
      </div>
      <Card>
        <CardHeader>
          <CardTitle>{t('quadlets.detail.service')}</CardTitle>
        </CardHeader>
        <CardContent>
          <dl>
            <Row label={t('quadlets.detail.unit')}>
              <span className="font-mono">{unit.unitName ?? t('quadlets.notGenerated')}</span>
            </Row>
            <Row label={t('quadlets.detail.scope')}>
              {unit.scope === 'user' ? t('quadlets.scope.user') : t('quadlets.scope.system')}
            </Row>
            <Row label={t('quadlets.detail.path')}>
              <span className="font-mono text-xs">{unit.path ?? '—'}</span>
            </Row>
            {unit.app && <Row label={t('quadlets.detail.app')}>{unit.app}</Row>}
            {unit.pod && <Row label={t('quadlets.detail.pod')}>{unit.pod}</Row>}
          </dl>
        </CardContent>
      </Card>
      <UnitFile hostId={hostId} units={units.data} unit={unit} />
      <Card>
        <CardHeader>
          <CardTitle>
            {t('quadlets.detail.containers', { count: unit.containers.length })}
          </CardTitle>
        </CardHeader>
        <CardContent className="grid gap-1">
          {unit.containers.length === 0 && (
            <p className="text-muted-foreground text-sm">{t('quadlets.detail.noContainers')}</p>
          )}
          {unit.containers.map((c) => (
            <Link
              key={c.id}
              to={`/containers/${c.id}/logs`}
              className="hover:bg-muted/60 flex items-center gap-2 rounded-md px-2 py-1.5 text-sm"
            >
              <StatusDot tone={c.state === 'running' ? 'success' : 'muted'} />
              <span className="font-medium">{c.name}</span>
              <span className="text-muted-foreground font-mono text-xs">{shortId(c.id)}</span>
              <span className="text-muted-foreground ml-auto">{c.state}</span>
            </Link>
          ))}
        </CardContent>
      </Card>
      {editing && file.data !== undefined && (
        <UnitDialog
          hostId={hostId}
          open
          onOpenChange={setEditing}
          editing={unit}
          initialContent={file.data}
        />
      )}
      <RemoveUnitDialog
        hostId={hostId}
        unit={removing}
        onClose={() => setRemoving(null)}
        onRemoved={back}
      />
    </div>
  );
}

export function QuadletPage() {
  const { name = '' } = useParams();
  return (
    <HostGate requires="podman">
      {(hostId) => <UnitDetail key={`${hostId}:${name}`} hostId={hostId} unitKey={name} />}
    </HostGate>
  );
}
