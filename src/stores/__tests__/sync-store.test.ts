import { describe, it, expect, vi, beforeEach } from "vitest";
import { useSyncStore } from "../sync-store";

// Mock @tauri-apps/api/core
vi.mock("@tauri-apps/api/core", () => ({
  invoke: vi.fn(),
}));

import { invoke } from "@tauri-apps/api/core";
const mockedInvoke = vi.mocked(invoke);

describe("useSyncStore", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    useSyncStore.setState({
      status: {
        state: "disconnected",
        last_sync_at: null,
        peer_device_name: null,
        error_message: null,
      },
    });
  });

  it("starts with disconnected state", () => {
    const state = useSyncStore.getState();
    expect(state.status.state).toBe("disconnected");
    expect(state.status.last_sync_at).toBeNull();
    expect(state.status.peer_device_name).toBeNull();
    expect(state.status.error_message).toBeNull();
  });

  it("fetchStatus updates status from invoke", async () => {
    const mockStatus = {
      state: "connected" as const,
      last_sync_at: "2024-06-15T10:00:00Z",
      peer_device_name: "Baby Device",
      error_message: null,
    };
    mockedInvoke.mockResolvedValueOnce(mockStatus);

    await useSyncStore.getState().fetchStatus();

    expect(mockedInvoke).toHaveBeenCalledWith("get_sync_status");
    expect(useSyncStore.getState().status).toEqual(mockStatus);
  });

  it("fetchStatus silently catches errors", async () => {
    mockedInvoke.mockRejectedValueOnce(new Error("Not initialized"));

    // Should not throw
    await useSyncStore.getState().fetchStatus();

    // State should remain unchanged
    expect(useSyncStore.getState().status.state).toBe("disconnected");
  });

  it("triggerSync calls invoke and updates status", async () => {
    const mockStatus = {
      state: "syncing" as const,
      last_sync_at: "2024-06-15T11:00:00Z",
      peer_device_name: "Baby Device",
      error_message: null,
    };

    mockedInvoke
      .mockResolvedValueOnce(undefined) // trigger_sync
      .mockResolvedValueOnce(mockStatus); // get_sync_status

    await useSyncStore.getState().triggerSync();

    expect(mockedInvoke).toHaveBeenCalledWith("trigger_sync");
    expect(mockedInvoke).toHaveBeenCalledWith("get_sync_status");
    expect(useSyncStore.getState().status).toEqual(mockStatus);
  });

  it("triggerSync silently catches errors", async () => {
    mockedInvoke.mockRejectedValueOnce(new Error("Sync failed"));

    await useSyncStore.getState().triggerSync();

    // State should remain unchanged
    expect(useSyncStore.getState().status.state).toBe("disconnected");
  });

  it("can set error state via setState", () => {
    useSyncStore.setState({
      status: {
        state: "error",
        last_sync_at: null,
        peer_device_name: null,
        error_message: "Connection lost",
      },
    });

    expect(useSyncStore.getState().status.state).toBe("error");
    expect(useSyncStore.getState().status.error_message).toBe("Connection lost");
  });

  it("triggerSync calls trigger_sync then get_sync_status in order", async () => {
    const callOrder: string[] = [];
    mockedInvoke.mockImplementation(async (cmd: string) => {
      callOrder.push(cmd);
      if (cmd === "get_sync_status") {
        return {
          state: "syncing",
          last_sync_at: null,
          peer_device_name: null,
          error_message: null,
        };
      }
    });

    await useSyncStore.getState().triggerSync();

    expect(callOrder).toEqual(["trigger_sync", "get_sync_status"]);
  });

  it("fetchStatus does not change state on error", async () => {
    useSyncStore.setState({
      status: {
        state: "connected",
        last_sync_at: "2024-01-01",
        peer_device_name: "Device",
        error_message: null,
      },
    });
    mockedInvoke.mockRejectedValueOnce(new Error("fail"));

    await useSyncStore.getState().fetchStatus();

    expect(useSyncStore.getState().status.state).toBe("connected");
    expect(useSyncStore.getState().status.peer_device_name).toBe("Device");
  });

  it("triggerSync when get_sync_status fails preserves previous state", async () => {
    useSyncStore.setState({
      status: {
        state: "connected",
        last_sync_at: "2024-01-01",
        peer_device_name: "Peer",
        error_message: null,
      },
    });
    mockedInvoke
      .mockResolvedValueOnce(undefined) // trigger_sync ok
      .mockRejectedValueOnce(new Error("status fail")); // get_sync_status fails

    await useSyncStore.getState().triggerSync();

    // The whole try/catch wraps both calls, so state stays unchanged
    expect(useSyncStore.getState().status.state).toBe("connected");
  });
});
