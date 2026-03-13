// App.tsx — Application shell
// Responsibilities:
//   1. Register Tauri event listeners (once on mount) — they update Zustand store
//   2. Render the top-level layout (populated in Story 1.2)
//
// ARCHITECTURE RULE: Never import invoke() here or in any component.
//   All Tauri IPC goes through src/lib/tauri.ts only.

function App() {
  // TODO (Story 1.2): Mount AppShell with 3-tab navigation
  // TODO (Story 1.3): Register Tauri event listeners here on mount via useEffect
  //   e.g., onPlaybackStateChanged((payload) => usePlaybackStore.getState().setState(payload))

  return (
    <div className="min-h-screen bg-background text-foreground">
      <p className="p-4 text-muted-foreground">
        ARTNET-TOOL — scaffold ready. UI shell implemented in Story 1.2.
      </p>
    </div>
  );
}

export default App;
