import { describe, it, expect, beforeEach } from "vitest";
import { useUiStore } from "../ui-store";

describe("useUiStore", () => {
  beforeEach(() => {
    // Reset store state
    useUiStore.setState({ locale: "zh-TW", theme: "light" });
    // Reset DOM classes
    document.documentElement.classList.remove("dark", "high-contrast");
  });

  it("starts with zh-TW locale and light theme", () => {
    const state = useUiStore.getState();
    expect(state.locale).toBe("zh-TW");
    expect(state.theme).toBe("light");
  });

  it("setLocale changes locale", () => {
    useUiStore.getState().setLocale("en");
    expect(useUiStore.getState().locale).toBe("en");
  });

  it("setLocale back to zh-TW", () => {
    useUiStore.getState().setLocale("en");
    useUiStore.getState().setLocale("zh-TW");
    expect(useUiStore.getState().locale).toBe("zh-TW");
  });

  it("setTheme changes theme to dark", () => {
    useUiStore.getState().setTheme("dark");
    expect(useUiStore.getState().theme).toBe("dark");
  });

  it("setTheme changes theme to high-contrast", () => {
    useUiStore.getState().setTheme("high-contrast");
    expect(useUiStore.getState().theme).toBe("high-contrast");
  });

  it("setTheme back to light", () => {
    useUiStore.getState().setTheme("dark");
    useUiStore.getState().setTheme("light");
    expect(useUiStore.getState().theme).toBe("light");
  });
});
