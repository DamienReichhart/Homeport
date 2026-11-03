import { useQuery } from '@tanstack/react-query';
import { commands, keys, toAppError } from '@/shared/api';
import { EmptyState, ErrorMessage, Spinner } from '@/shared/ui';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from '@/shared/ui/dialog';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/shared/ui/tabs';

const TEXT = 'bg-muted h-[60vh] overflow-auto rounded-md p-3 font-mono text-xs whitespace-pre-wrap';

/** Homeport's license and the notices of every package it bundles. */
export function LicensesDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const licenses = useQuery({
    queryKey: keys.licenses,
    queryFn: () => commands.getLicenses(),
    enabled: open,
    staleTime: Infinity,
  });

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-3xl">
        <DialogHeader>
          <DialogTitle>Licenses</DialogTitle>
          <DialogDescription>
            Homeport is free software under the MIT License, built on the open-source packages
            below.
          </DialogDescription>
        </DialogHeader>
        {licenses.isPending && <Spinner />}
        {licenses.error && <ErrorMessage error={toAppError(licenses.error)} />}
        {licenses.data && (
          <Tabs defaultValue="app" className="min-w-0">
            <TabsList>
              <TabsTrigger value="app">Homeport</TabsTrigger>
              <TabsTrigger value="third-party">Third-party software</TabsTrigger>
            </TabsList>
            <TabsContent value="app">
              <pre className={TEXT}>{licenses.data.app}</pre>
            </TabsContent>
            <TabsContent value="third-party">
              {licenses.data.third_party ? (
                <pre className={TEXT}>{licenses.data.third_party}</pre>
              ) : (
                <EmptyState
                  title="Not included in this build"
                  description={
                    <>
                      Release builds bundle them; run <code>bun run licenses</code> to generate them
                      for a development build.
                    </>
                  }
                />
              )}
            </TabsContent>
          </Tabs>
        )}
      </DialogContent>
    </Dialog>
  );
}
