import { create } from 'zustand';

// Tauri serializes event payloads as JSON — channels arrive as number[], not Uint8Array.
// If a Uint8Array view is needed for performance in a component, convert at the use site:
//   const channelBytes = new Uint8Array(universeData.channels);
type ChannelValues = number[]; // 512 values, 0–255 per DMX channel

type UniverseData = {
  universeId: number;
  channels: ChannelValues;
  lastUpdated: number; // Date.now() timestamp
};

type MonitorState = {
  universes: Record<number, UniverseData>;
  isActive: boolean;
};

type MonitorActions = {
  updateUniverse: (universeId: number, data: UniverseData) => void;
  setIsActive: (isActive: boolean) => void;
};

export const useMonitorStore = create<MonitorState & MonitorActions>()((set) => ({
  // State
  universes: {},
  isActive: false,
  // Actions (stubs — real high-frequency updates wired in Epic 2, Story 2.3)
  //
  // PERFORMANCE NOTE (Story 2.3): updateUniverse spreads the top-level universes object
  // on every call (~30fps). Components that need a single universe should subscribe with
  // a selector to avoid re-rendering on unrelated universe updates:
  //   const universe1 = useMonitorStore((s) => s.universes[1]);
  // Avoid subscribing to the whole universes map in hot render paths.
  updateUniverse: (universeId, data) =>
    set((state) => ({ universes: { ...state.universes, [universeId]: data } })),
  setIsActive: (isActive) => set({ isActive }),
}));
