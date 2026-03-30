import { create } from 'zustand';

// Minimal stub types — expanded by their respective feature epics
type NetworkInterface = {
  id: string;
  name: string;
  // TODO (Epic 2): expand with IP, MAC, etc.
};

type MidiDevice = {
  id: string;
  name: string;
  // TODO (Epic 4): expand with port, channel, etc.
};

type BootConfig = {
  autoStart: boolean;
  autoLoad: boolean;
  // TODO (Epic 6): expand with project path, auto-play, etc.
};

type SettingsState = {
  networkInterfaces: NetworkInterface[];
  midiDevices: MidiDevice[];
  bootConfig: BootConfig;
};

type SettingsActions = {
  setNetworkInterfaces: (networkInterfaces: NetworkInterface[]) => void;
  setMidiDevices: (midiDevices: MidiDevice[]) => void;
  setBootConfig: (bootConfig: BootConfig) => void;
};

export const useSettingsStore = create<SettingsState & SettingsActions>()((set) => ({
  // State
  networkInterfaces: [],
  midiDevices: [],
  bootConfig: { autoStart: false, autoLoad: false },
  // Actions (stubs — real logic wired in Epics 2, 4, 6)
  setNetworkInterfaces: (networkInterfaces) => set({ networkInterfaces }),
  setMidiDevices: (midiDevices) => set({ midiDevices }),
  setBootConfig: (bootConfig) => set({ bootConfig }),
}));
