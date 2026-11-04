# Frontend in four layers, with one fixed layout per feature

The React code is split into `app` → `features` → `core` → `shared`, and a module only imports from the layers to its right. `app` is the shell: providers, layout and a router assembled from the features' manifests. `features` are vertical slices; each one has the same layout (`api/` for queries and mutations, `model/` for pure logic, `ui/` for components, `manifest.ts` for its routes and navigation entry, `index.ts` for its public API), and its components never call IPC commands directly. `core` is the domain every feature stands on: the Host (selection, status, the gate in front of Host-scoped pages), the query keys and lists of the Engine resources a Host has, the live updates that keep them fresh, and the user's settings. `shared` has no domain knowledge: the IPC gateway, the UI kit and generic helpers.

We considered Feature-Sliced Design and grouping features into domain modules. FSD spreads one concept (a Stack) over five layers, which breaks the vertical slices, and costs a lot for an app this size. Domain modules need groupings ("workloads", "resources") that aren't in the glossary and would be guessed too early.

## Consequences

- Adding a feature means a folder and one line in `src/app/router/features.ts`; deleting one is the reverse.
- The lists of Engine resources (containers, images, volumes, networks, Stacks, system usage, events) live in `core/host`, not in their features, because several features read them (running a container lists images and networks; a network lists containers) and Engine events invalidate them. Reading them from features would create import cycles.
- Features import each other only through `index.ts`; ESLint rejects deep imports and import cycles, and warns past 300 lines in a file.
