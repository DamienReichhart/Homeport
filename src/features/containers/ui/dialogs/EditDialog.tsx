import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import type { Container, RunSpec } from '@/shared/api';
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
import { useRecreateContainer } from '../../api/mutations';
import { containerQueries } from '../../api/queries';
import { fromRunSpec, hasAdvanced } from '../../model/fromRunSpec';
import { runFormErrors, type RunForm } from '../../model/runForm';
import { EditConfirmation } from './EditConfirmation';
import { LoadingDialog } from './LoadingDialog';
import { RunFormFields } from './run/RunFormFields';

function EditForm({
  hostId,
  container,
  spec,
  onClose,
}: {
  hostId: string;
  container: Container;
  spec: RunSpec;
  onClose: () => void;
}) {
  const navigate = useNavigate();
  const [initial] = useState(() => ({ ...fromRunSpec(spec), name: container.name }));
  const [form, setForm] = useState<RunForm>(initial);
  const [confirming, setConfirming] = useState(false);
  const recreate = useRecreateContainer(hostId, container.id, (id) => {
    onClose();
    navigate(`/containers/${id}/logs`, { replace: true });
  });
  const errors = runFormErrors(form);
  const invalid = Object.keys(errors).length > 0;

  return (
    <Dialog open onOpenChange={(open) => !open && !recreate.isPending && onClose()}>
      <DialogContent className="max-h-[90vh] overflow-y-auto sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Edit {container.name}</DialogTitle>
          <DialogDescription>
            Applying recreates the container with these settings, keeping its name, networks and
            named volumes.
          </DialogDescription>
        </DialogHeader>
        <form
          className="grid gap-4"
          onSubmit={(e) => {
            e.preventDefault();
            if (!invalid) setConfirming(true);
          }}
        >
          <RunFormFields
            hostId={hostId}
            form={form}
            setForm={setForm}
            errors={errors}
            advancedOpen={hasAdvanced(initial)}
            lockName
          />
          {recreate.error && <ErrorMessage error={recreate.error} />}
          <DialogFooter>
            <Button type="button" variant="ghost" onClick={onClose} disabled={recreate.isPending}>
              Cancel
            </Button>
            <Button type="submit" disabled={recreate.isPending || !form.image.trim() || invalid}>
              {recreate.isPending ? 'Recreating…' : 'Apply'}
            </Button>
          </DialogFooter>
        </form>
        {confirming && (
          <EditConfirmation
            hostId={hostId}
            container={container}
            // A failure shows in the form above, so the confirmation closes either way.
            onConfirm={() =>
              recreate.mutateAsync(form).then(
                () => {},
                () => {},
              )
            }
            onClose={() => setConfirming(false)}
          />
        )}
      </DialogContent>
    </Dialog>
  );
}

/** Edit: the Run dialog filled in from a container, applied by recreating it. */
export function EditDialog({
  hostId,
  container,
  onClose,
}: {
  hostId: string;
  container: Container;
  onClose: () => void;
}) {
  return (
    <LoadingDialog
      title={`Edit ${container.name}`}
      query={containerQueries.runSpec(hostId, container.id)}
      onClose={onClose}
    >
      {(spec) => <EditForm hostId={hostId} container={container} spec={spec} onClose={onClose} />}
    </LoadingDialog>
  );
}
