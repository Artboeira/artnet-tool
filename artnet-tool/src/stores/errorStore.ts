import { create } from 'zustand';

type ErrorState = {
  currentError: string | null;
};

type ErrorActions = {
  setError: (message: string) => void;
  clearError: () => void;
};

export const useErrorStore = create<ErrorState & ErrorActions>()((set) => ({
  currentError: null,
  setError: (message) => set({ currentError: message }),
  clearError: () => set({ currentError: null }),
}));
