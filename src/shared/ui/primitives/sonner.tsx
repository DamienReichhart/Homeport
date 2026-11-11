import { useResolvedTheme } from '@/shared/lib/theme';
import { Toaster as Sonner, type ToasterProps } from 'sonner';
// Bundled as a stylesheet: the <style> Sonner adds by itself carries no CSP
// nonce, so a built app's webview ignores it and toasts would lose their layout.
import 'sonner/dist/styles.css';
import {
  CircleCheckIcon,
  InfoIcon,
  TriangleAlertIcon,
  OctagonXIcon,
  Loader2Icon,
} from 'lucide-react';

/** Sonner's `richColors` variables for one tone, tinted from a theme token. */
const tone = (name: string, colour: string) => ({
  [`--${name}-bg`]: `color-mix(in oklch, ${colour} 10%, var(--popover))`,
  [`--${name}-border`]: `color-mix(in oklch, ${colour} 30%, var(--popover))`,
  [`--${name}-text`]: colour,
});

const Toaster = ({ ...props }: ToasterProps) => {
  const theme = useResolvedTheme();

  return (
    <Sonner
      theme={theme as ToasterProps['theme']}
      className="toaster group"
      icons={{
        success: <CircleCheckIcon className="size-4" />,
        info: <InfoIcon className="size-4" />,
        warning: <TriangleAlertIcon className="size-4" />,
        error: <OctagonXIcon className="size-4" />,
        loading: <Loader2Icon className="size-4 animate-spin" />,
      }}
      style={
        {
          '--normal-bg': 'var(--popover)',
          '--normal-text': 'var(--popover-foreground)',
          '--normal-border': 'var(--border)',
          '--border-radius': 'var(--radius)',
          ...tone('success', 'var(--success)'),
          ...tone('info', 'var(--info)'),
          ...tone('warning', 'var(--warning)'),
          ...tone('error', 'var(--destructive)'),
        } as React.CSSProperties
      }
      toastOptions={{
        classNames: {
          toast: 'cn-toast',
        },
      }}
      {...props}
    />
  );
};

export { Toaster };
