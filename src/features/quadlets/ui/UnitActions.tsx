import { CirclePlayIcon, CircleStopIcon, RotateCwIcon } from 'lucide-react';
import type { LucideIcon } from 'lucide-react';
import type { QuadletUnit, QuadletUnits, UnitAction } from '@/shared/api';
import { Button, Tooltip, TooltipContent, TooltipTrigger } from '@/shared/ui';
import { useUnitAction } from '../api/mutations';
import { actBlocked, unitActions } from '../model/units';
import { t, type MessageKey } from '@/shared/i18n';

const ACTIONS: Record<UnitAction, { icon: LucideIcon; label: MessageKey; one: MessageKey }> = {
  start: { icon: CirclePlayIcon, label: 'quadlets.action.start', one: 'quadlets.actionOne.start' },
  stop: { icon: CircleStopIcon, label: 'quadlets.action.stop', one: 'quadlets.actionOne.stop' },
  restart: {
    icon: RotateCwIcon,
    label: 'quadlets.action.restart',
    one: 'quadlets.actionOne.restart',
  },
};

/**
 * Start, Stop and Restart for a unit's service, disabled with the reason
 * where Homeport can't run `systemctl` for it: outline buttons on its
 * page, the main one as an icon in a list row.
 */
export function UnitActions({
  hostId,
  units,
  unit,
  compact = false,
}: {
  hostId: string;
  units: QuadletUnits;
  unit: QuadletUnit;
  compact?: boolean;
}) {
  const act = useUnitAction(hostId);
  const blocked = actBlocked(units, unit);
  const name = unit.unitName ?? unit.name ?? '';
  return (
    <>
      {unitActions(unit.status, compact).map((action) => {
        const { icon: Icon, label, one } = ACTIONS[action];
        const button = (
          <Button
            variant={compact ? 'ghost' : 'outline'}
            size={compact ? 'icon-sm' : 'default'}
            aria-label={compact ? t(one, { name }) : undefined}
            disabled={blocked !== null || act.isPending}
            onClick={() => act.mutate({ unit, action })}
          >
            <Icon /> {!compact && t(label)}
          </Button>
        );
        if (!compact && !blocked) return <span key={action}>{button}</span>;
        return (
          <Tooltip key={action}>
            <TooltipTrigger asChild>
              {/* A disabled button shows no tooltip; its wrapper does. */}
              <span tabIndex={blocked ? 0 : -1}>{button}</span>
            </TooltipTrigger>
            <TooltipContent>{blocked ? t(blocked) : t(label)}</TooltipContent>
          </Tooltip>
        );
      })}
    </>
  );
}
