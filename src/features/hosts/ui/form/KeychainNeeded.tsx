import { TriangleAlertIcon } from 'lucide-react';
import { Alert, AlertDescription, AlertTitle } from '@/shared/ui';

/** Why keys and certificates can't be entered on a machine without an OS keychain. */
export function KeychainNeeded() {
  return (
    <Alert variant="destructive">
      <TriangleAlertIcon />
      <AlertTitle>No OS keychain</AlertTitle>
      <AlertDescription>
        Keys and certificates are encrypted with a key kept in the OS keychain, and none is
        available. Use ssh-agent, a local socket or TCP instead.
      </AlertDescription>
    </Alert>
  );
}
