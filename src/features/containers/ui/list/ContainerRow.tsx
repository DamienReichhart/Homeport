import { openUrl } from '@tauri-apps/plugin-opener';
import { Link, useNavigate } from 'react-router-dom';
import { NewerImage, newerLabel, type PodRef } from '@/core/host';
import type { Container, ContainerUsage } from '@/shared/api';
import { formatBytes, formatPercent, formatRelative, shortId } from '@/shared/lib';
import {
  Badge,
  Checkbox,
  type ConfirmOptions,
  StatusDot,
  TableCell,
  TableRow,
  toastError,
} from '@/shared/ui';
import { containerTone } from '../../model/containerTone';
import { portLabels } from '../../model/grouping';
import { ContainerMenu } from '../ContainerMenu';
import type { ContainerDialogRequest } from '../dialogs/ContainerDialogs';
import { t } from '@/shared/i18n';

/** The label Podman puts on the containers a Quadlet unit created. */
const QUADLET_UNIT = 'PODMAN_SYSTEMD_UNIT';

/** One container in the list; `nested` rows sit under their Stack and show the service. */
export function ContainerRow({
  hostId,
  container: c,
  nested,
  usage,
  pod,
  newer,
  address,
  selected,
  onSelect,
  onOpenDialog,
  confirm,
}: {
  hostId: string;
  container: Container;
  nested: boolean;
  usage?: ContainerUsage;
  /** The Podman pod it runs in. */
  pod?: PodRef;
  /** The registry has a newer image under the tag it was created from. */
  newer?: boolean;
  /** Where published ports are reachable, for links. */
  address: string;
  selected: boolean;
  onSelect: (on: boolean) => void;
  onOpenDialog: (request: ContainerDialogRequest) => void;
  confirm: (options: ConfirmOptions) => void;
}) {
  const navigate = useNavigate();
  return (
    <TableRow
      className="cursor-pointer"
      data-state={selected ? 'selected' : undefined}
      onClick={() => navigate(`/containers/${c.id}/logs`)}
    >
      <TableCell onClick={(e) => e.stopPropagation()}>
        <Checkbox
          aria-label={t('containers.selectOne', { name: c.name })}
          checked={selected}
          onCheckedChange={(v) => onSelect(v === true)}
        />
      </TableCell>
      <TableCell className={nested ? 'pl-8' : undefined}>
        <div className="flex items-center gap-2">
          <StatusDot tone={containerTone(c.state, c.health)} />
          <div className="min-w-0">
            <p className="flex items-center gap-1.5 font-medium">
              <span className="truncate">{nested && c.service ? c.service : c.name}</span>
              {c.labels[QUADLET_UNIT] && (
                <Badge asChild variant="secondary">
                  <Link
                    to={`/quadlets/${encodeURIComponent(c.labels[QUADLET_UNIT])}`}
                    title={t('containers.fromUnit', { unit: c.labels[QUADLET_UNIT] })}
                    onClick={(e) => e.stopPropagation()}
                  >
                    {c.labels[QUADLET_UNIT]}
                  </Link>
                </Badge>
              )}
              {pod && (
                <Badge asChild variant="outline">
                  <Link
                    to={`/podman-pods/${pod.id}`}
                    title={t('containers.inPod', { pod: pod.name })}
                    onClick={(e) => e.stopPropagation()}
                  >
                    {pod.name}
                  </Link>
                </Badge>
              )}
            </p>
            <p className="text-muted-foreground font-mono text-xs">{shortId(c.id)}</p>
          </div>
        </div>
      </TableCell>
      <TableCell className="max-w-56">
        <span className="flex items-center gap-1.5">
          <span className="truncate">{c.image}</span>
          {newer && <NewerImage label={newerLabel([c.image])} />}
        </span>
      </TableCell>
      <TableCell>
        <span className="text-sm">{c.status || c.state}</span>
        {c.health && (
          <Badge variant={c.health === 'unhealthy' ? 'destructive' : 'outline'} className="ml-1.5">
            {c.health}
          </Badge>
        )}
      </TableCell>
      <TableCell onClick={(e) => e.stopPropagation()}>
        <div className="flex max-w-48 flex-wrap gap-1">
          {portLabels(c)
            .slice(0, 3)
            .map((p) =>
              p.publicPort ? (
                <button
                  key={p.label}
                  type="button"
                  className="text-primary font-mono text-xs hover:underline"
                  onClick={() => openUrl(`http://${address}:${p.publicPort}`).catch(toastError)}
                >
                  {p.label}
                </button>
              ) : (
                <span key={p.label} className="text-muted-foreground font-mono text-xs">
                  {p.label}
                </span>
              ),
            )}
        </div>
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {usage ? formatPercent(usage.cpuPercent ?? 0) : '—'}
      </TableCell>
      <TableCell className="text-right tabular-nums">
        {usage ? formatBytes(usage.memoryUsage) : '—'}
      </TableCell>
      <TableCell className="text-muted-foreground whitespace-nowrap">
        {formatRelative(c.created)}
      </TableCell>
      <TableCell>
        <ContainerMenu
          hostId={hostId}
          container={c}
          onOpenDialog={onOpenDialog}
          confirm={confirm}
        />
      </TableCell>
    </TableRow>
  );
}
