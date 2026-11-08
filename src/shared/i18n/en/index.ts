import apiresources from './apiresources';
import app from './app';
import apply from './apply';
import board from './board';
import clusterevents from './clusterevents';
import clusters from './clusters';
import common from './common';
import config from './config';
import containers from './containers';
import core from './core';
import errors from './errors.json';
import events from './events';
import hosts from './hosts';
import images from './images';
import network from './network';
import networks from './networks';
import nodes from './nodes';
import overview from './overview';
import pods from './pods';
import portforwards from './portforwards';
import registries from './registries';
import repositories from './repositories';
import search from './search';
import settings from './settings';
import stacks from './stacks';
import swarm from './swarm';
import ui from './ui';
import updates from './updates';
import volumes from './volumes';
import workloads from './workloads';

/** The UI's text: one namespace per file, its keys prefixed with its name. */
export const uiMessages = {
  ...common,
  ...app,
  ...core,
  ...ui,
  ...apiresources,
  ...apply,
  ...board,
  ...clusterevents,
  ...clusters,
  ...config,
  ...containers,
  ...events,
  ...hosts,
  ...images,
  ...network,
  ...networks,
  ...nodes,
  ...overview,
  ...pods,
  ...portforwards,
  ...registries,
  ...repositories,
  ...search,
  ...settings,
  ...stacks,
  ...swarm,
  ...updates,
  ...volumes,
  ...workloads,
};

/**
 * Error messages and hints, generated from the Rust crates' `messages!`
 * blocks by `bun run bindings` (ADR 0011); never edited by hand.
 */
export const errorMessages: Record<string, string> = errors;

/** Every English message, by key. */
export const en: Record<string, string> = { ...uiMessages, ...errorMessages };
