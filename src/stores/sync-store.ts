import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";

export type SyncState = "disconnected" | "discovering" | "connected" | "syncing" | "error";

interface SyncStatus {
  state: SyncState;
  last_sync_at: string | null;
  peer_device_name: string | null;
  error_message: string | null;
}

interface SyncStore {
  status: SyncStatus;
  fetchStatus: () => Promise<void>;
  triggerSync: () => Promise<void>;
}

export const useSyncStore = create<SyncStore>((set) => ({
  status: {
    state: "disconnected",
    last_sync_at: null,
    peer_device_name: null,
    error_message: null,
  },
  fetchStatus: async () => {
    try {
      const status = await invoke<SyncStatus>("get_sync_status");
      set({ status });
    } catch {
      // Sync coordinator may not be initialized yet
    }
  },
  triggerSync: async () => {
    try {
      await invoke("trigger_sync");
      const status = await invoke<SyncStatus>("get_sync_status");
      set({ status });
    } catch {
      // Sync may fail silently
    }
  },
}));
