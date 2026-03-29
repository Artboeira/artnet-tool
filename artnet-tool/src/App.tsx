// App.tsx — Application shell
// Responsibilities:
//   1. Register Tauri event listeners (once on mount) — they update Zustand store
//   2. Render the top-level layout
//
// ARCHITECTURE RULE: Never import invoke() here or in any component.
//   All Tauri IPC goes through src/lib/tauri.ts only.

import { AppShell } from '@/components/layout/AppShell';

function App() {
  // TODO (Story 1.3): Register Tauri event listeners here on mount via useEffect
  //   e.g., onPlaybackStateChanged((payload) => usePlaybackStore.getState().setState(payload))
  return <AppShell />;
}

export default App;
