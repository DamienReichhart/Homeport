import { useState, type ReactNode } from 'react';
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

/**
 * The Run dialog: create and start a container, pulling the image if needed.
 * `initial` fills it in (Duplicate); `notice` shows above the buttons.
 */
export function RunContainerDialog({
  hostId,
  open,
  onOpenChange,
  image = '',
  initial,
  title = 'Run a container',
  notice,
}: {
  hostId: string;
  open: boolean;
  onOpenChange: (open: boolean) => void;
  image?: string;
  initial?: RunForm;
  title?: string;
  notice?: (form: RunForm) => ReactNode;
}) {
  const navigate = useNavigate();
  const [form, setForm] = useState<RunForm>(() => initial ?? emptyRunForm(image));
  const run = useRunContainer(hostId, (id) => {
    onOpenChange(false);
    setForm(emptyRunForm());
    navigate(`/containers/${id}/logs`);
  });
  const errors = runFormErrors(form);
  const invalid = Object.keys(errors).length > 0;

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>
            The image is pulled first if this host doesn&apos;t have it.
          </DialogDescription>
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
            />
          )}
          {notice?.(form)}
          {run.error && <ErrorMessage error={run.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={() => onOpenChange(false)}>
              Cancel
            </Button>
            <Button type="submit" disabled={run.isPending || !form.image.trim() || invalid}>
              {run.isPending ? 'Starting…' : 'Run'}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
