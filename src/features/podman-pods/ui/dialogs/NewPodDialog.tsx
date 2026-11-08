import { useState } from 'react';
import { PortsField } from '@/features/containers';
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
  Textarea,
} from '@/shared/ui';
import { useCreatePod } from '../../api/mutations';
import {
  canCreatePod,
  emptyPodForm,
  podFormErrors,
  toPodSpec,
  type PodForm,
} from '../../model/pods';
import { t } from '@/shared/i18n';

/** Creates a pod with its published ports and labels. */
export function NewPodDialog({
  hostId,
  open,
  onOpenChange,
  onCreated,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onCreated?: (id: string) => void;
}) {
  const [form, setForm] = useState<PodForm>(emptyPodForm);
  const create = useCreatePod(hostId, (id) => {
    setForm(emptyPodForm());
    onOpenChange(false);
    onCreated?.(id);
  });
  const errors = podFormErrors(form);
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('podmanpods.newDialog.title')}</DialogTitle>
          <DialogDescription>{t('podmanpods.newDialog.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (canCreatePod(form)) create.mutate(toPodSpec(form));
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="pod-name">{t('podmanpods.newDialog.name')}</Label>
            <Input
              id="pod-name"
              value={form.name}
              placeholder="web"
              autoFocus
              aria-invalid={errors.name ? true : undefined}
              onChange={(e) => setForm({ ...form, name: e.target.value })}
            />
            {errors.name && <p className="text-destructive text-xs">{t(errors.name)}</p>}
          </div>
          <div className="grid gap-1.5">
            <PortsField ports={form.ports} onChange={(ports) => setForm({ ...form, ports })} />
            <p className="text-muted-foreground text-xs">{t('podmanpods.newDialog.portsHint')}</p>
            {errors.ports && <p className="text-destructive text-xs">{t(errors.ports)}</p>}
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="pod-labels">{t('podmanpods.newDialog.labels')}</Label>
            <Textarea
              id="pod-labels"
              rows={3}
              className="font-mono text-xs"
              placeholder="app=web"
              value={form.labels}
              onChange={(e) => setForm({ ...form, labels: e.target.value })}
            />
            {errors.labels && <p className="text-destructive text-xs">{t(errors.labels)}</p>}
          </div>
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="outline" onClick={() => onOpenChange(false)}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!canCreatePod(form) || create.isPending}>
              {t('podmanpods.newDialog.create')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
