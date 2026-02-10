import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { SyncIndicator } from "../sync-indicator";
import { useSyncStore } from "@/stores/sync-store";

vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

describe("SyncIndicator", () => {
  const mockFetchStatus = vi.fn();
  const mockTriggerSync = vi.fn();

  beforeEach(() => {
    vi.useFakeTimers();
    mockFetchStatus.mockClear();
    mockTriggerSync.mockClear();
    useSyncStore.setState({
      status: {
        state: "disconnected",
        last_sync_at: null,
        peer_device_name: null,
        error_message: null,
      },
      fetchStatus: mockFetchStatus,
      triggerSync: mockTriggerSync,
    });
  });

  afterEach(() => {
    vi.useRealTimers();
  });

  it("renders as a button", () => {
    render(<SyncIndicator />);
    expect(screen.getByRole("button")).toBeDefined();
  });

  it("does not show label by default", () => {
    render(<SyncIndicator />);
    expect(screen.queryByText("未連線")).toBeNull();
  });

  it("shows label when showLabel is true", () => {
    render(<SyncIndicator showLabel />);
    expect(screen.getByText("未連線")).toBeDefined();
  });

  it("shows connected label for connected state", () => {
    useSyncStore.setState({
      status: {
        state: "connected",
        last_sync_at: null,
        peer_device_name: null,
        error_message: null,
      },
    });
    render(<SyncIndicator showLabel />);
    expect(screen.getByText("已連線")).toBeDefined();
  });

  it("shows syncing label for syncing state", () => {
    useSyncStore.setState({
      status: {
        state: "syncing",
        last_sync_at: null,
        peer_device_name: null,
        error_message: null,
      },
    });
    render(<SyncIndicator showLabel />);
    expect(screen.getByText("同步中")).toBeDefined();
  });

  it("shows discovering label", () => {
    useSyncStore.setState({
      status: {
        state: "discovering",
        last_sync_at: null,
        peer_device_name: null,
        error_message: null,
      },
    });
    render(<SyncIndicator showLabel />);
    expect(screen.getByText("搜尋中")).toBeDefined();
  });

  it("shows error label for error state", () => {
    useSyncStore.setState({
      status: {
        state: "error",
        last_sync_at: null,
        peer_device_name: null,
        error_message: "fail",
      },
    });
    render(<SyncIndicator showLabel />);
    expect(screen.getByText("錯誤")).toBeDefined();
  });

  it("calls triggerSync on click", () => {
    render(<SyncIndicator />);
    fireEvent.click(screen.getByRole("button"));
    expect(mockTriggerSync).toHaveBeenCalledTimes(1);
  });

  it("calls fetchStatus on mount", () => {
    render(<SyncIndicator />);
    expect(mockFetchStatus).toHaveBeenCalled();
  });

  it("includes peer device name in title", () => {
    useSyncStore.setState({
      status: {
        state: "connected",
        last_sync_at: null,
        peer_device_name: "Baby iPad",
        error_message: null,
      },
    });
    render(<SyncIndicator />);
    const button = screen.getByRole("button");
    expect(button.getAttribute("title")).toContain("Baby iPad");
  });

  it("applies custom className", () => {
    render(<SyncIndicator className="my-custom" />);
    const button = screen.getByRole("button");
    expect(button.className).toContain("my-custom");
  });
});
