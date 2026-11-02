export function ErrorMessage({ error }: { error: Error }) {
  return (
    <p role="alert" className="error">
      {error.message}
    </p>
  );
}
