import type { Registry, RegistryDraft } from '@/shared/api';

/** What the add/edit dialog holds. An empty secret keeps the stored one when editing. */
export interface LoginForm {
  address: string;
  username: string;
  secret: string;
}

export const emptyLogin: LoginForm = { address: '', username: '', secret: '' };

/** The form to edit a stored login: its secret stays on the backend. */
export const loginFormOf = (registry: Registry): LoginForm => ({
  address: registry.address,
  username: registry.username,
  secret: '',
});

/** What's still missing before the form can be checked or saved; `null` when complete. */
export function missingField(form: LoginForm, editing: boolean): string | null {
  if (!form.address.trim()) return 'Enter the registry’s address.';
  if (!form.username.trim()) return 'Enter the user name.';
  if (!editing && !form.secret) return 'Enter the password or access token.';
  return null;
}

/** The draft sent to the backend; `secret: null` keeps the stored one. */
export const toDraft = (form: LoginForm): RegistryDraft => ({
  address: form.address.trim(),
  username: form.username.trim(),
  secret: form.secret === '' ? null : form.secret,
});

/** How an image is pulled or pushed: "as alice on ghcr.io", or anonymously. */
export function describeLogin(login: Registry | null | undefined): string {
  return login ? `as ${login.username} on ${login.address}` : 'anonymously';
}

/** Docker Hub's logins are stored as docker.io; say what that is. */
export const registryLabel = (address: string) =>
  address === 'docker.io' ? 'Docker Hub (docker.io)' : address;

/**
 * The registry an image reference is pulled from or pushed to, by Docker's
 * rule (the same as the backend's): a first path component with a dot, a
 * port or `localhost` names it, otherwise Docker Hub.
 */
export function registryOf(reference: string): string {
  const [first, ...rest] = reference.trim().split('/');
  if (rest.length > 0 && first && (/[.:]/.test(first) || first === 'localhost')) {
    const host = first.toLowerCase();
    return ['index.docker.io', 'registry-1.docker.io'].includes(host) ? 'docker.io' : host;
  }
  return 'docker.io';
}
