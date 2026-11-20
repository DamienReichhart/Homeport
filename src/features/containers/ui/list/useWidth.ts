import { useEffect, useState } from 'react';

/**
 * A ref callback and the width of the element it's given, kept current as it
 * resizes: 0 until measured, or without ResizeObserver. A callback, not a ref
 * object, so it measures the element whenever it appears, after a spinner.
 */
export function useWidth(): [(element: HTMLElement | null) => void, number] {
  const [element, setElement] = useState<HTMLElement | null>(null);
  const [width, setWidth] = useState(0);
  useEffect(() => {
    if (!element || typeof ResizeObserver === 'undefined') return;
    const observer = new ResizeObserver(([entry]) => {
      if (entry) setWidth(Math.round(entry.contentRect.width));
    });
    observer.observe(element);
    return () => observer.disconnect();
  }, [element]);
  return [setElement, width];
}
