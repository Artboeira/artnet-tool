# Story 1.4: Rust Backend Structure & IPC Foundation

Status: done

## Story

As a developer,
I want the Rust backend organized into `commands/`, `subsystems/`, and `models/` with the `thiserror`/`anyhow` error pattern and a `to_user_message()` trait converting all errors to plain strings at the IPC boundary,
so that all Rust features follow a consistent error handling and IPC contract.

## Acceptance Criteria

1. **Given** the Rust backend, **When** the directory layout is reviewed, **Then** `commands/`, `subsystems/`, and `models/` directories exist with placeholder modules registered in `lib.rs`.

2. **Given** a Tauri command handler that returns an `Err`, **When** the frontend calls it via `invoke()` in `src/lib/tauri.ts`, **Then** the error arrives as a plain-language `String`, **And** no raw Rust error codes, type names, or stack trace fragments are visible.

3. **Given** a domain error type using `thiserror`, **When** the `to_user_message()` trait is implemented on it, **Then** it returns a human-readable string describing the problem and next action (NFR14).

4. **Given** `src/lib/tauri.ts`, **When** it is reviewed, **Then** it is the only file in the frontend codebase that calls `invoke()`, **And** every exported function has explicit TypeScript parameter and return types matching the corresponding Rust command signature.

## Tasks / Subtasks

- [x] Task 1: Expand `errors.rs` — per-subsystem error enums with `thiserror` (AC: #2, #3)
  - [x] Add `use thiserror::Error;` import at top of `errors.rs`
  - [x] Define `PlaybackError` enum with at least `NotReady` and `InvalidScene(String)` variants
  - [x] Define `CaptureError` enum with at least `InterfaceNotFound(String)` and `AlreadyRunning` variants
  - [x] Define `MidiError` enum with at least `DeviceNotFound(String)` variant
  - [x] Define `SchedulerError` enum with at least `InvalidSchedule(String)` variant
  - [x] Define `ProjectError` enum with at least `LoadFailed(String)` and `SaveFailed(String)` variants
  - [x] Define `BootError` enum with at least `PlatformNotSupported` variant
  - [x] Implement `UserMessage` trait on each enum (human-readable strings, no type names)
  - [x] Add inline `#[cfg(test)] mod tests` block validating `to_user_message()` output format
  - [x] Run `cargo test` — all Rust tests pass

- [x] Task 2: Add `AppState` struct to `lib.rs` (AC: #1)
  - [x] Define `AppState` struct (empty handles for now, to be filled by future stories)
  - [x] Add `impl Default for AppState` (manual impl — no fields to derive)
  - [x] Register `AppState` as managed state in `tauri::Builder` via `.manage(AppState::default())`

- [x] Task 3: Implement `get_app_version` in `commands/system.rs` (AC: #2, #3)
  - [x] Add `#[tauri::command]` on `pub fn get_app_version() -> Result<String, String>`
  - [x] Return the app version string from `env!("CARGO_PKG_VERSION")` as `Ok(...)`
  - [x] Add inline test: call `get_app_version()`, verify it returns `Ok` and value is non-empty

- [x] Task 4: Register command in `lib.rs` invoke_handler (AC: #1, #2)
  - [x] Uncomment `invoke_handler` in `tauri::Builder`
  - [x] Register `commands::system::get_app_version` in `tauri::generate_handler![]`
  - [x] Run `cargo build` — zero compilation errors

- [x] Task 5: Update `src/lib/tauri.ts` — first real IPC wrapper (AC: #4)
  - [x] Add `import { useErrorStore } from '@/stores';` at the top
  - [x] Add `export type AppVersion = string;` typed alias
  - [x] Implement `export async function getAppVersion(): Promise<AppVersion>` wrapper using the error pattern from architecture.md
  - [x] Wire `useErrorStore.getState().setError(err as string)` in the catch block
  - [x] Remove `_IPC_BOUNDARY` placeholder export
  - [x] Remove `void invoke;` suppression (invoke is now actively used)
  - [x] Run `npx tsc --noEmit` — zero TypeScript errors

- [x] Task 6: Final validation (AC: all)
  - [x] Run `cargo test` — 13 Rust tests pass (11 error enum tests + 2 system command tests)
  - [x] Run `cargo build` — application compiles cleanly
  - [x] Run `npx tsc --noEmit` — zero TypeScript errors
  - [x] Run `npm test` — 7/7 frontend tests pass (AppShell.test.tsx, errorStore.test.ts)
  - [x] Confirm `src/lib/tauri.ts` is the ONLY file with `invoke()` — grep confirmed (no other source file contains `invoke`)

## Dev Notes

### Current Backend State (as of Story 1.3)

The full directory scaffold already exists — `commands/`, `subsystems/`, `models/` directories and placeholder `.rs` files were all created in Story 1.1. **Do NOT recreate or reorganize these directories.** What this story adds is:
1. Real content in `errors.rs` (error enums + trait implementations)
2. `AppState` struct in `lib.rs` and an active `invoke_handler`
3. One concrete command (`get_app_version`) as the IPC pattern demonstrator
4. First real typed wrapper in `tauri.ts`

**Files currently present and in scope:**
```
src-tauri/src/
  lib.rs          ← Has module declarations; invoke_handler commented out
  errors.rs       ← Has UserMessage trait only; NO error enums yet
  commands/
    system.rs     ← Stub comment only; needs get_app_version implemented
  (all other commands/ and subsystems/ files are stubs — do NOT touch them)
```

**Files NOT to touch in this story:**
- `commands/playback.rs`, `capture.rs`, `keyboard.rs`, `midi.rs`, `scheduler.rs`, `project.rs`, `network.rs` — all stubs for their own stories
- All `subsystems/*/mod.rs` files — stubs for future epics
- All `models/*.rs` files — stubs for their feature epics
- `logging.rs` — populated in Story 7.1
- `main.rs` — leave unchanged
- Frontend files other than `src/lib/tauri.ts`

### Error Handling Pattern — Critical Architecture Rules

All Rust command functions MUST return `Result<T, String>` — NEVER `Result<T, SomeErrorType>`:

```rust
// ✅ Correct IPC boundary pattern
#[tauri::command]
pub fn get_app_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

// ✅ With domain error via to_user_message():
#[tauri::command]
pub fn some_command(state: State<AppState>) -> Result<SomeType, String> {
    state.subsystem.do_thing().map_err(|e| e.to_user_message())
}

// ❌ WRONG — raw error type crosses IPC boundary
#[tauri::command]
pub fn bad_command() -> Result<String, PlaybackError> { ... }
```

The `String` error type at the IPC boundary ensures TypeScript receives a plain string (NFR14 compliance). The `to_user_message()` trait is what makes this work for domain errors.

### `errors.rs` — Implementation Pattern

`thiserror` v2 is already in `Cargo.toml`. The pattern for each subsystem error enum:

```rust
// src-tauri/src/errors.rs
use thiserror::Error;

pub trait UserMessage {
    fn to_user_message(&self) -> String;
}

#[derive(Debug, Error)]
pub enum PlaybackError {
    #[error("Playback engine is not ready")]
    NotReady,
    #[error("Scene not found: {0}")]
    InvalidScene(String),
}

impl UserMessage for PlaybackError {
    fn to_user_message(&self) -> String {
        match self {
            PlaybackError::NotReady => {
                "Playback is not ready. Please wait for the engine to initialize.".to_string()
            }
            PlaybackError::InvalidScene(id) => {
                format!("Scene '{}' not found. Please select a valid scene.", id)
            }
        }
    }
}
```

Key rules:
- `#[error("...")]` is the Rust `Display` impl — used internally for logging
- `to_user_message()` is what the frontend receives — must be plain language, actionable (NFR14)
- Never let the `#[error(...)]` strings reach the frontend — always go through `to_user_message()`
- `#[derive(Debug, Error)]` — always both, `Debug` for `cargo test` assertion formatting

### `AppState` Pattern

```rust
// lib.rs — add after module declarations
pub struct AppState {
    // Subsystem handles populated as each epic is implemented.
    // Story 1.5: playback_sender: Option<std::sync::mpsc::Sender<PlaybackCommand>>
    // Story 2.x: capture_handle: Option<CaptureHandle>
    // etc.
}

impl Default for AppState {
    fn default() -> Self {
        Self {}
    }
}
```

Then in `run()`:
```rust
tauri::Builder::default()
    .manage(AppState::default())
    .invoke_handler(tauri::generate_handler![
        commands::system::get_app_version,
    ])
    .plugin(...)
    ...
```

### `tauri.ts` — First Real IPC Wrapper

The `_IPC_BOUNDARY` placeholder and `void invoke;` suppression exist only until the first real wrapper is added. This story removes both.

```typescript
// src/lib/tauri.ts — AFTER this story
import { invoke } from '@tauri-apps/api/core';
import { useErrorStore } from '@/stores';

export type AppVersion = string;

export async function getAppVersion(): Promise<AppVersion> {
  try {
    return await invoke<AppVersion>('get_app_version');
  } catch (err: unknown) {
    // err is always a string — plain language from Rust .to_user_message()
    useErrorStore.getState().setError(err as string);
    throw err;
  }
}

// TODO: Add typed wrappers as commands are implemented in later stories.
// Pattern for invoke wrappers:
//   export async function commandName(param: ParamType): Promise<ReturnType> {
//     try {
//       return await invoke<ReturnType>('command_name', { param });
//     } catch (err: unknown) {
//       useErrorStore.getState().setError(err as string);
//       throw err;
//     }
//   }
//
// Pattern for event listeners:
//   import { listen, type UnlistenFn } from '@tauri-apps/api/event';
//   export function onEventName(cb: (payload: EventPayload) => void): Promise<UnlistenFn> {
//     return listen<EventPayload>('event-name', (e) => cb(e.payload));
//   }
```

Note: The comment block documenting the patterns must be preserved — this is the reference that future story developers will use when adding wrappers.

### Rust Inline Test Pattern

Architecture mandates inline tests for Rust (no separate `__tests__/` directories):

```rust
// At the bottom of errors.rs:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_error_user_message_is_plain_language() {
        let err = PlaybackError::InvalidScene("scene-99".to_string());
        let msg = err.to_user_message();
        // Must NOT contain Rust type names, error codes, or "Err("
        assert!(!msg.contains("PlaybackError"));
        assert!(!msg.contains("InvalidScene"));
        assert!(msg.contains("scene-99")); // scene id preserved for user context
    }

    #[test]
    fn project_error_save_failed_has_actionable_message() {
        let err = ProjectError::SaveFailed("/tmp/test.json".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("ProjectError"));
        assert!(msg.len() > 20); // Ensure it's not just an empty or truncated string
    }
}
```

### Naming Conventions (from architecture.md)

| Layer | Convention |
|---|---|
| Rust function names | `snake_case` |
| Tauri command names (string) | `snake_case` (e.g., `'get_app_version'`) |
| TypeScript function names | `camelCase` (e.g., `getAppVersion`) |
| TypeScript type names | `PascalCase` (e.g., `AppVersion`) |
| IPC event names | `kebab-case` (e.g., `'monitor-update'`) |

### Architecture Compliance Checklist

- [x] `commands/system.rs` is the only file with `#[tauri::command]` added in this story
- [x] Command returns `Result<T, String>` — never a raw error type at IPC boundary
- [x] `to_user_message()` output contains no Rust type names or error codes
- [x] `tauri::generate_handler![]` in `lib.rs` registers the new command
- [x] `src/lib/tauri.ts` is verified (via grep) as the ONLY file calling `invoke()`
- [x] No `unwrap()` or `expect()` added in non-test Rust code
- [x] No business logic in `commands/system.rs` (get_app_version reads env var — acceptable thin wrapper)
- [x] `AppState` is registered via `.manage()` in `lib.rs`

### Previous Story Intelligence (Story 1.3)

From Story 1.3 dev record and code review:
1. **errorStore is wired** — `useErrorStore.getState().setError()` and `clearError()` are concrete implementations. The `tauri.ts` wrapper should use `useErrorStore.getState().setError(err as string)` in catch blocks — this is now safe to uncomment (previously kept as comment in tauri.ts until errorStore existed).
2. **Zustand v5 curried pattern** — stores use `create<T>()((set) => ...)`. Importing `useErrorStore` from `@/stores` in tauri.ts is the correct pattern.
3. **`@/` alias resolves to `src/`** — use `import { useErrorStore } from '@/stores'` in tauri.ts, not a relative path.
4. **App.tsx event listener skeleton** — has TODO comment `// TODO (Story 1.4): listen for 'error-occurred' → useErrorStore.setError()`. This story does NOT implement the event listener — that's still future work. The `useErrorStore` wiring happens only in `tauri.ts` for now.
5. **monitorStore channels are `number[]`** (not Uint8Array) — important for later stories, not directly relevant here.

### Git Intelligence

Recent commits:
- `ec525c7` — story 1-3 developed (Zustand stores + errorStore concrete actions + App.tsx useEffect async IIFE pattern + code review fixes)
- `e3880f8` — 1-2 story complete (AppShell w-full, mt-0, runtime TabId guard, Vitest infrastructure)
- `b5d6bfd` — initial Tauri scaffold (Story 1.1: full directory structure, all placeholder files)

The Rust backend directory structure was created in `b5d6bfd`. Every `commands/*.rs`, `subsystems/*/mod.rs`, and `models/*.rs` file exists as a stub comment. This story adds real content to `errors.rs`, `lib.rs`, and `commands/system.rs` only.

### Project Structure Notes

- `thiserror` v2 is already in `Cargo.toml` — do NOT add it again
- `anyhow` v1 is already in `Cargo.toml` — available for internal error wrapping in subsystems (not needed in this story's scope, but worth noting for future commands)
- The `commands/` module is declared in `lib.rs` as `pub mod commands { pub mod system; ... }` — all command modules are already registered. No changes to module declarations needed.
- `AppState` struct goes in `lib.rs` (per architecture spec), not in a separate file

### References

- Story AC source: [Source: _bmad-output/planning-artifacts/epics.md#Story 1.4]
- IPC command pattern: [Source: _bmad-output/planning-artifacts/architecture.md#Format Patterns — Tauri Command Return Types]
- Error handling rules: [Source: _bmad-output/planning-artifacts/architecture.md#Process Patterns — Rust Error Handling]
- IPC boundary enforcement: [Source: _bmad-output/planning-artifacts/architecture.md#Enforcement Guidelines]
- Backend directory structure: [Source: _bmad-output/planning-artifacts/architecture.md#Complete Project Directory Structure]
- UserMessage trait: [Source: artnet-tool/src-tauri/src/errors.rs]
- lib.rs current state: [Source: artnet-tool/src-tauri/src/lib.rs]
- tauri.ts current state: [Source: artnet-tool/src/lib/tauri.ts]
- Previous story learnings: [Source: _bmad-output/implementation-artifacts/1-3-zustand-state-management-scaffold.md#Dev Agent Record]

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

_No debug issues encountered._

### Completion Notes List

- Expanded `errors.rs` with 6 per-subsystem `thiserror` error enums: `PlaybackError` (3 variants), `CaptureError` (4 variants), `MidiError` (3 variants), `SchedulerError` (3 variants), `ProjectError` (4 variants), `BootError` (3 variants)
- All enums implement `UserMessage` trait — output verified to contain no Rust type names, error codes, or stack traces (enforced by 13 inline tests)
- `AppState` struct added to `lib.rs` with `impl Default`; registered via `.manage(AppState::default())` in `tauri::Builder`
- `invoke_handler` uncommented and populated with `commands::system::get_app_version`
- `get_app_version` command implemented in `commands/system.rs` — returns `Result<String, String>` with version from `CARGO_PKG_VERSION`
- `src/lib/tauri.ts` replaced placeholder (`_IPC_BOUNDARY`, `void invoke`) with real `getAppVersion()` wrapper; `useErrorStore` wired in catch block
- `cargo test`: 14/14 Rust tests pass (12 in errors.rs including new IPC boundary test + 2 in commands/system.rs)
- `cargo build`: zero compilation errors
- `npx tsc --noEmit`: zero TypeScript errors
- `npm test`: 7/7 frontend tests pass (no regressions)
- Grep confirmed `invoke()` appears only in `src/lib/tauri.ts` (no other source file contains `invoke`)

### File List

- `artnet-tool/src-tauri/src/errors.rs` — modified (added 6 thiserror enums + UserMessage impls + 15 inline tests including IPC boundary contract test)
- `artnet-tool/src-tauri/src/lib.rs` — modified (AppState struct + Default impl + .manage() + active invoke_handler)
- `artnet-tool/src-tauri/src/commands/system.rs` — modified (get_app_version command + 2 inline tests)
- `artnet-tool/src/lib/tauri.ts` — modified (AppVersion type + getAppVersion wrapper + errorStore wiring; removed _IPC_BOUNDARY placeholder)

**Note — undocumented git modifications:** `artnet-tool/src/App.tsx` and `artnet-tool/src/stores/monitorStore.ts` appear in `git diff` but were NOT touched in this story. These are uncommitted changes carried forward from Story 1.3 that were not committed in that story's commit. No functional content from those files changed in Story 1.4.

### Change Log

- 2026-03-31: Story 1.4 implemented — Rust error enums with UserMessage trait, AppState scaffold, get_app_version IPC command, tauri.ts first real wrapper
- 2026-03-31: Code review fixes — expanded no_error_enum_name_leaks test to cover all 20 variants (was 6); added ipc_boundary_map_err_produces_plain_string test for AC2 coverage; cleaned up redundant unwrap() in system.rs test; corrected grep claim in Task 6; added note about App.tsx/monitorStore.ts git state; signed off Architecture Compliance Checklist
