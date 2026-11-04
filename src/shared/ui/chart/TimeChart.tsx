import { useEffect, useRef } from 'react';
import uPlot, { type AlignedData, type Options } from 'uplot';
import 'uplot/dist/uPlot.min.css';
import { useResolvedTheme } from '@/shared/lib/theme';

export interface ChartSeries {
  label: string;
  /** A CSS colour; theme tokens like `var(--chart-1)` are resolved. */
  colour: string;
  /** Draw the line dashed, to tell a second series apart without another colour. */
  dashed?: boolean;
}

function resolveColour(colour: string): string {
  const match = /^var\((--[\w-]+)\)$/.exec(colour);
  if (!match?.[1]) return colour;
  return getComputedStyle(document.documentElement).getPropertyValue(match[1]).trim() || '#888';
}

/**
 * A small live line chart over time. `data[0]` holds Unix seconds, the rest
 * one array per series.
 */
export function TimeChart({
  data,
  series,
  format,
  height = 160,
}: {
  data: AlignedData;
  series: ChartSeries[];
  format: (value: number) => string;
  height?: number;
}) {
  const host = useRef<HTMLDivElement>(null);
  const chart = useRef<uPlot | null>(null);
  const theme = useResolvedTheme();
  const seriesKey = JSON.stringify(series);
  const formatRef = useRef(format);

  useEffect(() => {
    formatRef.current = format;
  });

  useEffect(() => {
    const el = host.current;
    if (!el) return;
    const axis = resolveColour('var(--muted-foreground)');
    const grid = resolveColour('var(--border)');
    const parsed = JSON.parse(seriesKey) as ChartSeries[];
    const options: Options = {
      width: el.clientWidth || 400,
      height,
      legend: { show: true },
      cursor: { drag: { x: false, y: false } },
      scales: { x: { time: true }, y: { range: (_u, _min, max) => [0, max > 0 ? max * 1.1 : 1] } },
      axes: [
        { stroke: axis, grid: { stroke: grid }, ticks: { stroke: grid } },
        {
          stroke: axis,
          grid: { stroke: grid },
          ticks: { stroke: grid },
          size: 70,
          values: (_u, values) => values.map((v) => formatRef.current(v)),
        },
      ],
      series: [
        {},
        ...parsed.map((s) => ({
          label: s.label,
          stroke: resolveColour(s.colour),
          width: 1.5,
          ...(s.dashed ? { dash: [5, 3] } : {}),
          points: { show: false },
          value: (_u: uPlot, v: number | null) => (v === null ? '—' : formatRef.current(v)),
        })),
      ],
    };
    const u = new uPlot(options, [[], ...parsed.map(() => [])], el);
    chart.current = u;
    const observer = new ResizeObserver(() => u.setSize({ width: el.clientWidth, height }));
    observer.observe(el);
    return () => {
      observer.disconnect();
      u.destroy();
      chart.current = null;
    };
  }, [seriesKey, height, theme]);

  useEffect(() => {
    chart.current?.setData(data);
  }, [data]);

  return <div ref={host} className="w-full" />;
}
