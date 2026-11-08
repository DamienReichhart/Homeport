import { describe, expect, it } from 'vitest';
import type { IncusProject } from '@/shared/api';
import { chosenIncusProject, preferredProject } from './incus';

const project = (name: string, instances = 0): IncusProject => ({
  name,
  description: '',
  instances,
  images: 0,
});

describe('preferredProject', () => {
  it('shows default when the connection may use it', () => {
    expect(preferredProject([project('default'), project('lab', 3)])).toBe('default');
    expect(preferredProject([])).toBe('default');
  });

  it('shows the restricted socket’s own project otherwise', () => {
    expect(preferredProject([project('user-1000', 2)])).toBe('user-1000');
  });
});

describe('chosenIncusProject', () => {
  it('keeps the remembered project while the Host has it', () => {
    expect(chosenIncusProject('h1', { h1: 'lab' }, [project('default'), project('lab')])).toBe(
      'lab',
    );
    expect(chosenIncusProject('h1', { h1: 'lab' }, undefined)).toBe('lab');
  });

  it('falls back when the remembered project is gone, and waits for the list', () => {
    expect(chosenIncusProject('h1', { h1: 'gone' }, [project('default')])).toBe('default');
    expect(chosenIncusProject('h1', {}, undefined)).toBeNull();
  });
});
