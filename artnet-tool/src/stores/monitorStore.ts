import { create } from 'zustand';

type ChannelValues = Uint8Array; // 512 bytes, 0–255 per channel

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
  updateUniverse: (universeId, data) =>
    set((state) => ({ universes: { ...state.universes, [universeId]: data } })),
  setIsActive: (isActive) => set({ isActive }),
}));
