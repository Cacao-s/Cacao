import { describe, it, expect, beforeEach } from "vitest";
import { useProfileStore } from "../profile-store";

describe("useProfileStore", () => {
  beforeEach(() => {
    // Reset store state between tests
    useProfileStore.setState({ profile: null, family: null });
  });

  it("starts with null profile and family", () => {
    const state = useProfileStore.getState();
    expect(state.profile).toBeNull();
    expect(state.family).toBeNull();
  });

  it("setProfile updates profile state", () => {
    const mockProfile = {
      id: 1,
      uuid: "test-uuid",
      display_name: "Alice",
      role: "giver",
      pin_hash: null,
      created_at: "2024-01-01",
      updated_at: "2024-01-01",
    };

    useProfileStore.getState().setProfile(mockProfile as never);
    expect(useProfileStore.getState().profile).toEqual(mockProfile);
  });

  it("setFamily updates family state", () => {
    const mockFamily = {
      id: 1,
      uuid: "family-uuid",
      name: "Smith Family",
      currency: "TWD",
      created_at: "2024-01-01",
      updated_at: "2024-01-01",
    };

    useProfileStore.getState().setFamily(mockFamily as never);
    expect(useProfileStore.getState().family).toEqual(mockFamily);
  });

  it("clear resets both profile and family to null", () => {
    useProfileStore.setState({
      profile: { display_name: "Alice" } as never,
      family: { name: "Test" } as never,
    });

    useProfileStore.getState().clear();

    const state = useProfileStore.getState();
    expect(state.profile).toBeNull();
    expect(state.family).toBeNull();
  });

  it("setting profile does not affect family", () => {
    const mockFamily = { name: "Test" } as never;
    useProfileStore.setState({ family: mockFamily });

    useProfileStore.getState().setProfile({ display_name: "Alice" } as never);

    expect(useProfileStore.getState().family).toEqual(mockFamily);
  });

  it("overwriting profile replaces previous value", () => {
    useProfileStore.getState().setProfile({ display_name: "Alice" } as never);
    useProfileStore.getState().setProfile({ display_name: "Bob" } as never);
    expect((useProfileStore.getState().profile as { display_name: string }).display_name).toBe(
      "Bob",
    );
  });

  it("clear then set works correctly", () => {
    useProfileStore.getState().setProfile({ display_name: "Alice" } as never);
    useProfileStore.getState().clear();
    expect(useProfileStore.getState().profile).toBeNull();

    useProfileStore.getState().setProfile({ display_name: "Bob" } as never);
    expect((useProfileStore.getState().profile as { display_name: string }).display_name).toBe(
      "Bob",
    );
  });

  it("setting family does not affect profile", () => {
    const mockProfile = { display_name: "Alice" } as never;
    useProfileStore.setState({ profile: mockProfile });

    useProfileStore.getState().setFamily({ name: "Smith" } as never);

    expect(useProfileStore.getState().profile).toEqual(mockProfile);
  });
});

// ── R46: ProfileStore Extended ──────────────────────────────

describe("useProfileStore extended", () => {
  beforeEach(() => {
    useProfileStore.setState({ profile: null, family: null });
  });

  it("multiple clears are idempotent", () => {
    useProfileStore.getState().clear();
    useProfileStore.getState().clear();
    expect(useProfileStore.getState().profile).toBeNull();
    expect(useProfileStore.getState().family).toBeNull();
  });

  it("setProfile with full Profile type", () => {
    const fullProfile = {
      id: 1,
      uuid: "uuid-1",
      display_name: "Test User",
      role: "giver",
      locale: "zh-TW",
      theme: "light",
      pin_hash: null,
      sync_version: 0,
      last_synced_at: null,
      is_deleted: 0,
      created_at: "2024-01-01",
      updated_at: "2024-01-01",
    };
    useProfileStore.getState().setProfile(fullProfile as never);
    const stored = useProfileStore.getState().profile as unknown as Record<string, unknown>;
    expect(stored.uuid).toBe("uuid-1");
    expect(stored.role).toBe("giver");
  });

  it("setFamily with full Family type", () => {
    const fullFamily = {
      id: 1,
      uuid: "fam-uuid",
      name: "Test Family",
      currency: "TWD",
      timezone: "Asia/Taipei",
      created_by_device: "device-1",
      sync_version: 0,
      last_synced_at: null,
      is_deleted: 0,
      created_at: "2024-01-01",
      updated_at: "2024-01-01",
    };
    useProfileStore.getState().setFamily(fullFamily as never);
    const stored = useProfileStore.getState().family as unknown as Record<string, unknown>;
    expect(stored.name).toBe("Test Family");
    expect(stored.currency).toBe("TWD");
  });

  it("state persists across getState calls", () => {
    useProfileStore.getState().setProfile({ display_name: "Persistent" } as never);
    const state1 = useProfileStore.getState();
    const state2 = useProfileStore.getState();
    expect(state1.profile).toEqual(state2.profile);
  });
});
