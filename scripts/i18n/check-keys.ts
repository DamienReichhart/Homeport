// `bun run i18n:check`: the English catalogue and the code name the same keys.
import { readdirSync, readFileSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { checkKeys, type Namespace, type Source } from './keys';

const root = join(import.meta.dir, '..', '..');
const catalogue = join(root, 'src', 'shared', 'i18n', 'en');

const namespaces: Namespace[] = [];
for (const file of readdirSync(catalogue)) {
  const match = /^([a-z]+)\.ts$/.exec(file);
  if (!match || file === 'index.ts') continue;
  const messages = ((await import(join(catalogue, file))) as { default: Record<string, string> })
    .default;
  namespaces.push({ name: match[1] ?? '', keys: Object.keys(messages) });
}

function walk(dir: string, out: Source[]) {
  for (const entry of readdirSync(dir)) {
    const path = join(dir, entry);
    if (statSync(path).isDirectory()) {
      if (path !== join(root, 'src', 'shared', 'i18n')) walk(path, out);
    } else if (/\.(ts|tsx)$/.test(entry) && !/\.test\.tsx?$/.test(entry)) {
      out.push({ path: relative(root, path), text: readFileSync(path, 'utf8') });
    }
  }
  return out;
}

const problems = checkKeys(namespaces, walk(join(root, 'src'), []));
if (problems.length > 0) {
  console.error(problems.join('\n'));
  console.error(`\n${problems.length} catalogue problem(s); see ADR 0011.`);
  process.exit(1);
}
console.log(`The catalogue and the code agree (${namespaces.length} namespaces).`);
