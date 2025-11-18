import { createContext, useContext, useMemo, useState, type ReactNode } from 'react';
import { DEFAULT_THEME, THEMES, type ThemeDefinition, type ThemeName } from './themes';

type ThemeContextValue = {
  name: ThemeName;
  theme: ThemeDefinition;
  setTheme: (name: ThemeName) => void;
};

const ThemeContext = createContext<ThemeContextValue | undefined>(undefined);

export function ThemeProvider({ children }: { children: ReactNode }) {
  const [name, setName] = useState<ThemeName>(DEFAULT_THEME);

  const value = useMemo<ThemeContextValue>(
    () => ({ name, theme: THEMES[name], setTheme: setName }),
    [name],
  );

  return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}

export function useTheme() {
  const ctx = useContext(ThemeContext);
  if (!ctx) {
    throw new Error('useTheme must be used within ThemeProvider');
  }
  return ctx;
}