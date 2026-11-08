import { useQueryClient } from '@tanstack/react-query';
import { PlugIcon, RotateCwIcon } from 'lucide-react';
import { useState } from 'react';
import { Button, Input, Label, terminalError, XTerm, type XTermHandle } from '@/shared/ui';
import { incusInstanceQueries } from '../api/queries';
import { startIncusConsole, startIncusExec, type IncusTerminalSession } from '../api/streams';
import { t } from '@/shared/i18n';

/** Writes a terminal's events to it, and wires its input and size to `session`. */
function wire(
  { terminal }: XTermHandle,
  start: (handlers: Parameters<typeof startIncusExec>[4]) => IncusTerminalSession,
  ended: (code: number | null) => string,
) {
  const session = start({
    onEvent: (event) => {
      if (event.type === 'output') terminal.write(event.data);
      else if (event.type === 'exit') {
        terminal.write(`\r\n\x1b[2m${ended(event.code)}\x1b[22m\r\n`);
        if (event.error) terminal.write(terminalError(event.error));
      } else terminal.write(`\r\n${terminalError(event.error)}`);
    },
    onOpen: () => terminal.focus(),
    onError: (error) => terminal.write(terminalError(error)),
  });
  const input = terminal.onData((data) => session.input(data));
  const resize = terminal.onResize(({ cols, rows }) => session.resize(cols, rows));
  return () => {
    input.dispose();
    resize.dispose();
    session.close();
  };
}

interface Spec {
  key: number;
  command: string;
  user: string;
  cwd: string;
}

/**
 * A shell, or a command of the user's, in a running Instance, through
 * Incus's exec. Virtual machines need their Incus agent for it.
 */
export function ExecTab({
  hostId,
  project,
  name,
  running,
}: {
  hostId: string;
  project: string;
  name: string;
  running: boolean;
}) {
  const [draft, setDraft] = useState({ command: '', user: '', cwd: '' });
  const [spec, setSpec] = useState<Spec>({ key: 1, command: '', user: '', cwd: '' });
  if (!running)
    return <p className="text-muted-foreground text-sm">{t('incus.exec.startFirst')}</p>;
  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <form
        className="flex flex-wrap items-end gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          setSpec({ key: spec.key + 1, ...draft });
        }}
      >
        <div className="grid min-w-48 flex-1 gap-1">
          <Label htmlFor="exec-command">{t('incus.exec.command')}</Label>
          <Input
            id="exec-command"
            className="font-mono"
            placeholder={t('incus.exec.commandPlaceholder')}
            value={draft.command}
            onChange={(e) => setDraft({ ...draft, command: e.target.value })}
          />
        </div>
        <div className="grid w-24 gap-1">
          <Label htmlFor="exec-user">{t('incus.exec.user')}</Label>
          <Input
            id="exec-user"
            className="font-mono"
            inputMode="numeric"
            placeholder="0"
            value={draft.user}
            onChange={(e) => setDraft({ ...draft, user: e.target.value })}
          />
        </div>
        <div className="grid w-36 gap-1">
          <Label htmlFor="exec-cwd">{t('incus.exec.cwd')}</Label>
          <Input
            id="exec-cwd"
            className="font-mono"
            placeholder="/root"
            value={draft.cwd}
            onChange={(e) => setDraft({ ...draft, cwd: e.target.value })}
          />
        </div>
        <Button type="submit" variant="outline">
          <RotateCwIcon /> {t('incus.exec.open')}
        </Button>
      </form>
      <div className="min-h-[20rem] flex-1">
        <XTerm
          key={spec.key}
          className="h-full"
          onReady={(handle) =>
            wire(
              handle,
              (handlers) =>
                startIncusExec(
                  hostId,
                  project,
                  name,
                  { ...spec, cols: handle.terminal.cols, rows: handle.terminal.rows },
                  handlers,
                ),
              (code) =>
                code === null ? t('incus.exec.exited') : t('incus.exec.exitedWithCode', { code }),
            )
          }
        />
      </div>
    </div>
  );
}

/**
 * The Instance's text console: what it printed so far, then attached, so
 * a login prompt or a boot can be watched. Closing the tab only detaches.
 */
export function ConsoleTab({
  hostId,
  project,
  name,
  running,
}: {
  hostId: string;
  project: string;
  name: string;
  running: boolean;
}) {
  const client = useQueryClient();
  const [key, setKey] = useState(1);
  if (!running)
    return <p className="text-muted-foreground text-sm">{t('incus.console.startFirst')}</p>;
  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="flex items-center justify-between gap-2">
        <p className="text-muted-foreground text-xs">{t('incus.console.hint')}</p>
        <Button variant="outline" size="sm" onClick={() => setKey(key + 1)}>
          <PlugIcon /> {t('incus.console.reattach')}
        </Button>
      </div>
      <div className="min-h-[20rem] flex-1">
        <XTerm
          key={key}
          className="h-full"
          onReady={(handle) => {
            let stop: (() => void) | null = null;
            let gone = false;
            // The log first, then what comes live.
            void client
              .fetchQuery(incusInstanceQueries.consoleLog(hostId, project, name))
              .catch(() => '')
              .then((log) => {
                if (gone) return;
                if (log) handle.terminal.write(log.replace(/\r?\n/g, '\r\n'));
                stop = wire(
                  handle,
                  (handlers) =>
                    startIncusConsole(
                      hostId,
                      project,
                      name,
                      { cols: handle.terminal.cols, rows: handle.terminal.rows },
                      handlers,
                    ),
                  () => t('incus.console.detached'),
                );
              });
            return () => {
              gone = true;
              stop?.();
            };
          }}
        />
      </div>
    </div>
  );
}
