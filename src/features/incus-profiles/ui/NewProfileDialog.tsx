import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
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
import { useCreateProfile } from '../api/mutations';
import { t } from '@/shared/i18n';

/** Makes an empty profile, then opens it to fill in. */
export function NewProfileDialog({
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
  const navigate = useNavigate();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const create = useCreateProfile(hostId, project);
  const close = () => {
    setName('');
    setDescription('');
    create.reset();
    onClose();
  };
  return (
    <Dialog open={open} onOpenChange={(next) => !next && close()}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>{t('incus.profiles.new.title')}</DialogTitle>
          <DialogDescription>{t('incus.profiles.new.description', { project })}</DialogDescription>
        </DialogHeader>
        <div className="grid gap-3">
          <div className="grid gap-1.5">
            <Label htmlFor="profile-name">{t('incus.profiles.new.name')}</Label>
            <Input
              id="profile-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              autoFocus
            />
          </div>
          <div className="grid gap-1.5">
            <Label htmlFor="profile-description">{t('incus.profiles.new.descriptionField')}</Label>
            <Input
              id="profile-description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
            />
          </div>
          {create.error && <ErrorMessage error={create.error} />}
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={close}>
            {t('common.cancel')}
          </Button>
          <Button
            disabled={!name.trim() || create.isPending}
            onClick={() =>
              create
                .mutateAsync({ name: name.trim(), description })
                .then(() => {
                  const made = name.trim();
                  close();
                  void navigate(`/incus/profiles/${encodeURIComponent(made)}`);
                })
                .catch(() => {})
            }
          >
            {t('common.create')}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
