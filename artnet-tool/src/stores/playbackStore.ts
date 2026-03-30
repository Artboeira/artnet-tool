import { create } from 'zustand';

type PlaybackMode = 'loop' | 'once' | 'bounce'; // placeholder — extend in Epic 3

type PlaybackState = {
  isPlaying: boolean;
  currentSceneId: string | null;
  playbackMode: PlaybackMode;
  speed: number; // 0.25–4.0, default 1.0
  isLoading: boolean;
};

type PlaybackActions = {
  setIsPlaying: (isPlaying: boolean) => void;
  setCurrentSceneId: (sceneId: string | null) => void;
  setPlaybackMode: (mode: PlaybackMode) => void;
  setSpeed: (speed: number) => void;
  setIsLoading: (isLoading: boolean) => void;
};

export const usePlaybackStore = create<PlaybackState & PlaybackActions>()((set) => ({
  // State
  isPlaying: false,
  currentSceneId: null,
  playbackMode: 'once',
  speed: 1.0,
  isLoading: false,
  // Actions (stubs — real logic wired in Epic 3)
  setIsPlaying: (isPlaying) => set({ isPlaying }),
  setCurrentSceneId: (currentSceneId) => set({ currentSceneId }),
  setPlaybackMode: (playbackMode) => set({ playbackMode }),
  setSpeed: (speed) => set({ speed }),
  setIsLoading: (isLoading) => set({ isLoading }),
}));
