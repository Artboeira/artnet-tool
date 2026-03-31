# Story 1.3: Zustand State Management Scaffold

Status: done

## Story

As a developer,
I want all 6 Zustand store slices (playback, capture, scenes, monitor, settings, errors) defined with typed initial state and action stubs,
so that feature development can import and extend state without defining slice shapes from scratch.

## Acceptance Criteria

1. **Given** the store scaffold is implemented, **When** a developer imports any slice (e.g., `usePlaybackStore`, `useCaptureStore`), **Then** the import resolves without TypeScript errors, **And** the initial state shape is defined according to the architecture slice structure.

2. **Given** the `errors` slice, **When** an error action is dispatched, **Then** the error state is updated and the value is accessible for the error toast system to consume.

3. **Given** all 6 store slice files, **When** TypeScript compilation runs (`npx tsc --noEmit`), **Then** there are zero type errors related to store definitions.

## Tasks / Subtasks

- [x] Task 1: Install Zustand (AC: #1, #3)
  - [x] Verify `zustand` is listed in `package.json` dependencies (already at `^5.0.11`)
  - [x] Run `npm install` if `node_modules/zustand` not present

- [x] Task 2: Create `src/stores/playbackStore.ts` (AC: #1, #3)
  - [x] Define `PlaybackState` type: `isPlaying`, `currentSceneId`, `playbackMode`, `speed`
  - [x] Define `PlaybackActions` type with stub action signatures
  - [x] Create store with `create<PlaybackState & PlaybackActions>()(...)` pattern
  - [x] Set typed initial state with zero/null/false defaults

- [x] Task 3: Create `src/stores/captureStore.ts` (AC: #1, #3)
  - [x] Define `CaptureState` type: `isRecording`, `captureInterface`, `captureStatus`
  - [x] Define `CaptureActions` with stub action signatures
  - [x] Create store with typed initial state

- [x] Task 4: Create `src/stores/sceneStore.ts` (AC: #1, #3)
  - [x] Define `SceneState` type: `scenes[]`, `activeSceneId`, `isLoading`
  - [x] Define `SceneActions` with stub action signatures
  - [x] Create store with typed initial state

- [x] Task 5: Create `src/stores/monitorStore.ts` (AC: #1, #3)
  - [x] Define `MonitorState` type: `universes` record keyed by universe number, `isActive`
  - [x] Define `MonitorActions` with `updateUniverse` stub
  - [x] Create store with typed initial state (empty universes map)

- [x] Task 6: Create `src/stores/settingsStore.ts` (AC: #1, #3)
  - [x] Define `SettingsState` type: `networkInterfaces`, `midiDevices`, `bootConfig`
  - [x] Define `SettingsActions` with stub action signatures
  - [x] Create store with typed initial state

- [x] Task 7: Create `src/stores/errorStore.ts` (AC: #1, #2, #3)
  - [x] Define `ErrorState` type: `currentError: string | null`
  - [x] Define `ErrorActions`: `setError(msg: string): void`, `clearError(): void`
  - [x] Implement `setError` and `clearError` as the only two concrete actions in this story
  - [x] Create store with `currentError: null` as initial state

- [x] Task 8: Create `src/stores/index.ts` (AC: #1, #3)
  - [x] Re-export all 6 store hooks from a single barrel file
  - [x] Delete `src/stores/.gitkeep` after adding real files

- [x] Task 9: Wire Tauri event listeners in `App.tsx` (AC: #1)
  - [x] Add `useEffect` import to `App.tsx`
  - [x] Add stub `useEffect` with cleanup skeleton — comment shows where each subsystem's listener will be registered
  - [x] Keep actual `listen()` calls stubbed/commented (real listeners come in each subsystem's epic)

- [x] Task 10: Final validation (AC: #3)
  - [x] Run `npx tsc --noEmit` — zero TypeScript errors
  - [x] Run `npm test` — existing AppShell.test.tsx still passes
  - [x] Confirm imports work: e.g., `import { useErrorStore } from '@/stores'` resolves

## Dev Notes

Zustand **v5.0.11** is already installed (see `artnet-tool/package.json`). Do NOT run `npm install zustand` — it is already present.

### Zustand v5 API Pattern

Zustand v5 uses the curried `create` form for TypeScript. The recommended pattern:

```typescript
// src/stores/playbackStore.ts
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
```

Key v5 notes:
- Use `create<T>()((set, get) => ...)` — the double-call (curried) form is required for TypeScript type inference in v5
- No `immer` middleware needed for this story — simple `set({ field })` is sufficient for all slices
- `get` is available if a setter needs to read current state, but not required in this scaffold

### All 6 Slice Shapes (from architecture.md)

```
playbackStore.ts  → isPlaying: boolean, currentSceneId: string | null, playbackMode: PlaybackMode, speed: number
captureStore.ts   → isRecording: boolean, captureInterface: string | null, captureStatus: 'idle' | 'capturing' | 'error'
sceneStore.ts     → scenes: Scene[], activeSceneId: string | null, isLoading: boolean
monitorStore.ts   → universes: Record<number, UniverseData>, isActive: boolean
settingsStore.ts  → networkInterfaces: NetworkInterface[], midiDevices: MidiDevice[], bootConfig: BootConfig
errorStore.ts     → currentError: string | null
```

Use placeholder/stub types for domain types (Scene, NetworkInterface, MidiDevice, etc.) — these will be filled in by their respective feature epics. The simplest approach is to use minimal inline types or `unknown` with a TODO comment, but prefer defining at least a skeleton so TypeScript doesn't infer `never`.

### errorStore — Only Concrete Actions Needed

The `errorStore` is the only store where real action logic is needed in this story (AC #2):

```typescript
// src/stores/errorStore.ts
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
```

This is also the store imported in `src/lib/tauri.ts` for error handling at the IPC boundary — the commented-out `useErrorStore.getState().setError(err as string)` line in `tauri.ts` can now be uncommented in the IPC wrapper template.

### Barrel File Pattern

```typescript
// src/stores/index.ts
export { usePlaybackStore } from './playbackStore';
export { useCaptureStore } from './captureStore';
export { useSceneStore } from './sceneStore';
export { useMonitorStore } from './monitorStore';
export { useSettingsStore } from './settingsStore';
export { useErrorStore } from './errorStore';
```

### App.tsx Event Listener Skeleton

Architecture mandates that Tauri event listeners are registered **once** on app mount in `App.tsx`. This story adds the skeleton `useEffect` with cleanup — actual `listen()` calls are stubbed as comments pointing forward to their respective stories:

```tsx
// src/App.tsx
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
```

Do NOT add actual `listen()` calls yet — they require backend events that don't exist until later stories.

### monitorStore — High-Frequency Update Note

The `monitorStore` will be updated at ~30 fps by the monitor subsystem (Epic 2, Story 2.3). For the scaffold, just define the shape. Do NOT add any polling logic, timers, or WebSocket connections. The architecture note about 30fps concerns performance optimization that belongs in Story 2.3.

Suggested minimal type for `UniverseData` in the scaffold:

```typescript
type ChannelValues = Uint8Array; // 512 bytes, 0–255 per channel

type UniverseData = {
  universeId: number;
  channels: ChannelValues;
  lastUpdated: number; // Date.now() timestamp
};
```

Using `Uint8Array` for 512 channels is correct and matches the ArtNet DMX frame size.

### File Locations

Files to **create**:
```
artnet-tool/src/
  stores/
    index.ts              ← barrel: re-exports all 6 hooks
    playbackStore.ts      ← isPlaying, currentSceneId, playbackMode, speed
    captureStore.ts       ← isRecording, captureInterface, captureStatus
    sceneStore.ts         ← scenes[], activeSceneId, isLoading
    monitorStore.ts       ← universes{}, isActive
    settingsStore.ts      ← networkInterfaces, midiDevices, bootConfig
    errorStore.ts         ← currentError: string | null; setError / clearError
```

Files to **modify**:
```
artnet-tool/src/App.tsx   ← add useEffect import + event listener skeleton
artnet-tool/src/stores/.gitkeep  ← DELETE after first real file added
```

Files **NOT to touch**:
- `src/lib/tauri.ts` — no new IPC functions in this story (keep the `_IPC_BOUNDARY` export and `void invoke` until story 1.4)
- `src/types/*.ts` — placeholder type files filled in by their respective stories
- All files under `src/components/`, `src/features/` — not in scope
- All Rust files in `src-tauri/` — no backend changes in this story

### Architecture Compliance Checklist

- [x] All store files live in `src/stores/` — one file per slice
- [x] Slices composed/exported from `src/stores/index.ts`
- [x] No `invoke()` in any store file — all IPC goes through `src/lib/tauri.ts`
- [x] No `I` prefix on TypeScript interfaces/types
- [x] Zustand store hooks named `useXxxStore` (camelCase as per arch rules)
- [x] Tauri event listeners registered ONCE in `App.tsx` (skeleton only in this story)
- [x] `npx tsc --noEmit` passes with zero errors

### Previous Story Intelligence (Stories 1.1 & 1.2)

From the Story 1.2 dev record:

1. **Vitest is configured** — `vite.config.ts` has test config, `src/test/setup.ts` exists. Tests run with `npm test`. The pattern for co-located tests is `ComponentName.test.tsx` alongside the file.
2. **`class-variance-authority` was NOT auto-installed by shadcn CLI** — needed `npm install class-variance-authority` manually. Relevant if any new shadcn components are added (not needed in this story).
3. **`src/App.tsx` currently has a TODO (Story 1.3) comment** — this is the exact hook point where the `useEffect` event listener skeleton should go.
4. **`@/` path alias resolves to `src/`** — use `import { useErrorStore } from '@/stores'` everywhere.
5. **`src/lib/tauri.ts`** has `_IPC_BOUNDARY` export and `void invoke` — leave these in place for this story.

### Git Intelligence

Recent commits:
- `e3880f8` — 1-2 story complete (Story 1.2 implementation + code review fixes: AppShell w-full, mt-0, runtime TabId guard, bg-background on TabsList, Vitest infrastructure)
- `e21fd76` — backup
- `f0c9ee9` — create README
- `b5d6bfd` — initial Tauri scaffold with full architecture structure (Story 1.1)

The repository is in a clean state post-Story-1.2. `stores/` directory exists with only `.gitkeep`.

### Testing Requirements

For this scaffold story, minimal testing is required. The acceptance criteria is TypeScript compilation passing and imports resolving. However, a small test for `errorStore` (the only store with real action logic per AC #2) is recommended:

```typescript
// src/stores/errorStore.test.ts
import { describe, it, expect, beforeEach } from 'vitest';
import { useErrorStore } from './errorStore';

describe('errorStore', () => {
  beforeEach(() => {
    useErrorStore.setState({ currentError: null });
  });

  it('setError updates currentError', () => {
    useErrorStore.getState().setError('Test error');
    expect(useErrorStore.getState().currentError).toBe('Test error');
  });

  it('clearError resets to null', () => {
    useErrorStore.getState().setError('Something went wrong');
    useErrorStore.getState().clearError();
    expect(useErrorStore.getState().currentError).toBeNull();
  });
});
```

Note: `useErrorStore.setState` is the Zustand built-in for test state reset — no mock needed.

### Project Structure Notes

- All stores are frontend-only TypeScript files; no Tauri/Rust interaction in this story
- `stores/.gitkeep` must be deleted once the first real store file is created (convention from previous stories — see Story 1.2 where `components/layout/.gitkeep` was deleted)
- The `src/types/scene.ts`, `src/types/events.ts`, and `src/types/project.ts` files are currently empty stubs (`export {}`). If any store type references these (e.g., `Scene` from `@/types/scene`), add the minimal stub inline in the store file for now rather than modifying the shared types files (those are populated story-by-story in their respective epics).

### References

- Story AC source: [Source: _bmad-output/planning-artifacts/epics.md#Story 1.3]
- Zustand store rules: [Source: _bmad-output/planning-artifacts/architecture.md#Zustand Store Rules]
- Slice shapes: [Source: _bmad-output/planning-artifacts/architecture.md#Complete Project Directory Structure]
- App.tsx event listener pattern: [Source: _bmad-output/planning-artifacts/architecture.md#Communication Patterns]
- IPC boundary rule: [Source: _bmad-output/planning-artifacts/architecture.md#Enforcement Guidelines]
- Naming conventions: [Source: _bmad-output/planning-artifacts/architecture.md#Naming Patterns]
- Previous story notes: [Source: _bmad-output/implementation-artifacts/1-2-ui-shell-dark-theme-shadcn-ui-and-3-tab-navigation.md#Dev Agent Record]

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

_No debug issues encountered._

### Completion Notes List

- Created all 6 Zustand v5 store slices with typed state and action stubs in `artnet-tool/src/stores/`
- `errorStore` has concrete `setError`/`clearError` implementations (only store with real logic per AC #2)
- All stores use the curried `create<T>()((set) => ...)` pattern required for TypeScript inference in v5
- Barrel file `src/stores/index.ts` re-exports all 6 hooks; `.gitkeep` deleted
- `App.tsx` updated with `useEffect` skeleton and cleanup, TODO comments pointing to future stories
- `npx tsc --noEmit` passes with zero errors
- `npm test` — 7/7 tests pass (2 files): existing `AppShell.test.tsx` + new `errorStore.test.ts`
- All architecture compliance rules satisfied (no `invoke()` in stores, no `I` prefix, correct naming)

**Code Review Fixes (2026-03-30):**
- M1 Fixed: `App.tsx` — replaced misleading `await` comment with correct async IIFE pattern; added `unlistenFns` array for proper cleanup accumulation
- M2 Fixed: `monitorStore.ts` — added performance note documenting selector pattern (`useMonitorStore((s) => s.universes[id])`) for 30fps subscription paths
- M3 Fixed: `monitorStore.ts` — changed `ChannelValues` from `Uint8Array` to `number[]` to match Tauri JSON IPC payload; added conversion note for components needing `Uint8Array` views

### File List

- `artnet-tool/src/stores/playbackStore.ts` — created
- `artnet-tool/src/stores/captureStore.ts` — created
- `artnet-tool/src/stores/sceneStore.ts` — created
- `artnet-tool/src/stores/monitorStore.ts` — created (modified in code review: channels type + selector docs)
- `artnet-tool/src/stores/settingsStore.ts` — created
- `artnet-tool/src/stores/errorStore.ts` — created
- `artnet-tool/src/stores/errorStore.test.ts` — created
- `artnet-tool/src/stores/index.ts` — created
- `artnet-tool/src/stores/.gitkeep` — deleted
- `artnet-tool/src/App.tsx` — modified (useEffect async IIFE pattern + unlisten cleanup; updated in code review)

### Change Log

- 2026-03-29: Story 1.3 implemented — all 6 Zustand store slices scaffolded, barrel index created, App.tsx event listener skeleton added, errorStore tests passing
- 2026-03-30: Code review fixes — App.tsx async IIFE pattern corrected, monitorStore channels type changed to number[] for Tauri JSON compat, selector performance note added
