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

  it("setTheme dark adds dark class to documentElement", () => {
    useUiStore.getState().setTheme("dark");
    expect(document.documentElement.classList.contains("dark")).toBe(true);
    expect(document.documentElement.classList.contains("high-contrast")).toBe(false);
  });

  it("setTheme high-contrast adds high-contrast class to documentElement", () => {
    useUiStore.getState().setTheme("high-contrast");
    expect(document.documentElement.classList.contains("high-contrast")).toBe(true);
    expect(document.documentElement.classList.contains("dark")).toBe(false);
  });

  it("switching dark to high-contrast removes dark class", () => {
    useUiStore.getState().setTheme("dark");
    expect(document.documentElement.classList.contains("dark")).toBe(true);
    useUiStore.getState().setTheme("high-contrast");
    expect(document.documentElement.classList.contains("dark")).toBe(false);
    expect(document.documentElement.classList.contains("high-contrast")).toBe(true);
  });

  it("setTheme light removes all theme classes", () => {
    useUiStore.getState().setTheme("dark");
    useUiStore.getState().setTheme("light");
    expect(document.documentElement.classList.contains("dark")).toBe(false);
    expect(document.documentElement.classList.contains("high-contrast")).toBe(false);
  });

  it("setLocale does not affect theme", () => {
    useUiStore.getState().setTheme("dark");
    useUiStore.getState().setLocale("en");
    expect(useUiStore.getState().theme).toBe("dark");
  });
});
