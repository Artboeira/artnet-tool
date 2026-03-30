// App.tsx — Application shell
// Responsibilities:
//   1. Register Tauri event listeners (once on mount) — they update Zustand store
//   2. Render the top-level layout
//
// ARCHITECTURE RULE: Never import invoke() here or in any component.
//   All Tauri IPC goes through src/lib/tauri.ts only.

import { useEffect } from 'react';
import { AppShell } from '@/components/layout/AppShell';

function App() {
  useEffect(() => {
    // Event listeners registered here on mount; each returns an UnlistenFn for cleanup.
    // Pattern: const unlisten = await onEventName((payload) => useStore.getState().action(payload))
    //
    // TODO (Story 2.2): listen for 'capture-started', 'capture-stopped' → useCaptureStore
    // TODO (Story 2.3): listen for 'monitor-update' → useMonitorStore.updateUniverse()
    // TODO (Story 3.2): listen for 'playback-state-changed' → usePlaybackStore
    // TODO (Story 1.4): listen for 'error-occurred' → useErrorStore.setError()
    //
    // All listen() calls live in src/lib/tauri.ts — import helpers from there.
    return () => {
      // Call unlisten() for each active listener on unmount
    };
  }, []);

  return <AppShell />;
}

export default App;
