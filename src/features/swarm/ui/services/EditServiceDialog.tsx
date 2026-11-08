import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import type { ServiceView } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  ErrorMessage,
  Input,
  Label,
  Spinner,
  Textarea,
} from '@/shared/ui';
import { useUpdateService } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import {
  changesSomething,
  editForm,
  toServiceChange,
  type EditForm,
} from '../../model/serviceForm';
import { t } from '@/shared/i18n';

function EditFields({
  swarmId,
  service,
  initial,
  onClose,
}: {
  swarmId: string;
  service: ServiceView;
  initial: EditForm;
  onClose: () => void;
}) {
  const [form, setForm] = useState(initial);
  const update = useUpdateService(swarmId, onClose);
  const set = (patch: Partial<EditForm>) => setForm((f) => ({ ...f, ...patch }));
  const change = toServiceChange(service, initial, form);
  return (
    <form
      className="grid gap-3"
      onSubmit={(e) => {
        e.preventDefault();
        if (change) update.mutate({ service, change });
      }}
    >
      <div className="grid gap-1.5">
        <Label htmlFor="edit-image">{t('swarm.create.image')}</Label>
        <Input
          id="edit-image"
          value={form.image}
          onChange={(e) => set({ image: e.target.value })}
        />
      </div>
      {service.mode === 'replicated' && (
        <div className="grid gap-1.5">
          <Label htmlFor="edit-replicas">{t('swarm.services.scale.replicas')}</Label>
          <Input
            id="edit-replicas"
            className="w-24"
            inputMode="numeric"
            value={form.replicas}
            aria-invalid={change === null}
            onChange={(e) => set({ replicas: e.target.value })}
          />
        </div>
      )}
      <div className="grid gap-1.5">
        <Label htmlFor="edit-env">{t('swarm.create.env')}</Label>
        <Textarea
          id="edit-env"
          rows={6}
          className="font-mono text-xs"
          value={form.env}
          onChange={(e) => set({ env: e.target.value })}
        />
      </div>
      <label className="flex items-center gap-2 text-sm">
        <Checkbox checked={form.redeploy} onCheckedChange={(v) => set({ redeploy: v === true })} />
        {t('swarm.edit.redeploy')}
      </label>
      {update.error && <ErrorMessage error={update.error} />}
      <DialogFooter>
        <Button type="button" variant="ghost" onClick={onClose}>
          {t('common.cancel')}
        </Button>
        <Button type="submit" disabled={!change || !changesSomething(change) || update.isPending}>
          {t('swarm.edit.submit')}
        </Button>
      </DialogFooter>
    </form>
  );
}

/**
 * Changes a service's image, environment and replicas, from the version
 * last read; the Swarm rolls its tasks over to them.
 */
export function EditServiceDialog({
  swarmId,
  service,
  onClose,
}: {
  swarmId: string;
  service: ServiceView;
  onClose: () => void;
}) {
  const inspect = useQuery(swarmFeatureQueries.serviceInspect(swarmId, service.id));
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('swarm.edit.title', { name: service.name })}</DialogTitle>
          <DialogDescription>{t('swarm.edit.description')}</DialogDescription>
        </DialogHeader>
        {inspect.error ? (
          <ErrorMessage error={inspect.error} onRetry={() => void inspect.refetch()} />
        ) : !inspect.data ? (
          <Spinner />
        ) : (
          <EditFields
            swarmId={swarmId}
            service={service}
            initial={editForm(service, inspect.data)}
            onClose={onClose}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}
