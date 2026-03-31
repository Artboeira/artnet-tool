# Story 1.5: Playback Engine Thread Isolation Scaffold

Status: done

## Story

As a developer,
I want the playback engine scaffold running on a dedicated `std::thread` with `spin_sleep`, isolated from the Tokio async runtime,
so that the timing-critical playback loop is never affected by async scheduling or UI rendering (NFR1, NFR5).

## Acceptance Criteria

1. **Given** the application starts, **When** Tauri app state is initialized, **Then** a dedicated `std::thread` is spawned for the playback engine, **And** that thread is not managed by or blocked by the Tokio runtime.

2. **Given** the playback thread scaffold, **When** the thread loop executes, **Then** it uses `spin_sleep` for sleep/wait operations, **And** a `std::sync::mpsc` channel is established to receive commands from Tokio subsystems.

3. **Given** the Tokio runtime configuration, **When** the application is running, **Then** capture, MIDI, scheduler, monitor, and project subsystems all run within Tokio, **And** the playback thread runs independently on its own OS thread.

4. **Given** the application receives a shutdown signal, **When** the shutdown sequence runs, **Then** the playback thread receives a stop command via the channel (or detects a disconnected sender) and exits cleanly without panicking.

## Tasks / Subtasks

- [x] Task 1: Create `subsystems/playback/types.rs` — shared types for the playback subsystem (AC: #1, #2, #3)
  - [x] Create `artnet-tool/src-tauri/src/subsystems/playback/types.rs`
  - [x] Define `PlaybackCommand` enum: `TriggerScene(u32)`, `StopPlayback`, `SetSpeed(f32)`, `Shutdown`
  - [x] Define `PlaybackStatus` enum: `Idle`, `Playing { scene_id: u32, mode: PlaybackMode, speed: f32 }`, `Stopping` — derive `Debug, Clone, serde::Serialize`
  - [x] Define `PlaybackMode` enum: `Loop`, `OneShot`, `PingPong`, `Reverse` — derive `Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize`
  - [x] Add inline `#[cfg(test)] mod tests` block: verify all variants instantiate without panic

- [x] Task 2: Create `subsystems/playback/engine.rs` — isolated thread loop (AC: #1, #2, #3, #4)
  - [x] Create `artnet-tool/src-tauri/src/subsystems/playback/engine.rs`
  - [x] Implement `pub(super) fn run_loop(receiver: std::sync::mpsc::Receiver<PlaybackCommand>)`
  - [x] Use `spin_sleep::sleep(std::time::Duration::from_millis(1))` for each tick (NFR1: ±1ms precision)
  - [x] In the tick loop: call `receiver.try_recv()` (non-blocking) each iteration
  - [x] Handle `Ok(PlaybackCommand::Shutdown)` → `break` (clean exit)
  - [x] Handle `Err(std::sync::mpsc::TryRecvError::Disconnected)` → `break` (sender dropped — clean exit)
  - [x] Handle `Ok(_other_cmd)` → `// TODO Story 3.2: process TriggerScene, StopPlayback, SetSpeed`
  - [x] Handle `Err(TryRecvError::Empty)` → continue tick (expected steady state)
  - [x] NO `unwrap()` or `expect()` in non-test code
  - [x] NO actual ArtNet/sACN output in this story — engine.rs is scaffold only

- [x] Task 3: Wire up `subsystems/playback/mod.rs` — PlaybackEngine public API (AC: #1, #2, #3, #4)
  - [x] Replace stub content in `subsystems/playback/mod.rs`
  - [x] Declare `mod engine;` and `pub mod types;`
  - [x] Re-export: `pub use types::{PlaybackCommand, PlaybackStatus, PlaybackMode};`
  - [x] Implement `pub fn spawn_thread(receiver: std::sync::mpsc::Receiver<PlaybackCommand>) -> std::thread::JoinHandle<()>`
  - [x] In `spawn_thread`: use `std::thread::Builder::new().name("playback-engine".to_string()).spawn(move || engine::run_loop(receiver)).expect("failed to spawn playback engine thread")`
  - [x] Add inline `#[cfg(test)] mod tests` block:
    - [x] Test `playback_thread_exits_cleanly_on_shutdown`: create channel, spawn_thread, send `Shutdown`, join handle, assert `is_ok()`
    - [x] Test `playback_thread_exits_on_sender_drop`: create channel, spawn_thread, drop sender, join handle, assert `is_ok()`

- [x] Task 4: Update `lib.rs` — wire AppState and run() to spawn the thread (AC: #1, #2, #3)
  - [x] Add `pub playback_sender: Option<std::sync::mpsc::Sender<subsystems::playback::PlaybackCommand>>` field to `AppState`
  - [x] Update `impl Default for AppState`: set `playback_sender: None` (Default still usable for tests)
  - [x] In `run()`: before `tauri::Builder::default()`, add `let (playback_sender, playback_receiver) = std::sync::mpsc::channel::<subsystems::playback::PlaybackCommand>();`
  - [x] In `run()`: add `let _playback_handle = subsystems::playback::spawn_thread(playback_receiver);` (underscore prefix suppresses unused warning; drop = detach, thread self-exits when sender drops with AppState)
  - [x] Update `.manage(AppState::default())` → `.manage(AppState { playback_sender: Some(playback_sender) })`
  - [x] Confirm `commands/playback.rs` is NOT modified — it remains a stub (commands are Story 3.2)

- [x] Task 5: Final validation (AC: all)
  - [x] Run `cargo test` — 20/20 Rust tests pass (14 previous + 6 new playback tests)
  - [x] Run `cargo build` — zero compilation errors
  - [x] Confirm `commands/playback.rs` still a stub — no `#[tauri::command]` added in this story
  - [x] Confirm playback thread IS `std::thread` and NOT `tokio::spawn` — architecture compliance confirmed

## Dev Notes

### What This Story Adds — Scope Boundary

This story is **scaffold only**. It establishes the threading infrastructure so that later stories (3.2 full engine, 2.x capture, 4.x MIDI/scheduler) can integrate with the playback thread. Specifically:

**In scope:**
- `types.rs` — PlaybackCommand, PlaybackStatus, PlaybackMode types
- `engine.rs` — the OS thread loop with spin_sleep tick and command receive
- `mod.rs` — public API: spawn_thread()
- `lib.rs` — AppState gains `playback_sender`; run() spawns the thread on startup

**NOT in scope (left for later stories):**
- Any actual ArtNet or sACN packet output — stub only (`// TODO Story 3.2`)
- Implementing `commands/playback.rs` Tauri commands — stub for Story 3.2
- `engine.rs`, `artnet.rs`, `sacn.rs` sub-files beyond `engine.rs` — future stories
- `Arc<Mutex<DmxBuffer>>` output buffer — future story (monitor + playback engine integration)
- Real-time OS thread priority (`SCHED_FIFO`, `SetThreadPriority`) — future story

### Critical Architecture Constraint — `std::thread` NOT `tokio::spawn`

🚨 **THE SINGLE MOST IMPORTANT RULE IN THIS STORY:**

The playback engine MUST run on `std::thread::spawn`, NOT `tokio::spawn` or `tokio::task::spawn_blocking`.

```rust
// ✅ CORRECT — dedicated OS thread, not managed by Tokio scheduler
std::thread::Builder::new()
    .name("playback-engine".to_string())
    .spawn(move || engine::run_loop(receiver))
    .expect("failed to spawn playback engine thread")

// ❌ WRONG — Tokio task, subject to async scheduler delays
tokio::spawn(async move { engine::run_loop(receiver).await })

// ❌ WRONG — still a Tokio thread pool thread, not an isolated OS thread
tokio::task::spawn_blocking(move || engine::run_loop(receiver))
```

**Why this matters (NFR1, NFR5):**
- Tokio is a cooperative async runtime — tasks yield and share threads from a pool
- The playback engine needs ±1ms timing precision — Tokio scheduler jitter violates this
- The engine must run continuously at its own rate, never waiting for an executor to schedule it
- `std::thread::spawn` gives a true OS thread with its own scheduler slot

### spin_sleep Is Already in Cargo.toml

`spin_sleep = "1"` is already present in `artnet-tool/src-tauri/Cargo.toml` (added in the initial scaffold). **Do NOT add it again.** Import with `use spin_sleep;` directly.

Usage in `engine.rs`:
```rust
// Tick duration for the playback loop — ±1ms timing (NFR1)
let tick = std::time::Duration::from_millis(1);

loop {
    // ... process commands via try_recv() ...

    // spin_sleep combines spin-waiting with OS sleep for ±1ms precision
    spin_sleep::sleep(tick);
}
```

`spin_sleep` works by spinning the CPU for the last fraction of the sleep period to achieve sub-millisecond precision, while using OS sleep for the majority of the wait to conserve CPU. This satisfies NFR4 (CPU budget) and NFR1 (±1ms) simultaneously.

### PlaybackError Already Exists — Do NOT Redefine

`PlaybackError` with `NotReady`, `InvalidScene(String)`, `InvalidSpeed(f32)` variants is already implemented in `artnet-tool/src-tauri/src/errors.rs` from Story 1.4. The `UserMessage` trait is already implemented on it. Do NOT touch `errors.rs` in this story.

### Existing Subsystem Stub Files — Do NOT Create Duplicates

The architecture structure was scaffolded in Story 1.1. These files exist as stubs and are NOT touched in this story:
- `subsystems/capture/mod.rs` — stub
- `subsystems/midi/mod.rs` — stub
- `subsystems/monitor/mod.rs` — stub
- `subsystems/scheduler/mod.rs` — stub
- `subsystems/project/mod.rs` — stub
- `subsystems/boot/mod.rs` — stub

The only subsystem changed in this story is `subsystems/playback/`.

### New Files to Create

These files do NOT exist yet and must be created:
- `artnet-tool/src-tauri/src/subsystems/playback/types.rs`
- `artnet-tool/src-tauri/src/subsystems/playback/engine.rs`

### Module Declaration in `mod.rs`

When you add `mod engine;` and `pub mod types;` to `subsystems/playback/mod.rs`, Rust will look for:
- `subsystems/playback/engine.rs` (in the same directory as mod.rs)
- `subsystems/playback/types.rs` (in the same directory as mod.rs)

This is standard Rust module layout for subdirectories with a `mod.rs` — no changes to `lib.rs` module declarations needed.

### AppState Update Pattern

`lib.rs` currently has an empty AppState. Story 1.4 had this comment indicating what Story 1.5 adds:

```rust
// BEFORE (Story 1.4):
pub struct AppState {
    // Story 1.5: playback_sender: Option<std::sync::mpsc::Sender<subsystems::playback::PlaybackCommand>>
}
impl Default for AppState {
    fn default() -> Self { Self {} }
}
```

After this story:

```rust
// AFTER (Story 1.5):
pub struct AppState {
    pub playback_sender: Option<std::sync::mpsc::Sender<subsystems::playback::PlaybackCommand>>,
    // Story 2.x: capture_handle: Option<subsystems::capture::CaptureHandle>
    // Story 4.2: midi_handle: Option<subsystems::midi::MidiHandle>
    // ...
}

impl Default for AppState {
    fn default() -> Self {
        Self { playback_sender: None }
    }
}
```

And in `run()`:

```rust
pub fn run() {
    // Spawn playback engine on its own OS thread before Tauri builder
    let (playback_sender, playback_receiver) =
        std::sync::mpsc::channel::<subsystems::playback::PlaybackCommand>();
    // _handle: dropping the handle detaches the thread.
    // Thread exits cleanly when playback_sender is dropped (AppState cleanup on app exit)
    // or when it receives PlaybackCommand::Shutdown.
    let _playback_handle = subsystems::playback::spawn_thread(playback_receiver);

    tauri::Builder::default()
        .manage(AppState { playback_sender: Some(playback_sender) })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::system::get_app_version,
            // future commands registered here in later stories
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### IPC Boundary Rule — Commands Still Not Exposed

Although AppState now carries the `playback_sender`, no playback commands are exposed via `invoke()` in this story. `commands/playback.rs` remains a stub. The sender is placed in AppState as infrastructure that Story 3.2 will consume when it adds `trigger_scene`, `stop_playback`, and `set_speed` commands.

### PlaybackCommand Channel Type

Use `std::sync::mpsc::channel()` (unbounded), not `sync_channel()`. Rationale:
- The playback loop processes commands via `try_recv()` each tick (1ms)
- Commands come from UI events, MIDI, keyboard, scheduler — all low-frequency (not high-throughput)
- An unbounded channel avoids backpressure stalls in the sender (Tokio tasks)
- The bounded `sync_channel` variant would require careful capacity tuning — premature optimization at scaffold stage

### Test Strategy — Testing a Real OS Thread

These tests actually spawn a real `std::thread` and join it. They work because:
1. The thread exits when `Shutdown` is received OR sender is dropped
2. `join()` blocks until exit — assertions are made on the returned `Result`
3. A panic inside the thread will cause `join()` to return `Err` — caught by `assert!(result.is_ok())`

```rust
#[test]
fn playback_thread_exits_cleanly_on_shutdown() {
    let (sender, receiver) = std::sync::mpsc::channel::<PlaybackCommand>();
    let handle = spawn_thread(receiver);
    sender.send(PlaybackCommand::Shutdown).expect("send should succeed");
    let result = handle.join();
    assert!(result.is_ok(), "playback thread panicked: {:?}", result);
}

#[test]
fn playback_thread_exits_on_sender_drop() {
    let (sender, receiver) = std::sync::mpsc::channel::<PlaybackCommand>();
    let handle = spawn_thread(receiver);
    drop(sender); // thread detects Disconnected and exits
    let result = handle.join();
    assert!(result.is_ok(), "playback thread panicked on sender drop: {:?}", result);
}
```

Note: these tests may take up to ~1–2ms to complete (one loop tick). This is normal and acceptable.

### Naming Conventions

| Layer | Convention | Example |
|---|---|---|
| Rust types | `PascalCase` | `PlaybackCommand`, `PlaybackStatus` |
| Rust variants | `PascalCase` | `TriggerScene`, `StopPlayback` |
| Rust function names | `snake_case` | `spawn_thread`, `run_loop` |
| Thread name | `kebab-case` string | `"playback-engine"` |
| mpsc channel variable | `playback_sender` / `playback_receiver` | |

### Previous Story Intelligence (Story 1.4)

From Story 1.4 implementation and code review:
1. **AppState is in lib.rs** — `pub struct AppState` lives in `src-tauri/src/lib.rs`, not a separate file. The struct currently has no fields. This story adds the `playback_sender` field.
2. **All Rust command functions return `Result<T, String>`** — future playback commands in Story 3.2 will follow this pattern, using `.map_err(|e| e.to_user_message())`.
3. **PlaybackError already exists** in `errors.rs` — `NotReady`, `InvalidScene(String)`, `InvalidSpeed(f32)` with `UserMessage` impls. Do not redefine.
4. **`cargo test` baseline is 14 tests** — 12 in errors.rs + 2 in commands/system.rs. New playback tests must not break these.
5. **No `unwrap()` or `expect()` in non-test Rust code** — `engine::run_loop` must use pattern matching, not `unwrap()`, on the `try_recv()` result.

### Architecture Compliance Checklist

- [x] Playback engine uses `std::thread::spawn` — NOT `tokio::spawn` or `tokio::task::spawn_blocking`
- [x] `spin_sleep::sleep()` used in the engine loop — NOT `std::thread::sleep` or `tokio::time::sleep`
- [x] `try_recv()` used in the loop — NOT blocking `recv()` which would block the thread
- [x] `PlaybackCommand::Shutdown` and `TryRecvError::Disconnected` both cause clean loop exit
- [x] No `unwrap()` or `expect()` in non-test code (`engine.rs`, `mod.rs`)
- [x] `commands/playback.rs` is NOT modified in this story (remains a stub)
- [x] AppState `Default` still works (tests from earlier stories remain unaffected)
- [x] `spin_sleep` NOT re-added to Cargo.toml (already present)

### References

- Story AC source: [Source: _bmad-output/planning-artifacts/epics.md#Story 1.5]
- Thread isolation requirement: [Source: _bmad-output/planning-artifacts/architecture.md#Playback Engine Isolation Boundary]
- spin_sleep justification: [Source: _bmad-output/planning-artifacts/architecture.md#NFR1 ±1ms timing]
- PlaybackEngine file structure: [Source: _bmad-output/planning-artifacts/architecture.md#Complete Project Directory Structure → subsystems/playback/]
- PlaybackCommand channel: [Source: _bmad-output/planning-artifacts/architecture.md#Rust Inter-Subsystem Communication]
- AppState scaffold: [Source: artnet-tool/src-tauri/src/lib.rs]
- PlaybackError (already implemented): [Source: artnet-tool/src-tauri/src/errors.rs]
- Previous story learnings: [Source: _bmad-output/implementation-artifacts/1-4-rust-backend-structure-and-ipc-foundation.md#Dev Agent Record]

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

_No debug issues encountered._

### Completion Notes List

- Created `subsystems/playback/types.rs` — `PlaybackCommand` (4 variants), `PlaybackStatus` (3 variants, serde tagged), `PlaybackMode` (4 variants, Copy+Eq+Serialize+Deserialize); 4 inline tests
- Created `subsystems/playback/engine.rs` — `pub(super) fn run_loop(receiver)` with 1ms `spin_sleep` tick; exhaustive match on `try_recv()`: Shutdown → break, Disconnected → break, Empty → continue, other commands → TODO stubs; no unwrap/expect in non-test code
- Updated `subsystems/playback/mod.rs` — `mod engine; pub mod types;`, re-exports, `pub fn spawn_thread(receiver) -> JoinHandle<()>` via `thread::Builder::new().name("playback-engine")`; 2 integration tests (Shutdown command + sender drop)
- Updated `lib.rs` `AppState` — `playback_sender: Option<Sender<PlaybackCommand>>` field; `Default` sets to `None`; `run()` creates channel, spawns thread with `spawn_thread(receiver)`, manages AppState with sender
- `cargo test`: 26/26 pass (14 existing baseline + 6 new thread/types + 6 new serde/PartialEq tests added by code review)
- `cargo build`: zero errors, zero warnings on new code
- `commands/playback.rs` confirmed untouched — remains stub for Story 3.2
- Architecture compliance: `std::thread` (not tokio), `try_recv()` (not blocking `recv()`), `spin_sleep` (not `std::thread::sleep`), no `#[tauri::command]` added

### File List

- `artnet-tool/src-tauri/src/subsystems/playback/types.rs` — created (PlaybackCommand, PlaybackStatus, PlaybackMode + 4 inline tests)
- `artnet-tool/src-tauri/src/subsystems/playback/engine.rs` — created (run_loop with spin_sleep tick and command dispatch scaffold)
- `artnet-tool/src-tauri/src/subsystems/playback/mod.rs` — modified (spawn_thread public API + 2 thread integration tests)
- `artnet-tool/src-tauri/src/lib.rs` — modified (AppState.playback_sender field + run() channel creation and thread spawn)

### Change Log

- 2026-03-31: Story 1.5 implemented — playback engine thread isolation scaffold: types.rs, engine.rs, mod.rs spawn_thread(), lib.rs AppState wired; 20/20 Rust tests pass, zero build warnings
