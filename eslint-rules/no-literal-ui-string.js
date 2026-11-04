/**
 * Flags user-facing text written inline instead of read from the catalogue
 * (ADR 0011): JSX text, string props that people read (`title`,
 * `aria-label`, `placeholder`…), and the text given to toasts and
 * confirmations. Strings that read as code rather than words — `tcp`,
 * `nginx:latest`, `/dev/dri`, `max-size=10m`, `NET_ADMIN` — and proper
 * nouns pass. To keep a literal on purpose, disable the rule for the line
 * with a reason: `// eslint-disable-next-line homeport/no-literal-ui-string -- a command`.
 */

/** Props whose value is read by people (or screen readers). */
const TEXT_PROPS = new Set([
  'title',
  'aria-label',
  'aria-description',
  'placeholder',
  'alt',
  'label',
  'description',
  'confirmLabel',
  'hint',
  'empty',
  'verb',
]);

/** Object keys holding text in toasts, confirmations and similar calls. */
const TEXT_KEYS = new Set([
  'title',
  'description',
  'confirmLabel',
  'label',
  'message',
  'header',
  'placeholder',
  'empty',
  'hint',
]);

/** Calls whose string arguments are shown to people. */
const TOAST_METHODS = new Set(['success', 'error', 'info', 'warning', 'message', 'loading']);

/** Names that stay as they are in every language. */
const PROPER_NOUNS = new Set([
  'Homeport',
  'Docker',
  'Docker Engine',
  'Docker Hub',
  'Docker Compose',
  'Podman',
  'GitHub',
  'GHCR',
  'SSH',
  'TLS',
  'TCP',
  'JSON',
  'YAML',
  'CPU',
  'stdout',
  'stderr',
]);

/** Whether `text` reads as words someone would translate. */
function isWords(text) {
  const s = text.trim();
  if (!/[A-Za-z]/.test(s)) return false; // punctuation, numbers, symbols
  if (PROPER_NOUNS.has(s)) return false;
  // One token with no capitalised word in it reads as code: tcp, nginx:latest, /dev/dri, NET_ADMIN.
  if (!/\s/.test(s) && !/^[A-Z][a-z]/.test(s)) return false;
  return true;
}

const MESSAGE =
  'User-facing text belongs in the catalogue: add a key to src/shared/i18n/en/<namespace>.ts and use t(…) (ADR 0011).';

function stringOf(node) {
  if (!node) return null;
  if (node.type === 'Literal' && typeof node.value === 'string') return node.value;
  if (node.type === 'TemplateLiteral')
    return node.quasis.map((q) => q.value.cooked ?? '').join(' ');
  if (node.type === 'JSXExpressionContainer') return stringOf(node.expression);
  if (node.type === 'ConditionalExpression')
    return [stringOf(node.consequent), stringOf(node.alternate)].filter(Boolean).join(' ') || null;
  if (node.type === 'LogicalExpression') return stringOf(node.right);
  return null;
}

function reportIfWords(context, node, valueNode = node) {
  const text = stringOf(valueNode);
  if (text !== null && isWords(text)) context.report({ node, message: MESSAGE });
}

function isToastCall(callee) {
  if (callee.type === 'Identifier') return callee.name === 'toast' || callee.name === 'toastError';
  return (
    callee.type === 'MemberExpression' &&
    callee.object.type === 'Identifier' &&
    callee.object.name === 'toast' &&
    callee.property.type === 'Identifier' &&
    TOAST_METHODS.has(callee.property.name)
  );
}

export default {
  meta: {
    type: 'problem',
    docs: { description: 'Disallow user-facing string literals outside the i18n catalogue.' },
    schema: [],
  },
  create(context) {
    return {
      JSXText(node) {
        if (isWords(node.value)) context.report({ node, message: MESSAGE });
      },
      JSXAttribute(node) {
        const name = node.name.type === 'JSXIdentifier' ? node.name.name : null;
        if (name && TEXT_PROPS.has(name) && node.value) reportIfWords(context, node, node.value);
      },
      JSXExpressionContainer(node) {
        // Text between tags written as {'…'} or {`…`}.
        if (node.parent?.type === 'JSXElement' || node.parent?.type === 'JSXFragment') {
          const e = node.expression;
          if (e.type === 'Literal' || e.type === 'TemplateLiteral') reportIfWords(context, node, e);
        }
      },
      CallExpression(node) {
        if (!isToastCall(node.callee)) return;
        for (const arg of node.arguments) {
          if (arg.type === 'ObjectExpression') continue; // its keys are checked below
          reportIfWords(context, arg);
        }
      },
      Property(node) {
        const key = node.key.type === 'Identifier' ? node.key.name : stringOf(node.key);
        if (key && TEXT_KEYS.has(key)) reportIfWords(context, node.value);
      },
    };
  },
};
