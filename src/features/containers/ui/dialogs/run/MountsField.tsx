import { PlusIcon, XIcon } from 'lucide-react';
import { Button, Checkbox, Input, Label } from '@/shared/ui';
import type { MountRow } from '../../../model/runForm';

/** The volumes and bind mounts of the Run dialog, one row each. */
export function MountsField({
  mounts,
  onChange,
}: {
  mounts: MountRow[];
  onChange: (mounts: MountRow[]) => void;
}) {
  const update = (i: number, patch: Partial<MountRow>) =>
    onChange(mounts.map((x, j) => (j === i ? { ...x, ...patch } : x)));
  return (
    <div className="grid gap-1.5">
      <Label>Volumes</Label>
      {mounts.map((m, i) => (
        <div key={i} className="flex items-center gap-2">
          <Input
            aria-label="Source"
            placeholder="Volume name or /host/path"
            value={m.source}
            onChange={(e) => update(i, { source: e.target.value })}
          />
          <span className="text-muted-foreground">→</span>
          <Input
            aria-label="Container path"
            placeholder="/container/path"
            value={m.target}
            onChange={(e) => update(i, { target: e.target.value })}
          />
          <label className="flex items-center gap-1 text-sm whitespace-nowrap">
            <Checkbox
              checked={m.readOnly}
              onCheckedChange={(v) => update(i, { readOnly: v === true })}
            />
            read-only
          </label>
          <Button
            type="button"
            variant="ghost"
            size="icon-sm"
            aria-label="Remove volume"
            onClick={() => onChange(mounts.filter((_, j) => j !== i))}
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
        onClick={() => onChange([...mounts, { source: '', target: '', readOnly: false }])}
      >
        <PlusIcon /> Add volume
      </Button>
    </div>
  );
}
