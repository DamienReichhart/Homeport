import { checkKeys, rustKeys } from './keys';

const catalogue = [
  { name: 'common', keys: ['common.cancel'] },
  { name: 'images', keys: ['images.remove.title', 'images.pull.done'] },
];

describe('i18n key check', () => {
  it('passes when the code and the catalogue agree', () => {
    const sources = [
      { path: 'a.tsx', text: 't(\'common.cancel\'); t("images.remove.title", { name })' },
      { path: 'b.ts', text: "const DONE: MessageKey = 'images.pull.done';" },
    ];
    expect(checkKeys(catalogue, sources)).toEqual([]);
  });

  it('names missing, unused, misplaced and runtime-built keys', () => {
    const problems = checkKeys(
      [...catalogue, { name: 'volumes', keys: ['images.stray'] }],
      [
        { path: 'a.tsx', text: "t('common.cancel'); t('images.remove.titel'); t(`images.${x}`)" },
        { path: 'b.ts', text: "import x from './foo.bar'; const host = 'host.docker.internal';" },
      ],
    );
    expect(problems).toEqual([
      'images.stray is in volumes.ts but doesn\'t start with "volumes."',
      'a.tsx uses images.remove.titel, which the catalogue lacks',
      'a.tsx builds a key at runtime (t(`images.${…): use literal keys, for example in a map',
      'images.remove.title is in the catalogue but nothing uses it',
      'images.pull.done is in the catalogue but nothing uses it',
      'images.stray is in the catalogue but nothing uses it',
    ]);
  });

  it('counts a key as read by Rust only where shipped code shows it', () => {
    const page = [
      '//! `t!("images.only.doc")` names a message here.',
      'pub const UNITS: [&str; 2] = ["format.bytes.b", "format.bytes.kb"];',
      'const TITLE: &\'static str = "images.title";',
      'fn view() -> String {',
      '    // t!("images.only.comment")',
      '    let id = "images.only.string";',
      '    t!("images.remove.title", name = id) + &rich!(',
      '        "images.pull.done"',
      '    )',
      '}',
      '#[cfg(test)]',
      'mod tests {',
      '    fn it() { t!("images.only.inline.test"); }',
      '}',
    ].join('\n');
    const test = 'fn it() { t!("images.only.test"); }';
    const keys = rustKeys([
      { path: 'app/src/ui/images/view.rs', text: page },
      { path: 'app/src/ui/images/tests.rs', text: test },
      { path: 'app/src/ui/images/stage_tests.rs', text: test },
      { path: 'app/src/test_support/fixtures.rs', text: test },
      { path: 'app/src/ui/widgets/gallery/mod.rs', text: test },
    ]);
    expect(keys.sort()).toEqual([
      'format.bytes.b',
      'format.bytes.kb',
      'images.pull.done',
      'images.remove.title',
      'images.title',
    ]);
  });
});
