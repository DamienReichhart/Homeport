import { useNavigate } from 'react-router-dom';
import type { Stack } from '@/shared/api';
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  useConfirm,
} from '@/shared/ui';
import { useForgetLinked } from '../../api/mutations';
import { t, tRich } from '@/shared/i18n';

/** Where a Linked Stack's compose files are, and forgetting it. */
export function LinkedDefinition({
  hostId,
  stack,
  linkedId,
}: {
  hostId: string;
  stack: Stack;
  linkedId: string;
}) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const forget = useForgetLinked(hostId, linkedId);
  return (
    <Card>
      <CardHeader>
        <CardTitle>{t('stacks.linked.title')}</CardTitle>
        <CardDescription>
          {tRich('stacks.linked.description', {
            folder: stack.workingDir ?? '',
            mono: (chunks) => <span className="font-mono">{chunks}</span>,
          })}
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Button
          variant="outline"
          onClick={() =>
            confirm({
              title: t('stacks.linked.forgetTitle', { name: stack.name }),
              description: t('stacks.linked.forgetDescription'),
              confirmLabel: t('stacks.linked.forget'),
              onConfirm: async () => {
                await forget.mutateAsync();
                navigate('/stacks');
              },
            })
          }
        >
          {t('stacks.linked.forget')}
        </Button>
      </CardContent>
      {dialog}
    </Card>
  );
}
