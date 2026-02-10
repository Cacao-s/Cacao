import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import { LockGuard } from "../lock-guard";

const mockListen = vi.fn();

vi.mock("@tauri-apps/api/event", () => ({
  listen: (...args: unknown[]) => mockListen(...args),
}));

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

vi.mock("@/app/lock-screen", () => ({
  LockScreen: ({ onUnlock }: { onUnlock: () => void }) => (
    <div data-testid="lock-screen">
      <button onClick={onUnlock}>Unlock</button>
    </div>
  ),
}));

describe("LockGuard", () => {
  beforeEach(() => {
    mockListen.mockReset();
    mockListen.mockResolvedValue(vi.fn());
  });

  it("renders children when not locked", () => {
    render(
      <LockGuard>
        <div data-testid="child">Hello</div>
      </LockGuard>,
    );
    expect(screen.getByTestId("child")).toBeDefined();
    expect(screen.getByText("Hello")).toBeDefined();
  });

  it("does not show lock screen initially", () => {
    render(
      <LockGuard>
        <div>Content</div>
      </LockGuard>,
    );
    expect(screen.queryByTestId("lock-screen")).toBeNull();
  });

  it("listens for tauri://resumed event", () => {
    render(
      <LockGuard>
        <div>Content</div>
      </LockGuard>,
    );
    expect(mockListen).toHaveBeenCalledWith("tauri://resumed", expect.any(Function));
  });
});
