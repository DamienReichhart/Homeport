import { useState } from 'react';
import {
  Button,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  Input,
  Label,
  Textarea,
} from '@/shared/ui';
import { importName } from '../../model/transfer';
import { t } from '@/shared/i18n';

/** Names the image a root filesystem tar becomes, before importing it. */
export function ImportDialog({
  path,
  onImport,
  onClose,
}: {
  path: string;
  onImport: (reference: string, changes: string[]) => void;
  onClose: () => void;
}) {
  const [reference, setReference] = useState(() => importName(path));
  const [changes, setChanges] = useState('');
  return (
    <Dialog open onOpenChange={(open) => !open && onClose()}>
      <DialogContent className="sm:max-w-lg">
        <DialogHeader>
          <DialogTitle>{t('images.importDialog.title')}</DialogTitle>
          <DialogDescription className="break-all">
            {t('images.importDialog.description', { path })}
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            onImport(
              reference.trim(),
              changes
                .split('\n')
                .map((l) => l.trim())
                .filter(Boolean),
            );
          }}
        >
          <div className="grid gap-1.5">
            <Label htmlFor="import-name">{t('images.importDialog.name')}</Label>
            <Input
              id="import-name"
              value={reference}
              onChange={(e) => setReference(e.target.value)}
              className="font-mono"
              autoFocus
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="import-changes">{t('images.importDialog.changes')}</Label>
            <Textarea
              id="import-changes"
              value={changes}
              onChange={(e) => setChanges(e.target.value)}
              // eslint-disable-next-line homeport/no-literal-ui-string -- Dockerfile instructions
              placeholder={'CMD ["/bin/sh"]\nENV LANG=C.UTF-8'}
              className="h-20 font-mono text-xs"
              spellCheck={false}
            />
            <p className="text-muted-foreground text-xs">{t('images.importDialog.changesHint')}</p>
          </div>
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={!reference.trim()}>
              {t('images.importDialog.import')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
