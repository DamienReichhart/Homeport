import { caddy } from './caddy';
import { empty } from './empty';
import { mariadb } from './mariadb';
import { postgres } from './postgres';
import { redis } from './redis';
import { staticSite } from './staticSite';
import type { StackTemplate } from './template';
import { uptimeKuma } from './uptimeKuma';
import { wordpress } from './wordpress';

export type { StackTemplate };

/** The templates a new Managed Stack can start from, in the gallery's order. */
export const TEMPLATES: StackTemplate[] = [
  staticSite,
  postgres,
  mariadb,
  redis,
  caddy,
  uptimeKuma,
  wordpress,
  empty,
];
