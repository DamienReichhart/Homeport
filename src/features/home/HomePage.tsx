import { useAsync } from '@/shared/hooks';
import { ErrorMessage, Spinner } from '@/shared/ui';
import { getAppInfo } from './api';

export function HomePage() {
  const info = useAsync(getAppInfo);

  return (
    <section>
      <h1>Homeport</h1>
      {info.status === 'loading' && <Spinner />}
      {info.status === 'error' && <ErrorMessage error={info.error} />}
      {info.status === 'success' && (
        <p className="text-muted-foreground">
          {info.data.name} v{info.data.version} on {info.data.os}/{info.data.arch}
        </p>
      )}
    </section>
  );
}
