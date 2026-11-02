import { CopyButton } from './CopyButton';

/** Pretty-printed JSON with a copy button, for raw inspect documents. */
export function JsonView({ value }: { value: unknown }) {
  const text = JSON.stringify(value, null, 2);
  return (
    <div className="relative">
      <div className="absolute top-2 right-2">
        <CopyButton text={text} />
      </div>
      <pre className="bg-muted/40 max-h-[70vh] overflow-auto rounded-md border p-3 font-mono text-xs leading-relaxed">
        {text}
      </pre>
    </div>
  );
}
