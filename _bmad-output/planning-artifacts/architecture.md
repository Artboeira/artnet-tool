---
stepsCompleted: ['step-01-init', 'step-02-context', 'step-03-starter', 'step-04-decisions', 'step-05-patterns', 'step-06-structure', 'step-07-validation', 'step-08-complete']
lastStep: 8
status: complete
completedAt: '2026-03-12'
inputDocuments:
  - '_bmad-output/planning-artifacts/prd.md'
  - '_bmad-output/planning-artifacts/ux-design-specification.md'
  - '_bmad-output/planning-artifacts/product-brief-ARTNET-TOOL-2026-03-06.md'
  - '_bmad-output/planning-artifacts/implementation-readiness-report-2026-03-06.md'
workflowType: 'architecture'
project_name: 'ARTNET-TOOL'
user_name: 'NODATA'
date: '2026-03-08'
---

# Architecture Decision Document

_This document builds collaboratively through step-by-step discovery. Sections are appended as we work through each architectural decision together._

---

## Project Context Analysis

### Requirements Overview

**Functional Requirements:**

42 FRs spanning 8 functional domains:

| Domain | FRs | Architectural Implication |
|---|---|---|
| Signal Capture (passive) | FR1–FR6 | Npcap/libpcap subsystem; platform-specific; highest-risk |
| Scene Management | FR7–FR10 | Scene data model; in-memory store; file persistence |
| Playback Engine | FR13–FR17 | Isolated real-time thread; frame-accurate timer; ArtNet/sACN output |
| Triggering & Control | FR11–FR12, FR18–FR21 | Keyboard, MIDI (hot-plug), UI trigger; cross-subsystem event bus |
| Scheduling | FR22–FR23 | Time-based scheduler; no external dependency |
| Signal Monitoring | FR24–FR27 | Reactive UI binding; separate rendering path from playback |
| Project Management | FR28–FR33 | Schema-validated file format; portable; forward-compatible |
| System & Boot Integration | FR34–FR37 | OS-specific: Task Scheduler / LaunchAgent / systemd |
| Diagnostics & Recovery | FR38–FR42 | Persistent log; watchdog; plain-language error messages |

**Non-Functional Requirements:**

NFRs that will directly drive architectural decisions:

| NFR | Constraint | Architectural Impact |
|---|---|---|
| NFR1 | ±1ms playback timing precision | Dedicated OS thread with real-time priority; no shared scheduling with UI |
| NFR4 | <15% CPU, <300MB RAM on Raspberry Pi 4 | Eliminates Electron (Chromium ~150MB baseline); Tauri or native strongly favored |
| NFR5 | UI must not affect playback engine | Process or thread isolation required; IPC or shared memory for state sync |
| NFR7 | 30 days unattended continuous operation | No memory leaks; resource budget enforcement; log rotation required |
| NFR8/9 | Auto-recover <60s, DMX gap <60s | Watchdog process independent of main app process |
| NFR10 | Recover from unplanned shutdown on next boot | Persistent project state file; auto-start + auto-load on boot |
| NFR17 | Identical behavior on Win/macOS/Linux ARM | Cross-platform abstraction layer for capture, MIDI, and boot subsystems |
| NFR20/21 | No telemetry; schema-validated project files | Fully offline; sandboxed parser; no eval or arbitrary execution from files |

**Scale & Complexity:**

- Primary domain: Desktop application + real-time network I/O + OS integration
- Complexity level: **Medium** (complex per-subsystem; manageable with clear isolation)
- Estimated architectural components: **9–11 distinct subsystems**
- Real-time requirements: Yes — ±1ms packet timing, live DMX monitor refresh
- External integrations: OS-level only (no cloud, no APIs, fully offline)
- Multi-platform: High — Windows x64, macOS Universal Binary (Intel + Apple Silicon), Linux ARM64

### Technical Constraints & Dependencies

**Platform-Specific:**
- Windows 10+: Npcap driver required; admin install; firewall rule injection; Task Scheduler for auto-start
- macOS 12+ (Monterey): libpcap built-in; Universal Binary (Intel + Apple Silicon); notarization required; LaunchAgent plist for auto-start
- Linux ARM64 (RPi 4+): libpcap; headless/no-display capability required; systemd service for auto-start; Debian/Ubuntu

**Runtime Constraints:**
- Scene data must be loaded fully into memory at project load (not streamed) — impacts max project file size
- No internet dependency for any core operation — update check is notification-only
- Admin privileges required on Windows for packet capture driver installation
- UI rendering must not compete with playback engine for CPU

**Integration Dependencies:**
- Npcap (Windows) / libpcap (macOS, Linux) — passive packet capture
- MIDI 1.0 — Note On/Off, CC, hot-plug detection across all platforms
- ArtNet UDP port 6454 / sACN UDP port 5568 + multicast
- OS boot registration: 3 distinct mechanisms across 3 platforms
- Project file: schema-validated, no arbitrary code execution, forward-compatible

### Cross-Cutting Concerns Identified

1. **Thread/process isolation** — playback engine timing must be unaffected by UI, capture, MIDI, or scheduler activity
2. **Cross-platform abstraction** — capture (Npcap/libpcap), MIDI, boot registration each need a platform adapter pattern
3. **Resource budget enforcement** — CPU + RAM ceiling must be maintained across 30-day unattended operation
4. **Error messaging pipeline** — all subsystem errors must surface as plain-language actionable messages (NFR14); no raw stack traces to users
5. **Project file schema and versioning** — forward-compatible; sandboxed parser; portable across all three platforms
6. **Watchdog / crash recovery** — must operate independently of the main application process
7. **Log rotation** — unaddressed in PRD; required for 30-day unattended deployment on low-disk hardware

---

## Starter Template Evaluation

### Primary Technology Domain

Desktop application (cross-platform: Windows x64, macOS Universal Binary Intel + Apple Silicon, Linux ARM64)

### Framework Decision: Tauri 2 (selected) vs Electron (eliminated)

Electron is disqualified by NFR4 (<300MB RAM on RPi 4): Electron's Chromium runtime alone consumes 200–500MB at idle, leaving insufficient headroom for the playback engine and scene data. Tauri 2 idles at 30–50MB total.

| Metric | Electron | Tauri 2 |
|---|---|---|
| Idle RAM | 200–500MB | 30–50MB |
| Startup time | 2–4 seconds | <0.5 seconds |
| Idle CPU | 2–5% | 0–1% |
| Binary size | 80–120MB | <10MB |

Additional Tauri advantages for ARTNET-TOOL:
- Process isolation between Rust backend and React UI enforces NFR5 architecturally — UI freezes cannot affect the playback engine
- Startup time <0.5s contributes directly to NFR3 (30s boot-to-playback target)
- Idle CPU 0–1% supports NFR4 (15% ceiling), leaving the budget for real work

### Backend Language: Rust

Rust is uniquely suited to this project's real-time and reliability requirements:

- **No garbage collector** — eliminates GC pauses that would violate ±1ms timing (NFR1)
- **Compile-time thread safety** — data races between playback/capture/MIDI/scheduler threads are impossible to compile; directly enforces NFR5 isolation
- **Direct OS thread priority APIs** — `SCHED_FIFO` on Linux, `SetThreadPriority` on Windows for real-time playback thread
- **Native crate ecosystem:**
  - `pcap` — libpcap/Npcap bindings for passive packet capture
  - `midir` — MIDI 1.0 hot-plug detection, cross-platform
  - `serde` / `serde_json` — schema-validated project file serialization (NFR21)
  - `tracing` / `tracing-appender` — structured logging with file rotation (NFR38)

### Frontend: React 19 + TypeScript + Vite

React retained for developer familiarity. No significant advantage from switching to Svelte or Solid inside Tauri's webview — the performance delta is negligible and React expertise reduces implementation risk. TypeScript enforces correctness across the IPC boundary between Rust backend and React UI.

### Selected Starter: create-tauri-app v2.10.1

**Rationale:** Official Tauri scaffold with React + TypeScript + Vite. Actively maintained (v2.10.1, March 2026). Provides correct cross-platform build configuration and IPC wiring out of the box.

**Initialization Command:**

```bash
npm create tauri-app@latest artnet-tool -- --template react-ts
```

**Architectural Decisions Provided by Starter:**

**Language & Runtime:** Rust (backend) + TypeScript/React 19 (frontend)

**Build Tooling:** Vite 6 (frontend HMR + production bundle) + Cargo (Rust backend) + Tauri CLI 2.10.1

**IPC Layer:** Tauri commands (`invoke`) and events (`listen`/`emit`) — type-safe bridge between Rust and React

**Project Structure:**
- `src/` — React/TypeScript UI components
- `src-tauri/` — Rust backend (main.rs, lib.rs, Cargo.toml, tauri.conf.json)
- `src-tauri/src/` — Rust modules: playback, capture, midi, scheduler, project, boot, watchdog, logging

**Testing Framework:** Vitest (frontend unit tests) + Rust built-in test harness (backend unit/integration tests)

**Note:** Project initialization using this command is the first implementation story.

---

## Core Architectural Decisions

### Decision Priority Analysis

**Critical Decisions (Block Implementation):**
- Playback engine threading model (spin_sleep isolated thread + Tokio async)
- OS service layer as unified watchdog + auto-start mechanism
- Project file format (JSON + schema_version migration chain)

**Important Decisions (Shape Architecture):**
- UI framework stack (shadcn/ui + Tailwind CSS + Zustand + React 19)
- CI/CD pipeline (Tauri GitHub Action matrix)
- IPC pattern (Tauri commands for request/response; events for real-time push)

**Deferred Decisions (Post-MVP):**
- macOS notarization: Apple Developer account not yet active; CI pipeline built with notarization step stubbed; activated when account is obtained
- MessagePack compression: project files use JSON; binary compression can be added if file size becomes a problem at scale

---

### Data Architecture

**Project File Format: JSON with schema_version**
- Format: UTF-8 JSON; parsed with `serde_json`; validated against typed Rust structs via `serde` deserialization
- Root field `schema_version: u32` required on all files
- Migration chain: `migrate_v1_to_v2(Value) -> Value` functions applied sequentially on load before schema validation
- Schema validation: deserialization into typed Rust structs acts as implicit schema validation; unknown fields rejected via `deny_unknown_fields`
- No arbitrary code execution possible from project files (NFR21 enforced by serde's data-only parsing)
- Portable across all three platforms with no path or platform encoding assumptions in file content (NFR18)

**In-Memory Scene Store:**
- Scenes loaded fully into memory at project load (per PRD technical constraint)
- Rust type: `Arc<RwLock<ProjectState>>` shared across subsystems
- Write access: only project manager and capture subsystem
- Read access: playback engine, scheduler, UI (via IPC)

---

### Security

**Project File Safety (NFR20/21):**
- `serde` deserialization is data-only; no `eval`, no code execution paths
- `deny_unknown_fields` attribute on all deserialized structs rejects unexpected keys
- File integrity validated at load: structural parse error → specific actionable error to user (NFR14)

**Network Isolation (NFR22):**
- No open ports beyond ArtNet UDP 6454 and sACN UDP 5568
- No telemetry, no analytics, no outbound connections
- Update check: version comparison only against a static file; no code download

---

### IPC & Communication Patterns

**Tauri IPC Architecture:**
- **Commands** (`invoke`): used for all request/response interactions from UI to Rust backend (trigger scene, load project, configure schedule, etc.)
- **Events** (`emit` → `listen`): used for all real-time push from Rust to UI (playback state changes, monitor channel data, capture status, errors)
- All command signatures typed end-to-end: Rust return types → TypeScript via `@tauri-apps/api/core`

**Real-Time Monitor Data:**
- Rust backend emits `monitor-update` event at UI refresh rate (~30fps)
- Zustand store subscribes once at app mount; React components read from store
- Playback engine does NOT emit events — a separate monitor thread samples the shared DMX output buffer and emits to UI, keeping the playback loop clean

---

### Frontend Architecture

**State Management: Zustand**
- Single store with slices: `playback`, `capture`, `scenes`, `monitor`, `settings`, `errors`
- Tauri event listeners registered at app mount update Zustand store
- UI components read from store; no direct Tauri IPC calls from components (all IPC wrapped in store actions)

**Component Library: shadcn/ui + Tailwind CSS**
- Components live in `src/components/ui/` — owned by the project, not a runtime dependency
- Dark theme configured in Tailwind config; matches the professional dark aesthetic required by UX spec
- Key components: Button, Dialog, Slider (speed control), Badge (channel status), Card (cue pad cells), Tabs (main navigation)

**Routing:** None — ARTNET-TOOL is a single-view desktop app with tabbed navigation (Cue Pad / Monitor / Settings). No router library needed.

---

### Infrastructure & Deployment

**CI/CD: Tauri GitHub Action (`tauri-apps/tauri-action`)**
- Matrix build: Windows x64, macOS Universal Binary (Intel + Apple Silicon), Linux ARM64 — all triggered on git tag push
- Produces: `.msi` (Windows), `.dmg` (macOS), `.deb` + `.AppImage` (Linux)
- macOS code signing: configured via GitHub Secrets when Apple Developer account is active; CI step present but skipped until then
- Rust compilation cached via `Swatinem/rust-cache` action

**Watchdog / Auto-Start: OS Service Layer (unified mechanism)**
- Linux (RPi 4+): systemd service with `Restart=on-failure`, `RestartSec=10`
- macOS: LaunchAgent plist with `KeepAlive = true`
- Windows: Windows Service via `windows-service` Rust crate (Task Scheduler does not natively restart on crash failure; a Windows Service does)
- Same OS registration serves both auto-start (FR34–FR36) and crash recovery (NFR8–NFR10) — one implementation covers both requirements

**Logging: `tracing` + `tracing-appender` (rolling file)**
- Non-blocking rolling file appender: daily rotation, max 30 files retained (covers 30-day unattended deployment without disk growth)
- Log levels: ERROR / WARN for subsystem failures; INFO for lifecycle events; DEBUG gated behind compile feature flag
- Log viewer in UI reads the current log file via Tauri command (FR39)

---

### Decision Impact Analysis

**Implementation Sequence:**

1. Project scaffold (create-tauri-app, Tokio runtime, shadcn/ui, Zustand)
2. Project file format: JSON schema + serde types + migration framework
3. Packet capture subsystem (pcap crate proof-of-concept — highest risk first)
4. ArtNet/sACN playback engine (isolated thread, spin_sleep timer)
5. Scene store + project management (load/save/export)
6. MIDI subsystem (midir, hot-plug)
7. Scheduler
8. OS service registration (auto-start + watchdog)
9. Differential monitor (sampling thread + Zustand + UI)
10. Logging + log viewer
11. CI/CD pipeline (Tauri GitHub Action)

**Cross-Component Dependencies:**
- Playback engine reads from `Arc<RwLock<ProjectState>>` — depends on scene store
- Monitor samples the playback engine's output buffer — depends on playback engine
- Scheduler sends trigger commands to playback engine via Tokio channel
- All subsystems report errors through a unified error channel → IPC event → UI
- OS service registration depends on installer (Tauri bundler output)

---

## Implementation Patterns & Consistency Rules

### Critical Conflict Points Identified

9 areas where AI agents could make different choices without explicit rules.

---

### Naming Patterns

**Rust (Backend):**
- Modules, functions, variables, fields: `snake_case` (enforced by compiler)
- Types, structs, enums, traits: `PascalCase` (enforced by compiler)
- Constants: `SCREAMING_SNAKE_CASE`
- Tauri command functions: `snake_case` → exposed as the same name on the TypeScript side
  - Example: `fn trigger_scene(scene_id: u32)` → `invoke('trigger_scene', { sceneId: ... })`

**TypeScript/React (Frontend):**
- Variables, functions, hook names: `camelCase`
- React component names and files: `PascalCase` (e.g., `CuePad.tsx`, `DmxMonitor.tsx`)
- Zustand store slices and actions: `camelCase`
- Constants: `SCREAMING_SNAKE_CASE`
- TypeScript interfaces: `I` prefix is FORBIDDEN — use plain `PascalCase`
  - ✅ `type SceneConfig = { ... }`
  - ❌ `interface ISceneConfig { ... }`

**Tauri Event Names:**
- Format: `kebab-case` for all events emitted from Rust to UI
- Pattern: `{subsystem}-{event}`
  - ✅ `playback-state-changed`, `monitor-update`, `capture-started`, `error-occurred`
  - ❌ `playbackStateChanged`, `PLAYBACK_STATE_CHANGED`, `PlaybackStateChanged`

**JSON Project File Fields:**
- All fields: `snake_case` (applied via `#[serde(rename_all = "snake_case")]`)
  - ✅ `"schema_version"`, `"scene_id"`, `"playback_mode"`
  - ❌ `"schemaVersion"`, `"sceneId"`

---

### Structure Patterns

**Frontend Organization (Feature-Based):**
```
src/
  features/
    cue-pad/          ← CuePad tab: components, hooks, types
    monitor/          ← DMX Monitor tab: components, hooks, types
    settings/         ← Settings tab: components, hooks, types
  components/
    ui/               ← shadcn/ui generated components (DO NOT EDIT manually)
    layout/           ← App shell, tabs, window chrome
  stores/             ← Zustand store definitions (one file per slice)
  lib/
    tauri.ts          ← All Tauri invoke() and listen() calls (NEVER call invoke directly from components)
    utils.ts          ← Pure utility functions
  types/              ← Shared TypeScript types used across features
  App.tsx
  main.tsx
```

**Rust Backend Organization:**
```
src-tauri/src/
  main.rs             ← Tauri app entry point; register commands; mount subsystems
  lib.rs              ← Re-exports; public API surface
  commands/           ← Tauri command handlers (thin wrappers; no business logic)
    playback.rs
    capture.rs
    midi.rs
    scheduler.rs
    project.rs
    system.rs
  subsystems/
    playback/         ← Playback engine (isolated thread, spin_sleep)
    capture/          ← Npcap/libpcap packet capture
    midi/             ← MIDI hot-plug and event handling
    scheduler/        ← Time-based scene scheduling
    project/          ← Project file load/save/migrate
    boot/             ← OS auto-start + watchdog registration
    monitor/          ← DMX output sampling thread
  models/             ← Shared data types (ProjectState, Scene, PlaybackMode, etc.)
  errors.rs           ← thiserror error enums per subsystem
  logging.rs          ← tracing + tracing-appender setup
```

**Test File Location:**
- Rust: inline `#[cfg(test)] mod tests { }` at the bottom of each source file
- TypeScript: co-located `ComponentName.test.tsx` alongside `ComponentName.tsx`
- No separate `__tests__/` directories

---

### Format Patterns

**Tauri Command Return Types:**
All Rust command functions return `Result<T, String>` — never raw errors to the frontend:
```rust
#[tauri::command]
fn trigger_scene(scene_id: u32, state: State<AppState>) -> Result<PlaybackStatus, String> {
    state.playback.trigger(scene_id).map_err(|e| e.to_user_message())
}
```
- `.to_user_message()` is a trait method on all domain errors (NFR14: plain language, no stack traces)
- TypeScript receives either the value or throws a `string` error — never a raw Rust error type

**Tauri Event Payloads:**
All event payloads are typed Rust structs deriving `Serialize`:
```rust
#[derive(Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct PlaybackStateChangedPayload {
    pub scene_id: u32,
    pub is_playing: bool,
    pub playback_mode: PlaybackMode,
}
```
Matching TypeScript type defined in `src/types/events.ts` — maintained in sync manually.

**Error Surface to User:**
```typescript
// In lib/tauri.ts — all error handling at the IPC boundary
export async function triggerScene(sceneId: number): Promise<PlaybackStatus> {
  try {
    return await invoke<PlaybackStatus>('trigger_scene', { sceneId });
  } catch (err: unknown) {
    // err is always a string (plain language from Rust .to_user_message())
    useErrorStore.getState().setError(err as string);
    throw err;
  }
}
```

**Date/Time in Project Files:**
- ISO 8601 strings: `"2026-03-12T14:30:00Z"` — never Unix timestamps in JSON

---

### Communication Patterns

**Zustand Store Rules:**
- One file per slice in `src/stores/` (e.g., `playbackStore.ts`, `sceneStore.ts`)
- Slices are composed into a single store in `src/stores/index.ts`
- Components NEVER call `invoke()` directly — all IPC goes through `src/lib/tauri.ts`
- Tauri event listeners registered ONCE in `App.tsx` on mount; they update Zustand store

```typescript
// ✅ Correct — component reads from store
const isPlaying = usePlaybackStore((s) => s.isPlaying);

// ❌ Wrong — component calls Tauri directly
const result = await invoke('get_playback_state');
```

**Rust Inter-Subsystem Communication:**
- Subsystems communicate via Tokio `mpsc` channels — never shared mutable state without `Arc<RwLock<>>`
- Playback engine receives commands only through its dedicated `Sender<PlaybackCommand>` channel
- Scheduler sends `PlaybackCommand::TriggerScene(id)` — it does not touch playback state directly

---

### Process Patterns

**Rust Error Handling:**
- Domain errors (playback, capture, MIDI, scheduler, project): `thiserror` typed enums
- Infrastructure/one-off errors: `anyhow::Result` internally; converted to `String` at command boundary
- Every error variant implements `.to_user_message() -> String` (NFR14)
- NEVER propagate raw `std::io::Error` or `pcap::Error` to the frontend

**Loading State in UI:**
- Each Zustand slice manages its own `isLoading: boolean` field
- No global app-wide loading spinner — per-operation loading state
- Loading set to `true` before `invoke()`, `false` in both success and error paths

**React Component Pattern:**
- Presentational components receive props; they do NOT read from Zustand directly
- Container components (one per feature) read from Zustand and pass props down
- `use` prefix for all custom hooks: `usePlayback`, `useScenes`, `useMidiDevices`

---

### Enforcement Guidelines

**All AI Agents MUST:**
- Place all `invoke()` calls in `src/lib/tauri.ts` — NEVER in components or stores directly
- Use `kebab-case` for Tauri event names; `snake_case` for JSON fields; `camelCase` for TypeScript
- Return `Result<T, String>` from all `#[tauri::command]` functions — never panic or unwrap
- Use `thiserror` for all new domain error types in Rust
- Keep Tauri command handlers thin — business logic belongs in `subsystems/`, not `commands/`
- Co-locate tests: inline `mod tests` in Rust, `.test.tsx` alongside TypeScript files
- Never hardcode subsystem logic in `main.rs`

**Anti-Patterns to Avoid:**
- ❌ `invoke()` called from a React component directly
- ❌ `unwrap()` or `expect()` in Rust subsystem code (only acceptable in test code)
- ❌ Business logic in `commands/*.rs` files
- ❌ Shared mutable state between subsystems without `Arc<RwLock<>>`
- ❌ `camelCase` JSON fields in project files
- ❌ `playbackStateChanged` event names (must be `playback-state-changed`)
- ❌ `ISceneName` TypeScript interface naming

---

## Project Structure & Boundaries

### Complete Project Directory Structure

```
artnet-tool/
├── .github/
│   └── workflows/
│       └── release.yml              ← Tauri GitHub Action matrix build (Win/macOS/Linux)
├── .gitignore
├── package.json                     ← npm workspace root; Vite + Tauri CLI deps
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── tailwind.config.ts
├── components.json                  ← shadcn/ui config
├── index.html
│
├── src/                             ── FRONTEND (React + TypeScript)
│   ├── main.tsx                     ← React entry point
│   ├── App.tsx                      ← App shell; mounts Tauri event listeners; registers Zustand
│   │
│   ├── features/
│   │   ├── cue-pad/                 ← FR7–FR12, FR18–FR21 (scene library, triggers)
│   │   │   ├── CuePad.tsx           ← Container: reads Zustand, passes props
│   │   │   ├── SceneCard.tsx        ← Presentational: single cue pad cell
│   │   │   ├── SceneCard.test.tsx
│   │   │   ├── TriggerBadge.tsx     ← Keyboard / MIDI trigger indicator
│   │   │   ├── PlaybackModeSelect.tsx
│   │   │   ├── SpeedSlider.tsx      ← FR16: real-time speed control
│   │   │   ├── useScenes.ts         ← Hook: scene list, reorder, delete
│   │   │   ├── usePlayback.ts       ← Hook: trigger, stop, playback state
│   │   │   └── types.ts
│   │   │
│   │   ├── monitor/                 ← FR24–FR27 (differential DMX monitor)
│   │   │   ├── DmxMonitor.tsx       ← Container
│   │   │   ├── UniversePanel.tsx    ← One panel per ArtNet universe
│   │   │   ├── ChannelGrid.tsx      ← 512-channel grid with activity state
│   │   │   ├── ChannelCell.tsx      ← Presentational: single channel (active/static/zero)
│   │   │   ├── ChannelCell.test.tsx
│   │   │   ├── useMonitor.ts        ← Hook: subscribe to monitor-update events
│   │   │   └── types.ts
│   │   │
│   │   └── settings/                ← FR3, FR11, FR12, FR15, FR22, FR28–FR37, FR42
│   │       ├── Settings.tsx         ← Container: tabbed settings panel
│   │       ├── NetworkSettings.tsx  ← FR3, FR15: interface selector (capture + output)
│   │       ├── MidiSettings.tsx     ← FR12: device list, note/CC mapping per scene
│   │       ├── ScheduleSettings.tsx ← FR22: time windows, day recurrence per scene
│   │       ├── ProjectSettings.tsx  ← FR28–FR33: load, save, export, import
│   │       ├── BootSettings.tsx     ← FR34–FR36: auto-start toggle, startup scene
│   │       ├── LogViewer.tsx        ← FR39: event/crash log viewer
│   │       ├── UpdateNotice.tsx     ← FR42: update available banner
│   │       ├── useSettings.ts
│   │       └── types.ts
│   │
│   ├── components/
│   │   ├── ui/                      ← shadcn/ui output (Button, Card, Slider, Badge, Dialog, Tabs…)
│   │   │   └── [generated files]    ← DO NOT edit manually; regenerate via shadcn CLI
│   │   ├── layout/
│   │   │   ├── AppShell.tsx         ← Window chrome; main tab bar (Cue Pad / Monitor / Settings)
│   │   │   ├── CaptureBar.tsx       ← Top bar: record button, status, network interface
│   │   │   └── ErrorToast.tsx       ← NFR14: plain-language error display
│   │   └── shared/
│   │       └── StatusIndicator.tsx  ← Reusable active/idle/error dot
│   │
│   ├── stores/
│   │   ├── index.ts                 ← Composes all slices into single Zustand store
│   │   ├── playbackStore.ts         ← isPlaying, currentSceneId, playbackMode, speed
│   │   ├── captureStore.ts          ← isRecording, captureInterface, captureStatus
│   │   ├── sceneStore.ts            ← scenes[], activeSceneId, isLoading
│   │   ├── monitorStore.ts          ← universes{}, channelValues — updated at ~30fps
│   │   ├── settingsStore.ts         ← networkInterfaces, midiDevices, bootConfig
│   │   └── errorStore.ts            ← currentError: string | null
│   │
│   ├── lib/
│   │   ├── tauri.ts                 ← ALL invoke() and listen() calls; error handling at boundary
│   │   └── utils.ts                 ← Pure utility functions (formatDmxValue, etc.)
│   │
│   └── types/
│       ├── events.ts                ← TypeScript types for all Tauri event payloads
│       ├── scene.ts                 ← Scene, PlaybackMode, TriggerConfig, ScheduleConfig
│       └── project.ts               ← ProjectFile, ProjectMetadata
│
└── src-tauri/                       ── BACKEND (Rust)
    ├── Cargo.toml                   ← Workspace deps: tauri, tokio, serde, pcap, midir, thiserror, tracing, tauri-plugin-global-shortcut…
    ├── build.rs
    ├── tauri.conf.json              ← App metadata, bundle config, permissions
    ├── icons/                       ← App icons (all platform sizes)
    │
    └── src/
        ├── main.rs                  ← Entry point; build Tauri app; register all commands
        ├── lib.rs                   ← AppState struct; subsystem handles; re-exports
        ├── errors.rs                ← UserMessage trait; thiserror error enums (all subsystems)
        ├── logging.rs               ← tracing + tracing-appender init; rolling file config
        │
        ├── commands/                ← Thin Tauri command wrappers (no business logic)
        │   ├── playback.rs          ← trigger_scene, stop_playback, set_speed, get_state
        │   ├── capture.rs           ← start_capture, stop_capture, get_capture_status
        │   ├── keyboard.rs          ← register_shortcut, unregister_shortcut (tauri-plugin-global-shortcut)
        │   ├── midi.rs              ← get_midi_devices, assign_midi_trigger
        │   ├── scheduler.rs         ← set_schedule, remove_schedule, get_schedules
        │   ├── project.rs           ← load_project, save_project, export_project, import_project
        │   ├── system.rs            ← enable_autostart, disable_autostart, get_log, check_update
        │   └── network.rs           ← list_interfaces, set_capture_interface, set_output_interface
        │
        ├── subsystems/
        │   ├── playback/            ← FR13–FR17, NFR1, NFR5, NFR6
        │   │   ├── mod.rs           ← PlaybackEngine; spawn_thread(); command channel
        │   │   ├── engine.rs        ← Isolated OS thread; spin_sleep timer; ArtNet/sACN output
        │   │   ├── artnet.rs        ← Art-Net 4 packet serialization; UDP socket send
        │   │   ├── sacn.rs          ← ANSI E1.31 sACN packet serialization; unicast + multicast
        │   │   └── types.rs         ← PlaybackCommand, PlaybackStatus, PlaybackMode
        │   │
        │   ├── capture/             ← FR1–FR6, NFR2, NFR15
        │   │   ├── mod.rs           ← CaptureEngine; start/stop; scene building
        │   │   ├── listener.rs      ← pcap packet capture loop (Tokio task)
        │   │   ├── artnet_parser.rs ← Art-Net 4 / Art-Net 3 packet parsing
        │   │   ├── sacn_parser.rs   ← ANSI E1.31 sACN packet parsing
        │   │   └── types.rs         ← CapturedFrame, CaptureStatus
        │   │
        │   ├── midi/                ← FR12, FR19, FR21, NFR19
        │   │   ├── mod.rs           ← MidiSubsystem; hot-plug detection; trigger dispatch
        │   │   ├── device_manager.rs← midir device enumeration; connect/disconnect events
        │   │   └── types.rs         ← MidiDevice, MidiTrigger, MidiMessage
        │   │
        │   ├── scheduler/           ← FR22–FR23
        │   │   ├── mod.rs           ← SchedulerEngine; Tokio task; time-based dispatch
        │   │   ├── clock.rs         ← Time window evaluation; day-of-week recurrence
        │   │   └── types.rs         ← Schedule, TimeWindow, RecurrenceRule
        │   │
        │   ├── project/             ← FR28–FR33, NFR11, NFR18, NFR21
        │   │   ├── mod.rs           ← ProjectManager; load/save/export/import
        │   │   ├── schema.rs        ← Serde structs (ProjectFile, Scene, TriggerConfig…)
        │   │   ├── migrations.rs    ← migrate_v1_to_v2(), migration dispatch chain
        │   │   └── validator.rs     ← Post-parse integrity checks; actionable error messages
        │   │
        │   ├── boot/                ← FR34–FR37, NFR3, NFR8–NFR10
        │   │   ├── mod.rs           ← BootManager; platform dispatch
        │   │   ├── windows.rs       ← Windows Service registration (windows-service crate)
        │   │   ├── macos.rs         ← LaunchAgent plist write/remove
        │   │   └── linux.rs         ← systemd user service unit write/enable
        │   │
        │   └── monitor/             ← FR24–FR27
        │       ├── mod.rs           ← MonitorSampler; samples playback output buffer
        │       └── emitter.rs       ← Tokio task; emits monitor-update events at ~30fps
        │
        └── models/                  ← Shared data types across subsystems
            ├── scene.rs             ← Scene, DmxFrame, SceneMetadata
            ├── project.rs           ← ProjectState (Arc<RwLock<>> root)
            └── dmx.rs               ← Universe, ChannelValue, DmxBuffer
```

---

### Architectural Boundaries

**IPC Boundary (Rust ↔ React):**
- All crossing: via Tauri commands (`invoke`) and events (`emit`/`listen`)
- Commands: defined in `src-tauri/src/commands/`; consumed from `src/lib/tauri.ts` only
- Events: emitted by subsystems; subscribed in `App.tsx`; update Zustand store
- No React component ever crosses this boundary directly

**Playback Engine Isolation Boundary (NFR5):**
- The playback engine runs on a dedicated OS thread (`std::thread::spawn`)
- It communicates with the rest of the Rust backend ONLY via `mpsc::Sender<PlaybackCommand>`
- It writes DMX output to a shared `Arc<Mutex<DmxBuffer>>` — read by the monitor sampler
- Nothing in the Tauri async runtime can block or starve the playback thread

**Project State Boundary:**
- `Arc<RwLock<ProjectState>>` is the single source of truth for scene data
- Writers: `project/` subsystem (load) and `capture/` subsystem (record)
- Readers: `playback/`, `scheduler/`, Tauri commands (for UI queries)
- Concurrent reads are non-blocking; writes are short-duration (load/save only)

---

### FR Category → File Mapping

| FR Category | FRs | Primary Files |
|---|---|---|
| Signal Capture | FR1–FR6 | `subsystems/capture/`, `commands/capture.rs`, `commands/network.rs` |
| Scene Management | FR7–FR10 | `subsystems/project/schema.rs`, `models/scene.rs`, `features/cue-pad/` |
| Playback Engine | FR13–FR17 | `subsystems/playback/engine.rs`, `subsystems/playback/artnet.rs` |
| Triggering & Control | FR11–FR12, FR18–FR21 | `subsystems/midi/`, `commands/midi.rs`, `features/cue-pad/TriggerBadge.tsx` |
| Scheduling | FR22–FR23 | `subsystems/scheduler/`, `commands/scheduler.rs`, `features/settings/ScheduleSettings.tsx` |
| Signal Monitoring | FR24–FR27 | `subsystems/monitor/`, `features/monitor/` |
| Project Management | FR28–FR33 | `subsystems/project/`, `commands/project.rs`, `features/settings/ProjectSettings.tsx` |
| Boot & Auto-Start | FR34–FR37 | `subsystems/boot/`, `commands/system.rs`, `features/settings/BootSettings.tsx` |
| Diagnostics | FR38–FR42 | `logging.rs`, `commands/system.rs`, `features/settings/LogViewer.tsx` |

---

### Data Flow

```
[Source Software] → ArtNet/sACN UDP →
  capture/listener.rs (pcap) →
  capture/artnet_parser.rs →
  models/scene.rs (DmxFrame[]) →
  project/schema.rs (Scene) →
  Arc<RwLock<ProjectState>>

[Trigger: keyboard / MIDI / UI / scheduler] →
  commands/playback.rs →
  subsystems/playback/engine.rs (isolated thread) →
  Arc<Mutex<DmxBuffer>> (output buffer) →
  playback/artnet.rs → UDP socket → [Lighting fixtures]

[Monitor path — parallel, non-blocking]:
  monitor/emitter.rs (samples DmxBuffer ~30fps) →
  Tauri emit("monitor-update") →
  App.tsx listener →
  Zustand monitorStore →
  features/monitor/ChannelGrid.tsx
```

---

### Development Workflow

**Local dev:** `npm run tauri dev` — Vite HMR for frontend; Cargo watch for backend
**Tests:** `cargo test` (Rust inline tests) + `npx vitest` (TypeScript co-located tests)
**Release:** Push git tag → GitHub Action triggers matrix → artifacts uploaded to GitHub Release

---

## Architecture Validation Results

### Coherence Validation ✅

**Decision Compatibility:**
All stack choices are mutually compatible. Tauri 2 uses Tokio internally — our Tokio runtime has zero conflict with the framework. `spin_sleep` on a dedicated `std::thread` (outside Tokio) is the correct isolation pattern for ±1ms timing with no contention from the async runtime. `midir` supports Tokio integration for MIDI hot-plug. `pcap` wraps Npcap/libpcap correctly on all three platforms. `serde` + `deny_unknown_fields` + `thiserror` + `tracing` are the canonical Rust production stack — fully compatible.

**Pattern Consistency:**
Naming conventions (kebab-case events, snake_case JSON, camelCase TypeScript) are mutually exclusive and non-overlapping. The IPC boundary rule (all `invoke()` in `lib/tauri.ts`) and the playback thread isolation pattern both flow naturally from the technology choices. No contradictions found.

**Structure Alignment:**
The feature-based frontend organization matches the tab-based UX spec. The `commands/` vs `subsystems/` split enforces the thin-handler pattern. The `models/` shared types layer prevents circular dependencies between subsystems.

---

### Requirements Coverage Validation ✅

**Functional Requirements (42 FRs):**
All 42 FRs are mapped to specific files in the project structure. No FR is without a designated implementation location.

**Non-Functional Requirements (22 NFRs):**

| NFR | Coverage | File/Mechanism |
|---|---|---|
| NFR1 ±1ms timing | ✅ | `engine.rs` isolated thread + `spin_sleep` |
| NFR2 Capture timing | ✅ | `pcap` timestamps preserved in `CapturedFrame` |
| NFR3 30s boot | ✅ | Tauri <0.5s startup + OS service auto-start |
| NFR4 <15% CPU / <300MB | ✅ | Tauri (30–50MB idle), Rust (no GC) |
| NFR5 UI isolation | ✅ | Separate `std::thread` for playback engine |
| NFR6 50ms trigger | ✅ | `mpsc` channel send to playback thread (sub-ms) |
| NFR7 30-day unattended | ✅ | Rust no GC leaks + 30-file log rotation |
| NFR8/9 60s crash recovery | ✅ | OS service `Restart=on-failure` / `KeepAlive` |
| NFR10 Boot recovery | ✅ | OS service + auto-load last project |
| NFR11 File integrity | ✅ | `validator.rs` + serde parse errors |
| NFR14 Error clarity | ✅ | `UserMessage` trait → `String` → `ErrorToast.tsx` |
| NFR15/16 ArtNet/sACN spec | ✅ | `artnet.rs`, `sacn.rs`, `artnet_parser.rs`, `sacn_parser.rs` |
| NFR17 Cross-platform parity | ✅ | Platform adapters in `subsystems/boot/` |
| NFR18 Project portability | ✅ | JSON snake_case; no platform-specific paths in file |
| NFR19 MIDI 1.0 | ✅ | `midir` crate, cross-platform |
| NFR20 No telemetry | ✅ | No outbound connections in architecture |
| NFR21 File safety | ✅ | `serde` + `deny_unknown_fields`; no `eval` |
| NFR22 No open ports | ✅ | Only ArtNet UDP 6454 + sACN UDP 5568 |

---

### Gap Analysis & Resolutions

**Gap 1 — Keyboard shortcut subsystem (FR11, FR18): RESOLVED ✅**
- Issue: Architecture defined keyboard triggers but did not specify the capture mechanism
- Resolution: `tauri-plugin-global-shortcut` added to `Cargo.toml`; `commands/keyboard.rs` added to project structure
- Rationale: OS-level global shortcuts work even when app window is minimized or not focused — required for deployment scenarios where ARTNET-TOOL runs in the background
- Keyboard shortcut events dispatch `PlaybackCommand::TriggerScene(id)` through the same channel as MIDI and scheduler triggers — unified trigger path

**Gap 2 — Windows installer: Npcap bundling + firewall rule injection (FR37): RESOLVED ✅**
- Issue: FR37 requires installer to auto-configure Npcap and firewall rules; this was not documented
- Resolution: Tauri's NSIS/MSI bundler supports `externalBin` and custom installer scripts; Npcap silent installer (`npcap-oem.exe /S`) bundled as an installer prerequisite; Windows Defender Firewall rule for ArtNet UDP 6454 and sACN UDP 5568 injected via `netsh advfirewall` in the NSIS post-install script
- Note: Requires elevated installer (UAC prompt on first install only)

---

### Architecture Completeness Checklist

**Requirements Analysis**
- [x] Project context thoroughly analyzed (42 FRs, 22 NFRs, 8 domains)
- [x] Scale and complexity assessed (Medium; 9–11 subsystems)
- [x] Technical constraints identified (platform-specific, runtime, integration)
- [x] Cross-cutting concerns mapped (7 identified)

**Architectural Decisions**
- [x] Critical decisions documented (threading model, watchdog, project file format)
- [x] Technology stack fully specified (Tauri 2 + Rust + React 19 + Zustand + shadcn/ui)
- [x] Integration patterns defined (Tokio channels, Arc<RwLock<>>, IPC commands/events)
- [x] Performance considerations addressed (all timing-critical NFRs covered)
- [x] Security requirements covered (NFR20/21/22)
- [x] Deferred decisions documented (macOS notarization)

**Implementation Patterns**
- [x] Naming conventions established (Rust/TypeScript/events/JSON)
- [x] Structure patterns defined (feature-based frontend, commands/subsystems Rust)
- [x] Communication patterns specified (Zustand rules, inter-subsystem channels)
- [x] Process patterns documented (error handling, loading state, component pattern)
- [x] Anti-patterns documented (9 explicit anti-patterns)

**Project Structure**
- [x] Complete directory structure defined (all files named and annotated)
- [x] Component boundaries established (IPC, playback isolation, project state)
- [x] Integration points mapped (FR → file mapping table)
- [x] Data flow documented (capture → store → playback → monitor paths)

---

### Architecture Readiness Assessment

**Overall Status: READY FOR IMPLEMENTATION** ✅

**Confidence Level: High**

**Key Strengths:**
- Real-time constraints (NFR1) are architecturally enforced, not left to implementation judgment
- Rust's compile-time thread safety guarantees the isolation boundaries cannot be accidentally violated
- Every FR maps to a specific file — no ambiguity about where implementation belongs
- OS service layer elegantly solves both auto-start and crash recovery in one mechanism
- Keyboard + MIDI + scheduler triggers share a single `PlaybackCommand` channel — unified, testable trigger path

**Areas for Future Enhancement (Post-MVP):**
- Web Remote (Phase 2): will add a new subsystem `subsystems/web-remote/` and Zustand slice; IPC boundary stays the same
- Scene Layering Engine (Phase 3): extends `models/scene.rs` and `subsystems/playback/engine.rs`; no architectural restructuring needed
- MessagePack project file compression: drop-in addition to `subsystems/project/`

---

### Implementation Handoff

**AI Agent Guidelines:**
- Follow all architectural decisions exactly as documented — no local deviations
- Use the FR → File Mapping table to determine where each requirement is implemented
- Respect the playback engine isolation boundary — the engine thread must never be touched from async Tokio code
- All `invoke()` calls live in `src/lib/tauri.ts` — non-negotiable
- Refer to the Enforcement Guidelines and Anti-Patterns sections before writing any cross-subsystem code

**First Implementation Step:**
```bash
npm create tauri-app@latest artnet-tool -- --template react-ts
```
Then: install `tauri-plugin-global-shortcut`, configure Tokio runtime, add `shadcn/ui`, scaffold Zustand store slices.
