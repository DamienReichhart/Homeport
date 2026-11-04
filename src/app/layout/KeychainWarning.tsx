import { TriangleAlertIcon } from 'lucide-react';
import { useCapabilities } from '@/shared/api';

/** Says what can't be stored when the machine has no OS keychain. */
export function KeychainWarning() {
  const capabilities = useCapabilities();
  if (!capabilities.data || capabilities.data.keychain) return null;
  return (
    <div
      role="alert"
      className="bg-destructive/10 text-destructive flex items-center gap-2 border-b px-6 py-2 text-sm"
    >
      <TriangleAlertIcon className="size-4" />
      No OS keychain is available, so SSH keys, TLS certificates and stack environments can’t be
      stored. Local, TCP and ssh-agent hosts still work.
    </div>
  );
}
