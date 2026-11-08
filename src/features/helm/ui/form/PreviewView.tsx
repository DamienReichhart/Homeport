import type { ReleasePreview } from '@/shared/api';
import { YamlEditor } from '@/shared/ui';
import { t } from '@/shared/i18n';

/** What an install or upgrade would apply: the rendered manifest, then the chart's notes. */
export function PreviewView({ preview }: { preview: ReleasePreview }) {
  return (
    <div className="grid gap-3">
      <p className="text-muted-foreground text-sm">{t('helm.preview.description')}</p>
      <YamlEditor
        value={preview.manifest}
        label={t('helm.preview.manifest')}
        readOnly
        height="22rem"
      />
      {preview.notes.trim() && (
        <pre
          aria-label={t('helm.notes.label')}
          className="bg-muted max-h-40 overflow-auto rounded-md p-3 font-mono text-xs"
        >
          {preview.notes}
        </pre>
      )}
    </div>
  );
}
