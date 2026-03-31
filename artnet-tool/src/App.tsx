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
    // Accumulate unlisten functions so cleanup can call them all on unmount.
    // useEffect callbacks are synchronous — listen() calls go inside an async IIFE.
    const unlistenFns: Array<() => void> = [];

    (async () => {
      // Pattern for each listener (import helpers from src/lib/tauri.ts):
      //   unlistenFns.push(await onEventName((payload) => useStore.getState().action(payload)));
      //
      // TODO (Story 2.2): listen for 'capture-started', 'capture-stopped' → useCaptureStore
      // TODO (Story 2.3): listen for 'monitor-update' → useMonitorStore.getState().updateUniverse()
      // TODO (Story 3.2): listen for 'playback-state-changed' → usePlaybackStore
      // TODO (Story 1.4): listen for 'error-occurred' → useErrorStore.getState().setError()
    })();

    return () => {
      unlistenFns.forEach((fn) => fn());
    };
  }, []);

  return <AppShell />;
}

export default App;
