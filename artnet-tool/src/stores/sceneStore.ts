import { create } from 'zustand';

// Minimal Scene stub — full type filled in by Epic 3
type Scene = {
  id: string;
  name: string;
  // TODO (Epic 3): expand with DMX frames, duration, etc.
};

type SceneState = {
  scenes: Scene[];
  activeSceneId: string | null;
  isLoading: boolean;
};

type SceneActions = {
  setScenes: (scenes: Scene[]) => void;
  setActiveSceneId: (sceneId: string | null) => void;
  setIsLoading: (isLoading: boolean) => void;
};

export const useSceneStore = create<SceneState & SceneActions>()((set) => ({
  // State
  scenes: [],
  activeSceneId: null,
  isLoading: false,
  // Actions (stubs — real logic wired in Epic 3)
  setScenes: (scenes) => set({ scenes }),
  setActiveSceneId: (activeSceneId) => set({ activeSceneId }),
  setIsLoading: (isLoading) => set({ isLoading }),
}));
