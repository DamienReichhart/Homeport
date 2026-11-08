import { useQuery } from '@tanstack/react-query';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { MountsField, PortsField } from '@/features/containers';
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
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Textarea,
} from '@/shared/ui';
import { useCreateService } from '../../api/mutations';
import { swarmFeatureQueries } from '../../api/queries';
import { emptyServiceForm, toServiceInput, type ServiceForm } from '../../model/serviceForm';
import { ObjectPicker } from './ObjectPicker';
import { t } from '@/shared/i18n';

/** Creates a service on a Swarm from an image and its main settings. */
export function CreateServiceDialog({
  swarmId,
  onClose,
}: {
  swarmId: string;
  onClose: () => void;
}) {
  const navigate = useNavigate();
  const [form, setForm] = useState<ServiceForm>(emptyServiceForm);
  const networks = useQuery(swarmFeatureQueries.networks(swarmId));
  const create = useCreateService(swarmId, (id) => {
    onClose();
    void navigate(`/swarms/${swarmId}/services/${id}`);
  });
  const set = (patch: Partial<ServiceForm>) => setForm((f) => ({ ...f, ...patch }));
  const input = toServiceInput(form);

  return (
    <Dialog open onOpenChange={(open) => !open && !create.isPending && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>{t('swarm.create.title')}</DialogTitle>
          <DialogDescription>{t('swarm.create.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (input) create.mutate(input);
          }}
        >
          <div className="grid gap-3 sm:grid-cols-2">
            <div className="grid gap-1.5">
              <Label htmlFor="service-image">{t('swarm.create.image')}</Label>
              <Input
                id="service-image"
                value={form.image}
                autoFocus
                placeholder={t('swarm.create.imagePlaceholder')}
                onChange={(e) => set({ image: e.target.value })}
              />
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="service-name">{t('swarm.create.name')}</Label>
              <Input
                id="service-name"
                value={form.name}
                onChange={(e) => set({ name: e.target.value })}
              />
            </div>
          </div>
          <div className="flex flex-wrap items-end gap-3">
            <div className="grid gap-1.5">
              <Label>{t('swarm.create.mode')}</Label>
              <Select
                value={form.mode}
                onValueChange={(mode) => set({ mode: mode as ServiceForm['mode'] })}
              >
                <SelectTrigger className="w-44" aria-label={t('swarm.create.mode')}>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="replicated">{t('swarm.services.mode.replicated')}</SelectItem>
                  <SelectItem value="global">{t('swarm.services.mode.global')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
            {form.mode === 'replicated' && (
              <div className="grid gap-1.5">
                <Label htmlFor="service-replicas">{t('swarm.services.scale.replicas')}</Label>
                <Input
                  id="service-replicas"
                  className="w-24"
                  inputMode="numeric"
                  value={form.replicas}
                  aria-invalid={input === null}
                  onChange={(e) => set({ replicas: e.target.value })}
                />
              </div>
            )}
            <div className="grid gap-1.5">
              <Label>{t('swarm.create.restart')}</Label>
              <Select
                value={form.restart}
                onValueChange={(restart) => set({ restart: restart as ServiceForm['restart'] })}
              >
                <SelectTrigger className="w-44" aria-label={t('swarm.create.restart')}>
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value="default">{t('swarm.create.restart.default')}</SelectItem>
                  <SelectItem value="on-failure">{t('swarm.create.restart.onFailure')}</SelectItem>
                  <SelectItem value="none">{t('swarm.create.restart.none')}</SelectItem>
                </SelectContent>
              </Select>
            </div>
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="service-command">{t('swarm.create.command')}</Label>
            <Input
              id="service-command"
              value={form.command}
              placeholder={t('swarm.create.commandPlaceholder')}
              onChange={(e) => set({ command: e.target.value })}
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="service-env">{t('swarm.create.env')}</Label>
            <Textarea
              id="service-env"
              rows={3}
              className="font-mono text-xs"
              placeholder="KEY=value"
              value={form.env}
              onChange={(e) => set({ env: e.target.value })}
            />
          </div>
          <PortsField ports={form.ports} onChange={(ports) => set({ ports })} />
          {form.ports.length > 0 && (
            <label className="flex items-center gap-2 text-sm">
              <Checkbox
                checked={form.hostMode}
                onCheckedChange={(v) => set({ hostMode: v === true })}
              />
              {t('swarm.create.hostMode')}
            </label>
          )}
          <MountsField mounts={form.mounts} onChange={(mounts) => set({ mounts })} />
          {!!networks.data?.length && (
            <div className="grid gap-1.5">
              <Label>{t('swarm.create.networks')}</Label>
              {networks.data.map((n) => (
                <label key={n.id} className="flex items-center gap-2 text-sm">
                  <Checkbox
                    checked={form.networks.includes(n.name)}
                    onCheckedChange={(on) =>
                      set({
                        networks:
                          on === true
                            ? [...form.networks, n.name]
                            : form.networks.filter((x) => x !== n.name),
                      })
                    }
                  />
                  {n.name}
                </label>
              ))}
            </div>
          )}
          <ObjectPicker
            swarmId={swarmId}
            kind="secret"
            picked={form.secrets}
            onChange={(secrets) => set({ secrets })}
          />
          <ObjectPicker
            swarmId={swarmId}
            kind="config"
            picked={form.configs}
            onChange={(configs) => set({ configs })}
          />
          {create.error && <ErrorMessage error={create.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!input || create.isPending}>
              {t('swarm.create.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
