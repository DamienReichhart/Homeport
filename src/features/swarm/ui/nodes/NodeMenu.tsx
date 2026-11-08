import {
  ArrowDownIcon,
  ArrowUpIcon,
  EllipsisVerticalIcon,
  ListChecksIcon,
  PauseIcon,
  PlayIcon,
  TagIcon,
  Trash2Icon,
  TruckIcon,
} from 'lucide-react';
import type { NodeChange, SwarmNode } from '@/shared/api';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/** A node's actions: availability, role, labels, its tasks and removal. */
export function NodeMenu({
  node,
  onChange,
  onEditLabels,
  onShowTasks,
  onRemove,
}: {
  node: SwarmNode;
  onChange: (change: NodeChange) => void;
  onEditLabels: () => void;
  onShowTasks?: () => void;
  onRemove: () => void;
}) {
  const set = (availability: NodeChange['availability']) =>
    onChange({ availability, role: null, labels: null });
  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button
          variant="ghost"
          size="icon-sm"
          aria-label={t('swarm.nodes.actionsFor', { name: node.hostname })}
          onClick={(e) => e.stopPropagation()}
        >
          <EllipsisVerticalIcon />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent align="end">
        <DropdownMenuItem disabled={node.availability === 'active'} onSelect={() => set('active')}>
          <PlayIcon /> {t('swarm.nodes.action.activate')}
        </DropdownMenuItem>
        <DropdownMenuItem disabled={node.availability === 'pause'} onSelect={() => set('pause')}>
          <PauseIcon /> {t('swarm.nodes.action.pause')}
        </DropdownMenuItem>
        <DropdownMenuItem disabled={node.availability === 'drain'} onSelect={() => set('drain')}>
          <TruckIcon /> {t('swarm.nodes.action.drain')}
        </DropdownMenuItem>
        <DropdownMenuSeparator />
        {node.role === 'worker' ? (
          <DropdownMenuItem
            onSelect={() => onChange({ availability: null, role: 'manager', labels: null })}
          >
            <ArrowUpIcon /> {t('swarm.nodes.action.promote')}
          </DropdownMenuItem>
        ) : (
          <DropdownMenuItem
            onSelect={() => onChange({ availability: null, role: 'worker', labels: null })}
          >
            <ArrowDownIcon /> {t('swarm.nodes.action.demote')}
          </DropdownMenuItem>
        )}
        <DropdownMenuItem onSelect={onEditLabels}>
          <TagIcon /> {t('swarm.nodes.action.labels')}
        </DropdownMenuItem>
        {onShowTasks && (
          <DropdownMenuItem onSelect={onShowTasks}>
            <ListChecksIcon /> {t('swarm.nodes.action.tasks')}
          </DropdownMenuItem>
        )}
        <DropdownMenuSeparator />
        <DropdownMenuItem variant="destructive" onSelect={onRemove}>
          <Trash2Icon /> {t('common.remove')}
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  );
}
