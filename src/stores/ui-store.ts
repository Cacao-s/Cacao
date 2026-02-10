import { create } from "zustand";
import i18n from "@/i18n";

type Locale = "zh-TW" | "en";
type Theme = "light" | "dark" | "high-contrast";

interface UiState {
  locale: Locale;
  theme: Theme;
  setLocale: (locale: Locale) => void;
  setTheme: (theme: Theme) => void;
}

function applyTheme(theme: Theme) {
  const root = document.documentElement;
  root.classList.remove("dark", "high-contrast");
  if (theme === "dark") root.classList.add("dark");
  else if (theme === "high-contrast") root.classList.add("high-contrast");
}

export const useUiStore = create<UiState>((set) => ({
  locale: "zh-TW",
  theme: "light",
  setLocale: (locale) => {
    i18n.changeLanguage(locale);
    set({ locale });
  },
  setTheme: (theme) => {
    applyTheme(theme);
    set({ theme });
  },
}));
