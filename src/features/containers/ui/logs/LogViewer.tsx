import {
  ArrowDownToLineIcon,
  ChevronDownIcon,
  ChevronUpIcon,
  DownloadIcon,
  EraserIcon,
} from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import type { LogSource } from '@/shared/api';
import { pickSavePath } from '@/shared/lib';
import {
  Button,
  Input,
  SEARCH_DECORATIONS,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  terminalError,
  Toggle,
  XTerm,
  type XTermHandle,
} from '@/shared/ui';
import { useSaveLogs } from '../../api/mutations';
import { streamLogs } from '../../api/streams';
import { MAX_KEPT_LINES, renderLine, SINCE, TAILS } from '../../model/logs';

/**
 * Live logs of one or more containers. Several sources are interleaved with
 * a coloured label per source, like `docker compose logs`.
 */
export function LogViewer({ hostId, sources }: { hostId: string; sources: LogSource[] }) {
  const handle = useRef<XTermHandle | null>(null);
  const kept = useRef<string[]>([]);
  const [ready, setReady] = useState(false);
  const [tail, setTail] = useState('1000');
  const [since, setSince] = useState('all');
  const [timestamps, setTimestamps] = useState(false);
  const [stdout, setStdout] = useState(true);
  const [stderr, setStderr] = useState(true);
  const [follow, setFollow] = useState(true);
  const [query, setQuery] = useState('');
  const saveLogs = useSaveLogs();
  const followRef = useRef(follow);
  const sourcesKey = JSON.stringify(sources);

  useEffect(() => {
    followRef.current = follow;
    if (follow) handle.current?.terminal.scrollToBottom();
  }, [follow]);

  useEffect(() => {
    const term = handle.current?.terminal;
    if (!ready || !term) return;
    term.reset();
    kept.current = [];
    const parsed = JSON.parse(sourcesKey) as LogSource[];
    const labels = parsed.flatMap((s) => (s.label ? [s.label] : []));
    return streamLogs(
      hostId,
      parsed,
      {
        tail: tail === 'all' ? null : Number(tail),
        since: since === 'all' ? null : Math.floor(Date.now() / 1000) - Number(since),
        timestamps,
        stdout,
        stderr,
        follow: true,
      },
      (event) => {
        if (event.type === 'lines') {
          const rendered = event.lines.map((l) => renderLine(l, labels, timestamps));
          term.write(`${rendered.join('\r\n')}\r\n`);
          kept.current.push(
            ...event.lines.map((l) => (l.source ? `${l.source} | ${l.text}` : l.text)),
          );
          if (kept.current.length > MAX_KEPT_LINES)
            kept.current.splice(0, kept.current.length - MAX_KEPT_LINES);
          if (followRef.current) term.scrollToBottom();
        } else if (event.type === 'error') {
          term.write(`\r\n${terminalError(event.error)}`);
        } else {
          term.write('\r\n\x1b[2m— end of logs —\x1b[22m\r\n');
        }
      },
      (error) => term.write(terminalError(error)),
    );
  }, [ready, hostId, sourcesKey, tail, since, timestamps, stdout, stderr]);

  const find = (forward: boolean) => {
    const search = handle.current?.search;
    if (!search || !query) return;
    const options = { decorations: SEARCH_DECORATIONS };
    if (forward) search.findNext(query, options);
    else search.findPrevious(query, options);
  };

  return (
    <div className="flex h-full min-h-0 flex-col gap-2">
      <div className="flex flex-wrap items-center gap-2">
        <Select value={tail} onValueChange={setTail}>
          <SelectTrigger className="w-32" size="sm" aria-label="History">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {TAILS.map((t) => (
              <SelectItem key={t.value} value={t.value}>
                {t.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Select value={since} onValueChange={setSince}>
          <SelectTrigger className="w-32" size="sm" aria-label="Since">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {SINCE.map((s) => (
              <SelectItem key={s.value} value={s.value}>
                {s.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
        <Toggle size="sm" variant="outline" pressed={timestamps} onPressedChange={setTimestamps}>
          Timestamps
        </Toggle>
        <Toggle size="sm" variant="outline" pressed={stdout} onPressedChange={setStdout}>
          stdout
        </Toggle>
        <Toggle size="sm" variant="outline" pressed={stderr} onPressedChange={setStderr}>
          stderr
        </Toggle>
        <Toggle
          size="sm"
          variant="outline"
          pressed={follow}
          onPressedChange={setFollow}
          aria-label="Follow"
        >
          <ArrowDownToLineIcon /> Follow
        </Toggle>
        <div className="ml-auto flex items-center gap-1">
          <Input
            className="h-7 w-44"
            placeholder="Find in logs"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && find(!e.shiftKey)}
            aria-label="Find in logs"
          />
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Previous match"
            onClick={() => find(false)}
          >
            <ChevronUpIcon />
          </Button>
          <Button variant="ghost" size="icon-sm" aria-label="Next match" onClick={() => find(true)}>
            <ChevronDownIcon />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Clear"
            onClick={() => {
              handle.current?.terminal.clear();
              kept.current = [];
            }}
          >
            <EraserIcon />
          </Button>
          <Button
            variant="ghost"
            size="icon-sm"
            aria-label="Download logs"
            onClick={async () => {
              const path = await pickSavePath('Save logs', 'logs.txt');
              if (!path) return;
              saveLogs.mutate({ path, lines: [...kept.current] });
            }}
          >
            <DownloadIcon />
          </Button>
        </div>
      </div>
      <XTerm
        readOnly
        className="flex-1"
        onReady={(h) => {
          handle.current = h;
          setReady(true);
          return () => {
            handle.current = null;
            setReady(false);
          };
        }}
      />
    </div>
  );
}
