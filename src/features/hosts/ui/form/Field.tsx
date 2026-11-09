import { cloneElement, isValidElement, useId, type ReactElement, type ReactNode } from 'react';
import { Label } from '@/shared/ui';

/**
 * A labelled form field. When its child is a single control, the label names
 * it and the hint describes it, so screen readers announce both.
 */
export function Field({
  label,
  hint,
  children,
}: {
  label: string;
  hint?: ReactNode;
  children: ReactNode;
}) {
  const id = useId();
  const hintId = `${id}-hint`;
  const control = isValidElement<{ id?: string; 'aria-describedby'?: string }>(children)
    ? (children as ReactElement<{ id?: string; 'aria-describedby'?: string }>)
    : null;
  const controlId = control ? (control.props.id ?? id) : undefined;
  return (
    <div className="grid gap-1.5">
      <Label htmlFor={controlId}>{label}</Label>
      {control
        ? cloneElement(control, {
            id: controlId,
            'aria-describedby': hint ? hintId : control.props['aria-describedby'],
          })
        : children}
      {hint && (
        <p id={hintId} className="text-muted-foreground text-xs">
          {hint}
        </p>
      )}
    </div>
  );
}
