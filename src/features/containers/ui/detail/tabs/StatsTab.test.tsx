import { screen, waitFor } from '@testing-library/react';
import { baseCommands, statsSample } from '@/test/fixtures';
import { mockCommands, sendToChannel } from '@/test/ipc';
import { renderWithProviders } from '@/test/render';
import { StatsTab } from './StatsTab';

// uPlot draws on a canvas jsdom lacks; the figures in the titles are what's checked.
vi.mock('@/shared/ui', async (original) => ({
  ...(await original<object>()),
  TimeChart: () => null,
}));

/** Matches a card title by its whole text, which React splits into several nodes. */
const title = (text: string) => (_: string, el: Element | null) =>
  el?.getAttribute('data-slot') === 'card-title' && el.textContent === text;

describe('stats tab', () => {
  it('shows the latest sample in each chart title', async () => {
    const channels: unknown[] = [];
    mockCommands({
      ...baseCommands,
      stats_subscribe: ({ onSample }) => {
        channels.push(onSample);
        return 1;
      },
    });
    renderWithProviders(<StatsTab hostId="h1" containerId="web-1" running />);
    await waitFor(() => expect(channels).toHaveLength(1));

    sendToChannel(channels[0], statsSample(1), statsSample(2, { cpuPercent: 50, pids: 7 }));
    expect(await screen.findByText(title('CPU · 50.0%'))).toBeInTheDocument();
    expect(screen.getByText(title('Memory · 64.0 MB of 1.0 GB'))).toBeInTheDocument();
    expect(screen.getByText(title('Network · 2.0 KB in, 1000 B out'))).toBeInTheDocument();
    expect(screen.getByText(title('Disk I/O · 7 processes'))).toBeInTheDocument();
  });

  it('asks to start a stopped container and subscribes to nothing', () => {
    mockCommands({ ...baseCommands });
    renderWithProviders(<StatsTab hostId="h1" containerId="web-1" running={false} />);
    expect(screen.getByText('Start the container to see its resource usage.')).toBeInTheDocument();
  });
});
