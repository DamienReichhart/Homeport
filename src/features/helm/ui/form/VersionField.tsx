import { useQuery } from '@tanstack/react-query';
import {
  ErrorMessage,
  Input,
  Label,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Spinner,
} from '@/shared/ui';
import { helmQueries } from '../../api/queries';
import { isOci } from '../../model/install';
import { t } from '@/shared/i18n';

/**
 * The chart version to use: picked among a repository chart's versions
 * (the latest first), or typed for an OCI chart, whose versions helm can't
 * list (empty for the latest).
 */
export function VersionField({
  chart,
  value,
  onChange,
}: {
  chart: string;
  value: string;
  onChange: (version: string) => void;
}) {
  const oci = isOci(chart);
  const versions = useQuery({ ...helmQueries.chartVersions(chart), enabled: !oci });
  return (
    <div className="grid gap-2">
      <Label htmlFor="helm-version">{t('helm.install.version')}</Label>
      {oci ? (
        <Input
          id="helm-version"
          value={value}
          placeholder={t('helm.install.versionLatest')}
          onChange={(e) => onChange(e.target.value.trim())}
        />
      ) : versions.isPending ? (
        <Spinner />
      ) : versions.error ? (
        <ErrorMessage error={versions.error} onRetry={() => void versions.refetch()} />
      ) : (
        <Select value={value || versions.data[0]?.version} onValueChange={onChange}>
          <SelectTrigger id="helm-version" className="w-full">
            <SelectValue placeholder={t('helm.install.versionLatest')} />
          </SelectTrigger>
          <SelectContent>
            {versions.data.map((v) => (
              <SelectItem key={v.version} value={v.version}>
                {t('helm.install.versionOption', {
                  version: v.version,
                  app: v.appVersion || '—',
                })}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      )}
    </div>
  );
}
