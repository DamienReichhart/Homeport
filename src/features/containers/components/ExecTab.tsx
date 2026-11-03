import { Channel } from '@tauri-apps/api/core';
import { PlusIcon, XIcon } from 'lucide-react';
import { useState } from 'react';
import { commands, toAppError, type ExecEvent } from '@/shared/api';
import { cn } from '@/shared/lib';
import { Button } from '@/shared/ui/button';
import { Input } from '@/shared/ui/input';
import { Label } from '@/shared/ui/label';
import { Popover, PopoverContent, PopoverTrigger } from '@/shared/ui/popover';
import { XTerm } from '@/shared/ui';

interface SessionSpec {
  key: number;
  title: string;
  command: string;
  user: string;
  workingDir: string;
}

/** One interactive exec session bound to a terminal; closed on unmount. */
function ExecTerminal({
  hostId,
  containerId,
  spec,
}: {
  hostId: string;
  containerId: string;
  spec: SessionSpec;
}) {
  return (
    <XTerm
      className="h-full"
      onReady={({ terminal }) => {
        let session: number | null = null;
        let disposed = false;
        let size = { cols: terminal.cols, rows: terminal.rows };
        const channel = new Channel<ExecEvent>();
        channel.onmessage = (event) => {
          if (event.type === 'output') terminal.write(event.data);
          else if (event.type === 'exit')
            terminal.write(
              `\r\n\x1b[2m[process exited${event.code === null ? '' : ` with code ${event.code}`}]\x1b[22m\r\n`,
            );
          else terminal.write(`\r\n\x1b[31m${event.message}\x1b[39m\r\n`);
        };
        commands
          .execStart(
            hostId,
            containerId,
            {
              command: spec.command || null,
              user: spec.user || null,
              workingDir: spec.workingDir || null,
              cols: size.cols,
              rows: size.rows,
            },
            channel,
          )
          .then((id) => {
            if (disposed) {
              void commands.execClose(id);
              return;
            }
            session = id;
            void commands.execResize(id, size.cols, size.rows);
            terminal.focus();
          })
          .catch((e: unknown) => terminal.write(`\x1b[31m${toAppError(e).message}\x1b[39m\r\n`));
        const input = terminal.onData((data) => {
          if (session !== null) void commands.execInput(session, data);
        });
        const resize = terminal.onResize(({ cols, rows }) => {
          size = { cols, rows };
          if (session !== null) void commands.execResize(session, cols, rows);
        });
        return () => {
          disposed = true;
          input.dispose();
          resize.dispose();
          channel.onmessage = () => {};
          if (session !== null) void commands.execClose(session);
        };
      }}
    />
  );
}

/** Terminals into a running container, several at once in tabs. */
export function ExecTab({
  hostId,
  containerId,
  running,
}: {
  hostId: string;
  containerId: string;
  running: boolean;
}) {
  const [sessions, setSessions] = useState<SessionSpec[]>([
    { key: 1, title: 'Shell', command: '', user: '', workingDir: '' },
  ]);
  const [active, setActive] = useState(1);
  const [draft, setDraft] = useState({ command: '', user: '', workingDir: '' });
  const [open, setOpen] = useState(false);

  if (!running)
    return <p className="text-muted-foreground text-sm">Start the container to open a terminal.</p>;

  const add = () => {
    const key = Math.max(0, ...sessions.map((s) => s.key)) + 1;
    setSessions((list) => [...list, { key, title: draft.command || `Shell ${key}`, ...draft }]);
    setActive(key);
    setOpen(false);
  };
  const close = (key: number) => {
    const rest = sessions.filter((s) => s.key !== key);
    setSessions(rest);
    if (active === key) setActive(rest.at(-1)?.key ?? 0);
  };

  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="flex items-center gap-1 border-b">
        {sessions.map((s) => (
          <div
            key={s.key}
            className={cn(
              '-mb-px flex items-center gap-1 border-b-2 px-2 py-1 text-sm',
              s.key === active ? 'border-primary' : 'text-muted-foreground border-transparent',
            )}
          >
            <button type="button" onClick={() => setActive(s.key)}>
              {s.title}
            </button>
            <button type="button" aria-label={`Close ${s.title}`} onClick={() => close(s.key)}>
              <XIcon className="size-3" />
            </button>
          </div>
        ))}
        <Popover open={open} onOpenChange={setOpen}>
          <PopoverTrigger asChild>
            <Button variant="ghost" size="sm">
              <PlusIcon /> New terminal
            </Button>
          </PopoverTrigger>
          <PopoverContent className="grid w-80 gap-3" align="start">
            <div className="grid gap-1.5">
              <Label htmlFor="exec-cmd">Command</Label>
              <Input
                id="exec-cmd"
                placeholder="bash, or sh when missing"
                value={draft.command}
                onChange={(e) => setDraft({ ...draft, command: e.target.value })}
                className="font-mono"
              />
            </div>
            <div className="grid grid-cols-2 gap-2">
              <div className="grid gap-1.5">
                <Label htmlFor="exec-user">User</Label>
                <Input
                  id="exec-user"
                  placeholder="default"
                  value={draft.user}
                  onChange={(e) => setDraft({ ...draft, user: e.target.value })}
                />
              </div>
              <div className="grid gap-1.5">
                <Label htmlFor="exec-wd">Directory</Label>
                <Input
                  id="exec-wd"
                  placeholder="default"
                  value={draft.workingDir}
                  onChange={(e) => setDraft({ ...draft, workingDir: e.target.value })}
                />
              </div>
            </div>
            <Button onClick={add}>Open</Button>
          </PopoverContent>
        </Popover>
      </div>
      <div className="relative min-h-0 flex-1">
        {sessions.map((s) => (
          <div key={s.key} className={cn('absolute inset-0', s.key !== active && 'invisible')}>
            <ExecTerminal hostId={hostId} containerId={containerId} spec={s} />
          </div>
        ))}
        {sessions.length === 0 && (
          <p className="text-muted-foreground text-sm">No terminal open.</p>
        )}
      </div>
    </div>
  );
}
