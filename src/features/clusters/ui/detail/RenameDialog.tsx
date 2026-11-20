import { useState } from 'react';
import type { ClusterView } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
} from '@/shared/ui';
import { useRenameCluster } from '../../api/mutations';
import { t } from '@/shared/i18n';

/** Renames a Kubernetes Cluster; its kubeconfig context keeps its own name. */
export function RenameDialog({ view, onClose }: { view: ClusterView; onClose: () => void }) {
  const [name, setName] = useState(view.cluster.name);
  const rename = useRenameCluster(view.cluster.id);
  const unchanged = !name.trim() || name.trim() === view.cluster.name;
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('clusters.rename.title', { name: view.cluster.name })}</DialogTitle>
          <DialogDescription>{t('clusters.rename.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (!unchanged) rename.mutate(name.trim(), { onSuccess: onClose });
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="cluster-name">{t('clusters.detail.name')}</Label>
            <Input
              id="cluster-name"
              value={name}
              autoFocus
              onChange={(e) => setName(e.target.value)}
            />
          </div>
          {rename.error && <ErrorMessage error={rename.error} />}
          <DialogFooter>
            <Button type="button" variant="outline" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={unchanged || rename.isPending}>
              {t('clusters.detail.rename')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
