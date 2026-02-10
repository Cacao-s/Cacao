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
