import type { ResourceKind, WorkloadKind } from '@/shared/api';

/** A workload kind as a resource kind, for watches, Events and YAML. */
export const kindResource = (kind: WorkloadKind): ResourceKind => kind;
