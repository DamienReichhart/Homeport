import { useQuery } from '@tanstack/react-query';
import { PlusIcon, XIcon } from 'lucide-react';
import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { resourceQueries } from '@/core/host';
import type { RestartPolicy } from '@/shared/api';
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
  Switch,
  Textarea,
} from '@/shared/ui';
import { useRunContainer } from '../api/mutations';
import { emptyRunForm, type RunForm } from '../model/runForm';

const RESTART: { value: RestartPolicy; label: string }[] = [
  { value: 'no', label: 'Never' },
  { value: 'on-failure', label: 'On failure' },
  { value: 'unless-stopped', label: 'Unless stopped' },
  { value: 'always', label: 'Always' },
];
const DEFAULT_NETWORK = '__default__';

/** The Run dialog: create and start a container, pulling the image if needed. */
export function RunContainerDialog({
  hostId,
  open,
  onOpenChange,
  image = '',
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  image?: string;
}) {
  const navigate = useNavigate();
  const [form, setForm] = useState<RunForm>(() => emptyRunForm(image));
  const images = useQuery({ ...resourceQueries.images(hostId), enabled: open });
  const networks = useQuery({ ...resourceQueries.networks(hostId), enabled: open });
  const run = useRunContainer(hostId, (id) => {
    onOpenChange(false);
    setForm(emptyRunForm());
    navigate(`/containers/${id}/logs`);
  });
  const set = <K extends keyof RunForm>(key: K, value: RunForm[K]) =>
    setForm((f) => ({ ...f, [key]: value }));

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Run a container</DialogTitle>
          <DialogDescription>
            The image is pulled first if this host doesn&apos;t have it.
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            run.mutate(form);
          }}
        >
          <div className="grid grid-cols-2 gap-3">
            <div className="grid gap-1.5">
              <Label htmlFor="run-image">Image</Label>
              <Input
                id="run-image"
                list="run-images"
                value={form.image}
                onChange={(e) => set('image', e.target.value)}
                placeholder="nginx:latest"
                autoFocus
              />
              <datalist id="run-images">
                {images.data
                  ?.flatMap((i) => i.tags)
                  .map((t) => (
                    <option key={t} value={t} />
                  ))}
              </datalist>
            </div>
            <div className="grid gap-1.5">
              <Label htmlFor="run-name">Name</Label>
              <Input
                id="run-name"
                value={form.name}
                onChange={(e) => set('name', e.target.value)}
                placeholder="Generated if empty"
              />
            </div>
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="run-cmd">Command</Label>
            <Input
              id="run-cmd"
              value={form.command}
              onChange={(e) => set('command', e.target.value)}
              placeholder="The image's default"
              className="font-mono"
            />
          </div>

          <div className="grid gap-1.5">
            <Label>Ports</Label>
            {form.ports.map((p, i) => (
              <div key={i} className="flex items-center gap-2">
                <Input
                  aria-label="Host port"
                  placeholder="Host port (any)"
                  value={p.hostPort}
                  inputMode="numeric"
                  onChange={(e) =>
                    set(
                      'ports',
                      form.ports.map((x, j) => (j === i ? { ...x, hostPort: e.target.value } : x)),
                    )
                  }
                />
                <span className="text-muted-foreground">→</span>
                <Input
                  aria-label="Container port"
                  placeholder="Container port"
                  value={p.containerPort}
                  inputMode="numeric"
                  onChange={(e) =>
                    set(
                      'ports',
                      form.ports.map((x, j) =>
                        j === i ? { ...x, containerPort: e.target.value } : x,
                      ),
                    )
                  }
                />
                <Select
                  value={p.protocol}
                  onValueChange={(v) =>
                    set(
                      'ports',
                      form.ports.map((x, j) => (j === i ? { ...x, protocol: v } : x)),
                    )
                  }
                >
                  <SelectTrigger className="w-24" aria-label="Protocol">
                    <SelectValue />
                  </SelectTrigger>
                  <SelectContent>
                    <SelectItem value="tcp">tcp</SelectItem>
                    <SelectItem value="udp">udp</SelectItem>
                  </SelectContent>
                </Select>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  aria-label="Remove port"
                  onClick={() =>
                    set(
                      'ports',
                      form.ports.filter((_, j) => j !== i),
                    )
                  }
                >
                  <XIcon />
                </Button>
              </div>
            ))}
            <Button
              type="button"
              variant="outline"
              size="sm"
              className="justify-self-start"
              onClick={() =>
                set('ports', [...form.ports, { hostPort: '', containerPort: '', protocol: 'tcp' }])
              }
            >
              <PlusIcon /> Add port
            </Button>
          </div>

          <div className="grid gap-1.5">
            <Label>Volumes</Label>
            {form.mounts.map((m, i) => (
              <div key={i} className="flex items-center gap-2">
                <Input
                  aria-label="Source"
                  placeholder="Volume name or /host/path"
                  value={m.source}
                  onChange={(e) =>
                    set(
                      'mounts',
                      form.mounts.map((x, j) => (j === i ? { ...x, source: e.target.value } : x)),
                    )
                  }
                />
                <span className="text-muted-foreground">→</span>
                <Input
                  aria-label="Container path"
                  placeholder="/container/path"
                  value={m.target}
                  onChange={(e) =>
                    set(
                      'mounts',
                      form.mounts.map((x, j) => (j === i ? { ...x, target: e.target.value } : x)),
                    )
                  }
                />
                <label className="flex items-center gap-1 text-sm whitespace-nowrap">
                  <Checkbox
                    checked={m.readOnly}
                    onCheckedChange={(v) =>
                      set(
                        'mounts',
                        form.mounts.map((x, j) => (j === i ? { ...x, readOnly: v === true } : x)),
                      )
                    }
                  />
                  read-only
                </label>
                <Button
                  type="button"
                  variant="ghost"
                  size="icon-sm"
                  aria-label="Remove volume"
                  onClick={() =>
                    set(
                      'mounts',
                      form.mounts.filter((_, j) => j !== i),
                    )
                  }
                >
                  <XIcon />
                </Button>
              </div>
            ))}
            <Button
              type="button"
              variant="outline"
              size="sm"
              className="justify-self-start"
              onClick={() =>
                set('mounts', [...form.mounts, { source: '', target: '', readOnly: false }])
              }
            >
              <PlusIcon /> Add volume
            </Button>
          </div>

          <div className="grid gap-1.5">
            <Label htmlFor="run-env">Environment</Label>
            <Textarea
              id="run-env"
              value={form.env}
              onChange={(e) => set('env', e.target.value)}
              placeholder={'KEY=value\nOTHER=value'}
              className="h-20 font-mono text-xs"
              spellCheck={false}
            />
          </div>

          <div className="grid grid-cols-2 gap-3">
            <div className="grid gap-1.5">
              <Label>Network</Label>
              <Select
                value={form.network || DEFAULT_NETWORK}
                onValueChange={(v) => set('network', v === DEFAULT_NETWORK ? '' : v)}
              >
                <SelectTrigger aria-label="Network">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  <SelectItem value={DEFAULT_NETWORK}>Default (bridge)</SelectItem>
                  {networks.data
                    ?.filter((n) => n.name !== 'bridge')
                    .map((n) => (
                      <SelectItem key={n.id} value={n.name}>
                        {n.name}
                      </SelectItem>
                    ))}
                </SelectContent>
              </Select>
            </div>
            <div className="grid gap-1.5">
              <Label>Restart</Label>
              <Select
                value={form.restart}
                onValueChange={(v) => set('restart', v as RestartPolicy)}
                disabled={form.autoRemove}
              >
                <SelectTrigger aria-label="Restart policy">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent>
                  {RESTART.map((r) => (
                    <SelectItem key={r.value} value={r.value}>
                      {r.label}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
          </div>
          <div className="flex items-center gap-2">
            <Switch
              id="run-rm"
              checked={form.autoRemove}
              onCheckedChange={(v) =>
                setForm((f) => ({ ...f, autoRemove: v, restart: v ? 'no' : f.restart }))
              }
            />
            <Label htmlFor="run-rm">Remove the container when it exits</Label>
          </div>

          {run.error && <ErrorMessage error={run.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button type="submit" disabled={run.isPending || !form.image.trim()}>
              {run.isPending ? 'Starting…' : 'Run'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
