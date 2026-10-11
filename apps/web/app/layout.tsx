import type { Metadata, Viewport } from 'next';
import { IBM_Plex_Mono, Jost } from 'next/font/google';
import type { ReactNode } from 'react';
import './globals.css';

const mono = IBM_Plex_Mono({
  subsets: ['latin'],
  weight: ['300', '400', '500'],
  variable: '--font-mono',
});

const jost = Jost({
  subsets: ['latin'],
  weight: ['200', '300'],
  variable: '--font-title',
});

export const metadata: Metadata = {
  title: 'Relight',
  description: 'Watch five AI civilizations evolve on their own and race to reignite a dead star.',
};

export const viewport: Viewport = {
  themeColor: '#000000',
};

export default function RootLayout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" className={`${mono.variable} ${jost.variable}`}>
      <body>{children}</body>
    </html>
  );
}
