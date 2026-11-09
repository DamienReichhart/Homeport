import type { StarlightUserConfig } from '@astrojs/starlight/types';
import { PREFIXED_LOCALES } from '../../core/site/locales.ts';
import en from './sidebar/en.ts';
import fr from './sidebar/fr.ts';
import es from './sidebar/es.ts';
import de from './sidebar/de.ts';
import it from './sidebar/it.ts';
import ptbr from './sidebar/pt-br.ts';
import ru from './sidebar/ru.ts';
import zhcn from './sidebar/zh-cn.ts';
import ja from './sidebar/ja.ts';
import ko from './sidebar/ko.ts';
import hi from './sidebar/hi.ts';
import ar from './sidebar/ar.ts';

type Key = keyof typeof en;

/**
 * Every language's labels, by its code in `LOCALES`. astro.config loads this
 * file outside Vite, so each language is imported here: a new language adds
 * its `sidebar/<code>.ts` and one line below; one without labels shows English.
 */
const BY_CODE: Record<string, Partial<Record<Key, string>>> = {
  fr: fr,
  es: es,
  de: de,
  it: it,
  'pt-br': ptbr,
  ru: ru,
  'zh-cn': zhcn,
  ja: ja,
  ko: ko,
  hi: hi,
  ar: ar,
};
const labels = new Map(
  PREFIXED_LOCALES.filter((l) => BY_CODE[l.code]).map((l) => [l.lang, BY_CODE[l.code] ?? {}]),
);

/** A label in English, with its translation for every language that has one. */
function label(key: Key) {
  const translations: Record<string, string> = {};
  for (const [lang, texts] of labels) {
    const text = texts[key];
    if (text) translations[lang] = text;
  }
  return { label: en[key], translations };
}
const group = label;
const entry = label;

/** The docs' sidebar, in reading order. A new docs page gets its entry here. */
export const sidebar: NonNullable<StarlightUserConfig['sidebar']> = [
  {
    ...group('group.getting-started'),
    items: [
      { ...entry('docs'), slug: 'docs' },
      { ...entry('docs/installation'), slug: 'docs/installation' },
      { ...entry('docs/quick-start'), slug: 'docs/quick-start' },
      { ...entry('docs/requirements'), slug: 'docs/requirements' },
    ],
  },
  {
    ...group('group.hosts'),
    items: [
      { ...entry('docs/hosts'), slug: 'docs/hosts' },
      { ...entry('docs/hosts/local'), slug: 'docs/hosts/local' },
      { ...entry('docs/hosts/ssh'), slug: 'docs/hosts/ssh' },
      { ...entry('docs/hosts/tls-tcp'), slug: 'docs/hosts/tls-tcp' },
      { ...entry('docs/hosts/containerd'), slug: 'docs/hosts/containerd' },
      { ...entry('docs/hosts/incus'), slug: 'docs/hosts/incus' },
    ],
  },
  {
    ...group('group.clusters'),
    items: [
      { ...entry('docs/clusters'), slug: 'docs/clusters' },
      { ...entry('docs/clusters/local'), slug: 'docs/clusters/local' },
      { ...entry('docs/clusters/exec-plugins'), slug: 'docs/clusters/exec-plugins' },
      { ...entry('docs/clusters/workloads'), slug: 'docs/clusters/workloads' },
      { ...entry('docs/clusters/networking-config'), slug: 'docs/clusters/networking-config' },
      { ...entry('docs/clusters/nodes-events'), slug: 'docs/clusters/nodes-events' },
      { ...entry('docs/clusters/api-resources'), slug: 'docs/clusters/api-resources' },
      { ...entry('docs/clusters/logs-exec'), slug: 'docs/clusters/logs-exec' },
      { ...entry('docs/clusters/port-forward'), slug: 'docs/clusters/port-forward' },
      { ...entry('docs/clusters/actions'), slug: 'docs/clusters/actions' },
      { ...entry('docs/clusters/yaml'), slug: 'docs/clusters/yaml' },
      { ...entry('docs/clusters/helm'), slug: 'docs/clusters/helm' },
      { ...entry('docs/clusters/nomad'), slug: 'docs/clusters/nomad' },
    ],
  },
  {
    ...group('group.using-homeport'),
    items: [
      { ...entry('docs/guides/overview'), slug: 'docs/guides/overview' },
      { ...entry('docs/guides/containers'), slug: 'docs/guides/containers' },
      { ...entry('docs/guides/images'), slug: 'docs/guides/images' },
      { ...entry('docs/guides/volumes-networks'), slug: 'docs/guides/volumes-networks' },
      { ...entry('docs/guides/stacks'), slug: 'docs/guides/stacks' },
      { ...entry('docs/guides/synced-stacks'), slug: 'docs/guides/synced-stacks' },
      { ...entry('docs/guides/swarm'), slug: 'docs/guides/swarm' },
      { ...entry('docs/guides/podman-pods'), slug: 'docs/guides/podman-pods' },
      { ...entry('docs/guides/quadlet'), slug: 'docs/guides/quadlet' },
      { ...entry('docs/guides/instances'), slug: 'docs/guides/instances' },
      { ...entry('docs/guides/settings'), slug: 'docs/guides/settings' },
      { ...entry('docs/guides/keyboard'), slug: 'docs/guides/keyboard' },
    ],
  },
  {
    ...group('group.reference'),
    items: [
      { ...entry('docs/reference/security'), slug: 'docs/reference/security' },
      { ...entry('docs/reference/verifying'), slug: 'docs/reference/verifying' },
      { ...entry('docs/reference/updating'), slug: 'docs/reference/updating' },
      { ...entry('docs/reference/troubleshooting'), slug: 'docs/reference/troubleshooting' },
      { ...entry('docs/reference/accessibility'), slug: 'docs/reference/accessibility' },
      { ...entry('docs/reference/faq'), slug: 'docs/reference/faq' },
      { ...entry('docs/reference/glossary'), slug: 'docs/reference/glossary' },
    ],
  },
  {
    ...group('group.project'),
    items: [
      { ...entry('docs/project/building'), slug: 'docs/project/building' },
      { ...entry('docs/project/contributing'), slug: 'docs/project/contributing' },
      { ...entry('/changelog/'), link: '/changelog/' },
    ],
  },
];
