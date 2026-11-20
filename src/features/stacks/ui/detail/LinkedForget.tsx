import { useNavigate } from 'react-router-dom';
import type { Stack, StackOwner } from '@/shared/api';
import { Button, useConfirm } from '@/shared/ui';
import { useForgetLinked } from '../../api/mutations';
import { t } from '@/shared/i18n';

/**
 * Forgets a Linked Stack. Homeport only forgets where its files are: the
 * folder and the containers stay, so it asks once, without alarm.
 */
export function ForgetButton({
  owner,
  stack,
  linkedId,
}: {
  owner: StackOwner;
  stack: Stack;
  linkedId: string;
}) {
  const navigate = useNavigate();
  const [dialog, confirm] = useConfirm();
  const forget = useForgetLinked(owner, linkedId);
  return (
    <>
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
      {dialog}
    </>
  );
}
