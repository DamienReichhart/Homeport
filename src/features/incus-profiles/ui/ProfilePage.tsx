import { useQuery } from '@tanstack/react-query';
import { ArrowLeftIcon, RotateCcwIcon, SaveIcon } from 'lucide-react';
import { useState } from 'react';
import { Link, useNavigate, useParams } from 'react-router-dom';
import { IncusGate } from '@/core/host';
import { Badge, Button, ErrorMessage, PageHeader, Spinner, YamlEditor } from '@/shared/ui';
import { useSaveProfile } from '../api/mutations';
import { incusProfileQueries } from '../api/queries';
import { t } from '@/shared/i18n';

function ProfileEditor({
  hostId,
  project,
  name,
}: {
  hostId: string;
  project: string;
  name: string;
}) {
  const navigate = useNavigate();
  const back = () => void navigate('/incus/profiles');
  const detail = useQuery(incusProfileQueries.detail(hostId, project, name));
  const save = useSaveProfile(hostId, project, name);
  // The text being edited, and the version of the profile it started from.
  const [draft, setDraft] = useState<{ yaml: string; etag: string | null } | null>(null);

  if (detail.isPending) return <Spinner />;
  if (detail.error)
    return (
      <div className="grid gap-3">
        <ErrorMessage error={detail.error} onRetry={() => void detail.refetch()} />
        <Button variant="outline" className="justify-self-start" onClick={back}>
          {t('incus.profiles.backToList')}
        </Button>
      </div>
    );
  const { profile } = detail.data;
  const yaml = draft?.yaml ?? detail.data.yaml;
  const etag = draft ? draft.etag : detail.data.etag;
  const changed = draft !== null && draft.yaml !== detail.data.yaml;
  const reload = () => {
    setDraft(null);
    save.reset();
    void detail.refetch();
  };
  return (
    <div className="flex h-full min-h-0 flex-col gap-3">
      <PageHeader
        title={
          <span className="flex items-center gap-2">
            <Button variant="ghost" size="icon-sm" aria-label={t('common.back')} onClick={back}>
              <ArrowLeftIcon />
            </Button>
            {profile.name}
          </span>
        }
        description={profile.description || t('incus.profiles.noDescription')}
        actions={
          <>
            <Button variant="outline" onClick={reload}>
              <RotateCcwIcon /> {t('incus.profiles.reload')}
            </Button>
            <Button
              disabled={!changed || save.isPending}
              onClick={() =>
                save
                  .mutateAsync({ yaml, etag })
                  .then(() => setDraft(null))
                  .catch(() => {})
              }
            >
              <SaveIcon /> {t('common.save')}
            </Button>
          </>
        }
      />
      <div className="flex flex-wrap items-center gap-1.5 text-sm">
        <span className="text-muted-foreground">{t('incus.profiles.usedBy')}</span>
        {profile.usedBy.length === 0 && <span className="text-muted-foreground">—</span>}
        {profile.usedBy.map((instance) => (
          <Badge key={instance} variant="outline" asChild>
            <Link to={`/incus/instances/${encodeURIComponent(instance)}`}>{instance}</Link>
          </Badge>
        ))}
      </div>
      {save.error && <ErrorMessage error={save.error} onRetry={undefined} />}
      <p className="text-muted-foreground text-xs">{t('incus.profiles.editHint')}</p>
      <div className="min-h-[20rem] flex-1">
        <YamlEditor
          label={t('incus.profiles.yamlOf', { name: profile.name })}
          value={yaml}
          onChange={(next) => setDraft({ yaml: next, etag })}
          height="100%"
        />
      </div>
    </div>
  );
}

export function ProfilePage() {
  const { name = '' } = useParams();
  return (
    <IncusGate>
      {(hostId, project) => (
        <ProfileEditor
          key={`${hostId}/${project}/${name}`}
          hostId={hostId}
          project={project}
          name={decodeURIComponent(name)}
        />
      )}
    </IncusGate>
  );
}
