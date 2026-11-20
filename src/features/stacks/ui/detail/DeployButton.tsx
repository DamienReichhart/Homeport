import { ChevronDownIcon, RocketIcon, type LucideIcon } from 'lucide-react';
import type { ReactNode } from 'react';
import {
  Button,
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/shared/ui';
import { t } from '@/shared/i18n';

/**
 * A definition's one solid button: deploy what's in the editor (saving it
 * first), with saving without deploying one step away under it. The same
 * for Managed, Linked and Synced Stacks, and for a new one.
 */
export function DeployButton({
  label = t('stacks.managed.deploy'),
  icon: Icon = RocketIcon,
  canDeploy,
  canSave,
  unsaved = false,
  pending,
  onDeploy,
  onSave,
  children,
}: {
  label?: string;
  icon?: LucideIcon;
  canDeploy: boolean;
  /** There's something to save. */
  canSave: boolean;
  /** Says there are unsaved changes beside it. */
  unsaved?: boolean;
  pending: boolean;
  onDeploy: () => void;
  onSave: () => void;
  /** Shown after it, such as a way out of a conflict. */
  children?: ReactNode;
}) {
  return (
    <div className="flex flex-wrap items-center gap-3">
      <div className="flex">
        <Button className="rounded-e-none" disabled={pending || !canDeploy} onClick={onDeploy}>
          <Icon /> {label}
        </Button>
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button
              size="icon"
              className="border-primary-foreground/20 rounded-s-none border-s"
              aria-label={t('stacks.managed.moreSave')}
              disabled={pending || !canSave}
            >
              <ChevronDownIcon />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="start" className="w-56">
            <DropdownMenuItem disabled={!canSave} onSelect={onSave}>
              {t('stacks.managed.saveOnly')}
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      {children}
      {unsaved && (
        <span className="text-muted-foreground text-sm">{t('stacks.managed.unsaved')}</span>
      )}
    </div>
  );
}
