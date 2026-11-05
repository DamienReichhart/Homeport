import { RuleTester } from 'eslint';
import tseslint from 'typescript-eslint';
import rule from './no-literal-ui-string.js';

const tester = new RuleTester({
  languageOptions: {
    parser: tseslint.parser,
    parserOptions: { ecmaFeatures: { jsx: true } },
  },
});

tester.run('no-literal-ui-string', rule, {
  valid: [
    'const a = <Button>{t("common.remove")}</Button>;',
    'const a = <Input placeholder="nginx:latest" />;',
    'const a = <Input placeholder="/dev/dri" />;',
    'const a = <SelectItem value="tcp">tcp</SelectItem>;',
    'const a = <span>·</span>;',
    'const a = <span>{count} / {total}</span>;',
    'const a = <Toggle>stdout</Toggle>;',
    'const a = <p>Homeport</p>;',
    'toast.success(t("images.pulled", { name }));',
    'const k = { kind: "rename", id: "x" };',
  ],
  invalid: [
    { code: 'const a = <Button>Delete</Button>;', errors: 1 },
    { code: 'const a = <Input placeholder="Search images" />;', errors: 1 },
    { code: 'const a = <Button aria-label={`Remove ${name}`} />;', errors: 1 },
    { code: 'toast.success("Done");', errors: 1 },
    { code: 'toast.success(`Removed ${n} images`);', errors: 1 },
    { code: 'toastError(e, "Pull failed");', errors: 1 },
    { code: 'confirm({ title: "Remove it?", confirmLabel: "Remove" });', errors: 2 },
    { code: 'const a = <p>{"Nothing here"}</p>;', errors: 1 },
  ],
});
