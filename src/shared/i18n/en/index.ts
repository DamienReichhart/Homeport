import app from './app';
import board from './board';
import common from './common';
import containers from './containers';
import core from './core';
import errors from './errors.json';
import events from './events';
import hosts from './hosts';
import images from './images';
import networks from './networks';
import overview from './overview';
import registries from './registries';
import repositories from './repositories';
import search from './search';
import settings from './settings';
import stacks from './stacks';
import swarm from './swarm';
import ui from './ui';
import updates from './updates';
import volumes from './volumes';

/** The UI's text: one namespace per file, its keys prefixed with its name. */
export const uiMessages = {
  ...common,
  ...app,
  ...core,
  ...ui,
  ...board,
  ...containers,
  ...events,
  ...hosts,
  ...images,
  ...networks,
  ...overview,
  ...registries,
  ...repositories,
  ...search,
  ...settings,
  ...stacks,
  ...swarm,
  ...updates,
  ...volumes,
};

/**
 * Error messages and hints, generated from the Rust crates' `messages!`
 * blocks by `bun run bindings` (ADR 0011); never edited by hand.
 */
export const errorMessages: Record<string, string> = errors;

/** Every English message, by key. */
export const en: Record<string, string> = { ...uiMessages, ...errorMessages };
