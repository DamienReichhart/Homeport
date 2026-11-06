import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { render, screen, within } from '@testing-library/react';
import { MemoryRouter } from 'react-router-dom';
import { baseCommands } from '@/test/fixtures';
import { mockCommands } from '@/test/ipc';
import { DiskUsageCard } from './DiskUsageCard';

const line = { total: 2, active: 1, size: 2048, reclaimable: 1024 };

function renderCard(partial: boolean) {
  mockCommands({
    ...baseCommands,
    system_disk_usage: () => ({
      images: line,
      containers: line,
      volumes: { ...line, size: 0, reclaimable: 0 },
      buildCache: { total: 0, active: 0, size: 0, reclaimable: 0 },
      partial,
    }),
  });
  render(
    <QueryClientProvider client={new QueryClient()}>
      <MemoryRouter>
        <DiskUsageCard hostId="h1" />
      </MemoryRouter>
    </QueryClientProvider>,
  );
}

describe('disk usage', () => {
  it('says what an older Engine can’t report instead of showing zeros', async () => {
    renderCard(true);
    expect(await screen.findByText(/older than version 29/)).toBeInTheDocument();
    const volumes = screen.getByText('Volumes').closest('tr')!;
    expect(within(volumes).getAllByText('—')).toHaveLength(2);
  });

  it('shows every size from a current Engine', async () => {
    renderCard(false);
    const volumes = (await screen.findByText('Volumes')).closest('tr')!;
    expect(within(volumes).getAllByText('0 B')).toHaveLength(2);
    expect(screen.queryByText(/older than version 29/)).toBeNull();
  });
});
