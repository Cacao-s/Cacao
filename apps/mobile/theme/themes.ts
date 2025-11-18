export type ThemeName = 'light' | 'dark' | 'highContrast';

export type ThemeDefinition = {
  background: string;
  surface: string;
  card: string;
  text: string;
  mutedText: string;
  accent: string;
  border: string;
};

export const THEMES: Record<ThemeName, ThemeDefinition> = {
  light: {
    background: '#f7f7fb',
    surface: '#ffffff',
    card: '#ffffff',
    text: '#1f2533',
    mutedText: '#6c7484',
    accent: '#4f46e5',
    border: '#d9dce4',
  },
  dark: {
    background: '#12121a',
    surface: '#1d1f2b',
    card: '#232535',
    text: '#f5f5f7',
    mutedText: '#a5a7b6',
    accent: '#a78bfa',
    border: '#2f3244',
  },
  highContrast: {
    background: '#000000',
    surface: '#000000',
    card: '#111111',
    text: '#ffffff',
    mutedText: '#fffa86',
    accent: '#ffcc00',
    border: '#ffffff',
  },
};

export const DEFAULT_THEME: ThemeName = 'light';