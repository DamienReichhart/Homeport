import { createElement, type ComponentType } from 'react';
import type { RouteObject } from 'react-router-dom';
import { SplitView } from '@/shared/ui';

/**
 * A list page at `path` whose items, at the `items` paths below it, open
 * beside the list on a wide window and in its place on a narrow one.
 */
export function splitRoutes(path: string, List: ComponentType, items: RouteObject[]): RouteObject {
  return {
    path,
    element: createElement(SplitView, {
      list: createElement(List),
      listParams: [...path.matchAll(/:(\w+)/g)].map((m) => m[1] ?? ''),
    }),
    children: [{ index: true, element: null }, ...items],
  };
}
