import { useQuery } from '@tanstack/react-query';
import { useState, type ReactNode } from 'react';
import { resourceQueries, supports, useHostSelection } from '@/core/host';
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
} from '@/shared/ui';
import { useRunContainer } from '../../api/mutations';
import { hasAdvanced } from '../../model/fromRunSpec';
import { emptyRunForm, runFormErrors, type RunForm } from '../../model/runForm';
import { RunFormFields } from './run/RunFormFields';
import { t } from '@/shared/i18n';

/**
 * The Run dialog: create and start a container, pulling the image if needed.
 * `initial` fills it in (Duplicate); `notice` shows above the buttons. On a
 * Podman Host a new container can go into a pod, `pod` when given.
 */
export function RunContainerDialog({
  hostId,
  open,
  onOpenChange,
  image = '',
  initial,
  pod = '',
  title = t('containers.run.title'),
  notice,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  image?: string;
  initial?: RunForm;
  pod?: string;
  title?: string;
  notice?: (form: RunForm) => ReactNode;
}) {
  const navigate = useNavigate();
  const { host } = useHostSelection();
  const fresh = () => ({ ...emptyRunForm(image), pod });
  const [form, setForm] = useState<RunForm>(() => initial ?? fresh());
  const run = useRunContainer(hostId, (id) => {
    onOpenChange(false);
    setForm(fresh());
    navigate(`/containers/${id}/logs`);
  });
  // A copy keeps its original's settings, which a pod can't take.
  const podman = supports(host, 'podman') && !initial?.basedOn;
  const pods = useQuery({ ...resourceQueries.pods(hostId), enabled: podman && open });
  const errors = runFormErrors(form);
  const invalid = Object.keys(errors).length > 0;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{t('containers.run.description')}</DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (!invalid) run.mutate(form);
          }}
        >
          {open && (
            <RunFormFields
              hostId={hostId}
              form={form}
              setForm={setForm}
              errors={errors}
              advancedOpen={initial ? hasAdvanced(initial) : false}
              pods={podman ? (pods.data ?? []) : null}
            />
          )}
          {notice?.(form)}
          {run.error && <ErrorMessage error={run.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              {t('common.cancel')}
            </Button>
            <Button type="submit" disabled={run.isPending || !form.image.trim() || invalid}>
              {run.isPending ? t('containers.run.starting') : t('containers.run.submit')}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
