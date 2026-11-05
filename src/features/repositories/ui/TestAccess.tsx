import { CircleCheckIcon } from 'lucide-react';
import type { Repository } from '@/shared/api';
import { Button, ErrorMessage } from '@/shared/ui';
import { useTestRepository } from '../api/mutations';
import { t } from '@/shared/i18n';

/** "Test access": lists the repository's branches and tags from this computer. */
export function TestAccess({ repository }: { repository: Repository }) {
  const test = useTestRepository();
  return (
    <div className="grid gap-2">
      <div>
        <Button
          type="button"
          variant="outline"
          size="sm"
          disabled={test.isPending}
          onClick={() => test.mutate(repository.id)}
        >
          {test.isPending ? t('repositories.test.testing') : t('repositories.test.button')}
        </Button>
      </div>
      {test.data && (
        <p className="text-success inline-flex items-center gap-1.5 text-sm">
          <CircleCheckIcon className="size-4" />
          {t('repositories.test.success', {
            branches: test.data.branches.length,
            tags: test.data.tags.length,
          })}
        </p>
      )}
      {test.error && <ErrorMessage error={test.error} />}
    </div>
  );
}
