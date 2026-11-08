import { PlugIcon } from 'lucide-react';
import { useState } from 'react';
import type { PodDetail } from '@/shared/api';
import { Button, Input, terminalError, XTerm } from '@/shared/ui';
import { startPodTerminal } from '../../api/streams';
import { defaultContainer } from '../../model/containers';
import { ContainerSelect } from './ContainerSelect';
import { t } from '@/shared/i18n';

/** A terminal in one of a Pod's running containers: a shell, or the command typed. */
export function PodTerminal({ clusterId, pod }: { clusterId: string; pod: PodDetail }) {
  const { namespace, name } = pod.view;
  const containers = pod.containers.filter((c) => !c.init);
  const [container, setContainer] = useState(() => defaultContainer(pod));
  const [draft, setDraft] = useState('');
  // Each connection: what it runs, and a number that makes Reconnect start afresh.
  const [session, setSession] = useState({ n: 1, container, command: '' });
  const [ended, setEnded] = useState(false);
  const connect = () => {
    setEnded(false);
    setSession((s) => ({ n: s.n + 1, container, command: draft.trim() }));
  };
  const running = pod.containers.find((c) => c.name === session.container)?.state === 'running';
  return (
    <div className="flex h-[60vh] flex-col gap-2">
      <form
        className="flex flex-wrap items-center gap-2"
        onSubmit={(e) => {
          e.preventDefault();
          connect();
        }}
      >
        {containers.length > 1 && (
          <ContainerSelect containers={containers} value={container} onChange={setContainer} />
        )}
        <Input
          className="h-8 w-72 font-mono"
          placeholder={t('pods.terminal.shell')}
          aria-label={t('pods.terminal.command')}
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
        />
        <Button type="submit" size="sm" variant={ended ? 'default' : 'outline'}>
          <PlugIcon /> {ended ? t('pods.terminal.reconnect') : t('pods.terminal.connect')}
        </Button>
        {!running && (
          <span className="text-muted-foreground text-sm">{t('pods.terminal.notRunning')}</span>
        )}
      </form>
      <div className="min-h-0 flex-1">
        <XTerm
          key={session.n}
          className="h-full"
          onReady={({ terminal }) => {
            const open = startPodTerminal(
              clusterId,
              namespace,
              name,
              {
                container: session.container,
                command: session.command,
                cols: terminal.cols,
                rows: terminal.rows,
              },
              {
                onEvent: (event) => {
                  if (event.type === 'output') terminal.write(event.data);
                  else if (event.type === 'exit') {
                    const exited =
                      event.code === null
                        ? t('pods.terminal.exited')
                        : t('pods.terminal.exitedWithCode', { code: event.code });
                    terminal.write(`\r\n\x1b[2m${exited}\x1b[22m\r\n`);
                    if (event.error) terminal.write(terminalError(event.error));
                    setEnded(true);
                  } else {
                    terminal.write(`\r\n${terminalError(event.error)}`);
                    setEnded(true);
                  }
                },
                onOpen: () => terminal.focus(),
                onError: (error) => {
                  terminal.write(terminalError(error));
                  setEnded(true);
                },
              },
            );
            const input = terminal.onData((data) => open.input(data));
            const resize = terminal.onResize(({ cols, rows }) => open.resize(cols, rows));
            return () => {
              input.dispose();
              resize.dispose();
              open.close();
            };
          }}
        />
      </div>
    </div>
  );
}
