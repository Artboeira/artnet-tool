import { create } from 'zustand';

type CaptureStatus = 'idle' | 'capturing' | 'error';

type CaptureState = {
  isRecording: boolean;
  captureInterface: string | null;
  captureStatus: CaptureStatus;
};

type CaptureActions = {
  setIsRecording: (isRecording: boolean) => void;
  setCaptureInterface: (captureInterface: string | null) => void;
  setCaptureStatus: (captureStatus: CaptureStatus) => void;
};

export const useCaptureStore = create<CaptureState & CaptureActions>()((set) => ({
  // State
  isRecording: false,
  captureInterface: null,
  captureStatus: 'idle',
  // Actions (stubs — real logic wired in Epic 2)
  setIsRecording: (isRecording) => set({ isRecording }),
  setCaptureInterface: (captureInterface) => set({ captureInterface }),
  setCaptureStatus: (captureStatus) => set({ captureStatus }),
}));
