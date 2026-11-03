export function ErrorMessage({ error }: { error: Error }) {
  return (
    <p role="alert" className="text-destructive text-sm">
      {error.message}
    </p>
  );
}
