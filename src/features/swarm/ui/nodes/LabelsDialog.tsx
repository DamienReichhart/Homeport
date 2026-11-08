import { PlusIcon, Trash2Icon } from 'lucide-react';
import { useState } from 'react';
import type { SwarmNode } from '@/shared/api';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
} from '@/shared/ui';
import { labelMap, labelRows, type LabelRow } from '../../model/labels';
import { t } from '@/shared/i18n';

/** Edits a node's labels as key/value rows. */
export function LabelsDialog({
  node,
  onSave,
  onClose,
}: {
  node: SwarmNode;
  onSave: (labels: Record<string, string>) => Promise<unknown>;
  onClose: () => void;
}) {
  const [rows, setRows] = useState<LabelRow[]>(() => labelRows(node.labels));
  const [saving, setSaving] = useState(false);
  const set = (index: number, row: Partial<LabelRow>) =>
    setRows((all) => all.map((r, i) => (i === index ? { ...r, ...row } : r)));
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t('swarm.nodes.labels.title', { name: node.hostname })}</DialogTitle>
          <DialogDescription>{t('swarm.nodes.labels.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-2"
          onSubmit={(e) => {
            e.preventDefault();
            setSaving(true);
            onSave(labelMap(rows))
              .then(onClose)
              .catch(() => setSaving(false));
          }}
        >
          {rows.map((row, index) => (
            <div key={index} className="flex gap-2">
              <Input
                aria-label={t('swarm.nodes.labels.key')}
                placeholder={t('swarm.nodes.labels.key')}
                value={row.key}
                onChange={(e) => set(index, { key: e.target.value })}
              />
              <Input
                aria-label={t('swarm.nodes.labels.value')}
                placeholder={t('swarm.nodes.labels.value')}
                value={row.value}
                onChange={(e) => set(index, { value: e.target.value })}
              />
              <Button
                type="button"
                variant="ghost"
                size="icon"
                aria-label={t('swarm.nodes.labels.removeOne', { key: row.key })}
                onClick={() => setRows((all) => all.filter((_, i) => i !== index))}
              >
                <Trash2Icon />
              </Button>
            </div>
          ))}
          <Button
            type="button"
            variant="outline"
            className="justify-self-start"
            onClick={() => setRows((all) => [...all, { key: '', value: '' }])}
          >
            <PlusIcon /> {t('swarm.nodes.labels.add')}
          </Button>
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={saving}>
              {t('common.save')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
