import { create } from "zustand";
import type { Profile, Family } from "@/lib/types";

interface ProfileState {
  profile: Profile | null;
  family: Family | null;
  setProfile: (p: Profile) => void;
  setFamily: (f: Family) => void;
  clear: () => void;
}

export const useProfileStore = create<ProfileState>((set) => ({
  profile: null,
  family: null,
  setProfile: (profile) => set({ profile }),
  setFamily: (family) => set({ family }),
  clear: () => set({ profile: null, family: null }),
}));
