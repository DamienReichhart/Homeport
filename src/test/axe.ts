import axe from 'axe-core';

/**
 * The WCAG 2.2 A and AA rules axe can check in jsdom. Colour contrast needs
 * real layout and colours, which jsdom lacks; `src/shared/styles/contrast.test.ts`
 * measures the brand tokens instead.
 */
const OPTIONS: axe.RunOptions = {
  runOnly: { type: 'tag', values: ['wcag2a', 'wcag2aa', 'wcag21a', 'wcag21aa', 'wcag22aa'] },
  rules: { 'color-contrast': { enabled: false } },
  resultTypes: ['violations'],
};

/** Each violation as one readable line: the rule, why, and where. */
export function describeViolations(violations: axe.Result[]): string[] {
  return violations.flatMap((v) =>
    v.nodes.map((n) => `${v.id}: ${v.help} — ${n.target.join(' ')} ${n.html.slice(0, 160)}`),
  );
}

/**
 * The axe violations in `root` (the whole document by default); a test
 * expects none: `expect(await accessibilityProblems()).toEqual([])`.
 */
export async function accessibilityProblems(root: Element = document.body): Promise<string[]> {
  const result = await axe.run(root, OPTIONS);
  return describeViolations(result.violations);
}

/** Fails with every axe violation in `root`, one per line. */
export async function expectAccessible(root: Element = document.body): Promise<void> {
  const problems = await accessibilityProblems(root);
  if (problems.length > 0) throw new Error(`Accessibility problems:\n${problems.join('\n')}`);
}
