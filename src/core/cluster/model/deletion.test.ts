import { describe, expect, it } from 'vitest';
import { deleteConfirmation } from './deletion';

describe('deleteConfirmation', () => {
  it('names the kind, the object, its namespace and the cluster', () => {
    const c = deleteConfirmation('kind-dev', [
      { type: { kind: 'deployment' }, namespace: 'shop', name: 'api' },
    ]);
    expect(c.title).toBe('Delete Deployment api?');
    expect(c.description).toBe(
      'Deployment api will be deleted from the namespace shop on kind-dev.',
    );
    expect(c.consequences).toEqual(['Its Pods are deleted with it.']);
  });

  it('says a Pod with a controller comes back', () => {
    const c = deleteConfirmation('kind-dev', [
      {
        type: { kind: 'pod' },
        namespace: 'shop',
        name: 'api-7d9c-xr4np',
        owner: { kind: 'ReplicaSet', name: 'api-7d9c' },
      },
    ]);
    expect(c.consequences).toEqual([
      'ReplicaSet api-7d9c controls it, so it will start a new Pod in its place.',
    ]);
  });

  it('says everything in a Namespace goes with it', () => {
    const c = deleteConfirmation('prod', [
      {
        type: {
          api: {
            group: '',
            version: 'v1',
            kind: 'Namespace',
            plural: 'namespaces',
            namespaced: false,
          },
        },
        namespace: null,
        name: 'shop',
      },
    ]);
    expect(c.description).toBe('Namespace shop will be deleted from prod.');
    expect(c.consequences[0]).toMatch(/^Everything in the namespace goes with it/);
  });

  it('lists several objects with their namespace', () => {
    const c = deleteConfirmation('kind-dev', [
      { type: { kind: 'configMap' }, namespace: 'shop', name: 'a' },
      { type: { kind: 'configMap' }, namespace: 'shop', name: 'b' },
    ]);
    expect(c.title).toBe('Delete 2 ConfigMap objects?');
    expect(c.description).toBe('These will be deleted from the namespace shop on kind-dev: a, b.');
    expect(c.consequences).toEqual([]);
  });

  it('names each kind when they differ', () => {
    const c = deleteConfirmation('kind-dev', [
      { type: { kind: 'service' }, namespace: 'shop', name: 'api' },
      { type: { kind: 'ingress' }, namespace: 'web', name: 'api' },
    ]);
    expect(c.title).toBe('Delete 2 objects?');
    expect(c.description).toBe('These will be deleted from kind-dev: Service api, Ingress api.');
  });
});
