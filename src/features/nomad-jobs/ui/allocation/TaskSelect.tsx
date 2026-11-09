import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** Picks one of an allocation's tasks, when its task group has several. */
export function TaskSelect({
  tasks,
  value,
  onChange,
}: {
  tasks: string[];
  value: string;
  onChange: (task: string) => void;
}) {
  return (
    <Select value={value} onValueChange={onChange}>
      <SelectTrigger size="sm" className="w-48" aria-label={t('nomad.alloc.task')}>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {tasks.map((task) => (
          <SelectItem key={task} value={task}>
            {task}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}
