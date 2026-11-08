import { useQuery, useQueryClient } from '@tanstack/react-query';
import { useEffect, useReducer, useRef, useState, type ReactNode } from 'react';
import { useNavigate } from 'react-router-dom';
import { toast } from 'sonner';
import { hostKeys } from '@/core/host';
import {
  DEFAULT_SERVER,
  incusImageQueries,
  initialProgress,
  OperationProgress,
  reduceProgress,
  RemoteImagePicker,
} from '@/features/incus-images';
import { incusProfileQueries } from '@/features/incus-profiles';
import type { InstanceType } from '@/shared/api';
import {
  Button,
  Checkbox,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Switch,
  ToggleGroup,
  ToggleGroupItem,
} from '@/shared/ui';
import { startCreateInstance } from '../api/streams';
import {
  emptyNewInstance,
  problem,
  toSpec,
  type FormProblem,
  type ImageOrigin,
  type NewInstanceForm,
} from '../model/newInstance';
import { t, type MessageKey } from '@/shared/i18n';

const PROBLEMS: Record<FormProblem, MessageKey> = {
  name: 'incus.new.problem.name',
  image: 'incus.new.problem.image',
  cpu: 'incus.new.problem.cpu',
  memory: 'incus.new.problem.memory',
  disk: 'incus.new.problem.disk',
};

function Field({ id, label, children }: { id: string; label: string; children: ReactNode }) {
  return (
    <div className="grid gap-1.5">
      <Label htmlFor={id}>{label}</Label>
      {children}
    </div>
  );
}

/**
 * Makes a system container or a virtual machine from a local image or a
 * remote server's, with profiles and limits, showing the download until
 * it's made.
 */
export function NewInstanceDialog({
  hostId,
  project,
  open,
  onClose,
}: {
  hostId: string;
  project: string;
  open: boolean;
  onClose: () => void;
}) {
  const client = useQueryClient();
  const navigate = useNavigate();
  const [form, setForm] = useState<NewInstanceForm>(() => emptyNewInstance(DEFAULT_SERVER));
  const set = <K extends keyof NewInstanceForm>(key: K, value: NewInstanceForm[K]) =>
    setForm((f) => ({ ...f, [key]: value }));
  const [started, setStarted] = useState(false);
  const [progress, dispatch] = useReducer(reduceProgress, initialProgress);
  const cancel = useRef<(() => void) | null>(null);
  const local = useQuery(incusImageQueries.list(hostId, project));
  const profiles = useQuery(incusProfileQueries.list(hostId, project));

  // Closing stops following it; Incus finishes making it anyway.
  useEffect(() => () => cancel.current?.(), []);

  const localImages = (local.data ?? []).filter((i) => i.type === form.type);
  const wrong = problem(form);

  const start = () => {
    const spec = toSpec(form);
    setStarted(true);
    cancel.current = startCreateInstance(
      hostId,
      project,
      spec,
      (event) => {
        dispatch(event);
        if (event.type === 'done') {
          toast.success(t('incus.new.made', { name: spec.name }));
          void client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'instances') });
          void client.invalidateQueries({ queryKey: hostKeys.incus(hostId, 'images') });
          void client.invalidateQueries({ queryKey: hostKeys.incusProjects(hostId) });
        }
      },
      (error) => dispatch({ type: 'error', error }),
    );
  };

  return (
    <Dialog open={open} onOpenChange={(next) => !next && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-xl">
        <DialogHeader>
          <DialogTitle>{t('incus.new.title')}</DialogTitle>
          <DialogDescription>{t('incus.new.description', { project })}</DialogDescription>
        </DialogHeader>
        {started ? (
          <OperationProgress state={progress} done={t('incus.new.done', { name: form.name })} />
        ) : (
          <div className="grid gap-3">
            <Field id="instance-name" label={t('incus.new.name')}>
              <Input
                id="instance-name"
                value={form.name}
                onChange={(e) => set('name', e.target.value)}
                placeholder="web"
                autoFocus
              />
            </Field>
            <ToggleGroup
              type="single"
              variant="outline"
              size="sm"
              value={form.type}
              onValueChange={(v) => v && set('type', v as InstanceType)}
              className="justify-self-start"
              aria-label={t('incus.instances.column.type')}
            >
              <ToggleGroupItem value="container">{t('incus.type.container')}</ToggleGroupItem>
              <ToggleGroupItem value="virtual-machine">
                {t('incus.type.virtualMachine')}
              </ToggleGroupItem>
            </ToggleGroup>
            <div className="grid gap-2 rounded-md border p-3">
              <ToggleGroup
                type="single"
                size="sm"
                value={form.origin}
                onValueChange={(v) => v && set('origin', v as ImageOrigin)}
                className="justify-self-start"
                aria-label={t('incus.new.image')}
              >
                <ToggleGroupItem value="remote">{t('incus.new.origin.remote')}</ToggleGroupItem>
                <ToggleGroupItem value="local">{t('incus.new.origin.local')}</ToggleGroupItem>
              </ToggleGroup>
              {form.origin === 'remote' ? (
                <>
                  <Field id="instance-server" label={t('incus.remote.server')}>
                    <Input
                      id="instance-server"
                      value={form.server}
                      onChange={(e) => set('server', e.target.value)}
                    />
                  </Field>
                  <RemoteImagePicker
                    hostId={hostId}
                    server={form.server}
                    type={form.type}
                    alias={form.alias}
                    onAlias={(alias) => set('alias', alias)}
                  />
                </>
              ) : (
                <Select value={form.fingerprint} onValueChange={(v) => set('fingerprint', v)}>
                  <SelectTrigger className="w-full" aria-label={t('incus.new.localImage')}>
                    <SelectValue placeholder={t('incus.new.localImagePlaceholder')} />
                  </SelectTrigger>
                  <SelectContent>
                    {localImages.map((image) => (
                      <SelectItem key={image.fingerprint} value={image.fingerprint}>
                        {image.aliases[0] ?? image.fingerprint.slice(0, 12)} · {image.description}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              )}
            </div>
            <div className="grid gap-1.5">
              <span className="text-sm font-medium">{t('incus.new.profiles')}</span>
              <div className="flex flex-wrap gap-x-4 gap-y-1.5">
                {(profiles.data ?? []).map((profile) => {
                  const id = `profile-${profile.name}`;
                  const checked = form.profiles.includes(profile.name);
                  return (
                    <div key={profile.name} className="flex items-center gap-1.5">
                      <Checkbox
                        id={id}
                        checked={checked}
                        onCheckedChange={(v) =>
                          set(
                            'profiles',
                            v === true
                              ? [...form.profiles, profile.name]
                              : form.profiles.filter((p) => p !== profile.name),
                          )
                        }
                      />
                      <Label htmlFor={id}>{profile.name}</Label>
                    </div>
                  );
                })}
              </div>
            </div>
            <div className="grid grid-cols-3 gap-3">
              <Field id="instance-cpu" label={t('incus.new.cpu')}>
                <Input
                  id="instance-cpu"
                  inputMode="numeric"
                  value={form.cpu}
                  onChange={(e) => set('cpu', e.target.value)}
                  placeholder="2"
                />
              </Field>
              <Field id="instance-memory" label={t('incus.new.memory')}>
                <Input
                  id="instance-memory"
                  value={form.memory}
                  onChange={(e) => set('memory', e.target.value)}
                  placeholder="2GiB"
                />
              </Field>
              <Field id="instance-disk" label={t('incus.new.disk')}>
                <Input
                  id="instance-disk"
                  value={form.disk}
                  onChange={(e) => set('disk', e.target.value)}
                  placeholder="20GiB"
                />
              </Field>
            </div>
            <p className="text-muted-foreground text-xs">{t('incus.new.limitsHint')}</p>
            <div className="flex items-center gap-2">
              <Switch
                id="instance-start"
                checked={form.start}
                onCheckedChange={(v) => set('start', v)}
              />
              <Label htmlFor="instance-start">{t('incus.new.start')}</Label>
            </div>
            {wrong && form.name !== '' && (
              <p className="text-muted-foreground text-sm">{t(PROBLEMS[wrong])}</p>
            )}
          </div>
        )}
        <DialogFooter>
          <Button variant="outline" onClick={onClose}>
            {started ? t('common.close') : t('common.cancel')}
          </Button>
          {!started && (
            <Button disabled={wrong !== null} onClick={start}>
              {t('incus.new.confirm')}
            </Button>
          )}
          {started && progress.done && (
            <Button
              onClick={() => {
                onClose();
                void navigate(`/incus/instances/${encodeURIComponent(form.name.trim())}`);
              }}
            >
              {t('incus.new.open')}
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
