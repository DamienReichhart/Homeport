import type { Candidate, SourceKind } from '@/shared/api';
import { Dialog, DialogContent } from '@/shared/ui';
import { ReviewChanges } from './ReviewChanges';

/**
 * The review step on its own, for editors that save the reviewed
 * definition before deploying it. `candidate` null closes it.
 */
export function ReviewDialog({
  kind,
  stackId,
  name,
  candidate,
  onDeploy,
  onClose,
}: {
  kind: SourceKind;
  stackId: string;
  name: string;
  candidate: Candidate | null;
  onDeploy: () => void;
  onClose: () => void;
}) {
  if (!candidate) return null;
  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="sm:max-w-3xl">
        <ReviewChanges
          kind={kind}
          stackId={stackId}
          name={name}
          candidate={candidate}
          onDeploy={() => {
            onClose();
            onDeploy();
          }}
          onBack={onClose}
        />
      </DialogContent>
    </Dialog>
  );
}
