import { expect, test } from 'bun:test';
import { releaseNotes } from './notes';

const body = `## What's changed

### Features

- **hosts:** reconnect on their own ([abc1234](https://example.com))

## Download

| File | Size |
| ---- | ---- |
`;

test('drops the download table, which the download page replaces', () => {
  expect(releaseNotes(body)).not.toContain('Download');
  expect(releaseNotes(body)).not.toContain('<table');
});

test('demotes headings below the release title', () => {
  const html = releaseNotes(body);
  expect(html).toContain('<h3>What&#39;s changed</h3>');
  expect(html).toContain('<h4>Features</h4>');
});

test('renders the rest as Markdown', () => {
  expect(releaseNotes(body)).toContain(
    '<li><strong>hosts:</strong> reconnect on their own (<a href="https://example.com">abc1234</a>)</li>',
  );
});

test('keeps notes without a download section whole', () => {
  expect(releaseNotes('# Title\n\nText')).toBe('<h2>Title</h2>\n<p>Text</p>\n');
});
