import { useState } from 'react';
import { Button, Input, Label, Tabs, TabsContent, TabsList, TabsTrigger } from '@/shared/ui';
import { ociProblem } from '../../model/install';
import { ChartSearch } from '../repositories/ChartSearch';
import { t } from '@/shared/i18n';

/** Picks the chart to install: one of the repositories', or an OCI reference. */
export function ChartStep({ onPick }: { onPick: (chart: string) => void }) {
  const [reference, setReference] = useState('');
  const [touched, setTouched] = useState(false);
  const problem = ociProblem(reference);
  return (
    <Tabs defaultValue="repository">
      <TabsList>
        <TabsTrigger value="repository">{t('helm.install.fromRepository')}</TabsTrigger>
        <TabsTrigger value="oci">{t('helm.install.fromOci')}</TabsTrigger>
      </TabsList>
      <TabsContent value="repository" className="pt-3">
        <ChartSearch onPick={(chart) => onPick(chart.name)} />
      </TabsContent>
      <TabsContent value="oci" className="pt-3">
        <form
          className="grid gap-3"
          onSubmit={(e) => {
            e.preventDefault();
            setTouched(true);
            if (!problem) onPick(reference.trim());
          }}
        >
          <Label htmlFor="helm-oci">{t('helm.install.ociReference')}</Label>
          <Input
            id="helm-oci"
            value={reference}
            placeholder="oci://registry-1.docker.io/bitnamicharts/nginx"
            onChange={(e) => setReference(e.target.value)}
          />
          <p className="text-muted-foreground text-sm">{t('helm.install.ociHint')}</p>
          {touched && problem && (
            <p role="alert" className="text-destructive text-sm">
              {t(problem)}
            </p>
          )}
          <Button type="submit" className="justify-self-start">
            {t('helm.install.next')}
          </Button>
        </form>
      </TabsContent>
    </Tabs>
  );
}
