# Story 1.1: Project Scaffold & Directory Structure

Status: done

## Story

As a developer,
I want the project scaffolded from the Tauri 2 + React 19 + TypeScript starter template with the full directory structure defined in the architecture,
so that all subsequent development starts from a consistent, architecture-compliant foundation.

## Acceptance Criteria

1. **Given** the developer runs `npm create tauri-app@latest artnet-tool -- --template react-ts`, **When** the scaffold command completes, **Then** the project runs with `npm run tauri dev` without errors.

2. **Given** the scaffolded project, **When** the directory structure is reviewed, **Then** `src/features/cue-pad/`, `src/features/monitor/`, `src/features/settings/` directories exist, **And** `src/stores/` and `src/lib/` directories exist, **And** `src-tauri/src/commands/`, `src-tauri/src/subsystems/`, `src-tauri/src/models/` directories exist with placeholder modules.

3. **Given** the project structure, **When** `src/lib/tauri.ts` is reviewed, **Then** it exports typed wrapper functions for all Tauri `invoke()` calls, **And** no component file contains a direct `invoke()` call.

## Tasks / Subtasks

- [x] Task 1: Initialize project from Tauri 2 starter template (AC: #1)
  - [x] Run: `npm create tauri-app@latest artnet-tool -- --template react-ts`
  - [x] Verify `npm run tauri dev` succeeds — ✅ Rust installed, app compiled and opened successfully
  - [x] `git init` in `/ARTNET-TOOL` (repo root covers artnet-tool/, _bmad/, _bmad-output/, docs/, .claude/)

- [x] Task 2: Create full frontend directory structure (AC: #2)
  - [x] Create `src/features/cue-pad/` with placeholder `index.ts`
  - [x] Create `src/features/monitor/` with placeholder `index.ts`
  - [x] Create `src/features/settings/` with placeholder `index.ts`
  - [x] Create `src/stores/` directory
  - [x] Create `src/lib/` directory
  - [x] Create `src/types/` directory
  - [x] Create `src/components/ui/` directory (for shadcn/ui output)
  - [x] Create `src/components/layout/` directory
  - [x] Create `src/components/shared/` directory

- [x] Task 3: Create full Rust backend directory structure (AC: #2)
  - [x] Create `src-tauri/src/commands/` with placeholder modules: `playback.rs`, `capture.rs`, `keyboard.rs`, `midi.rs`, `scheduler.rs`, `project.rs`, `system.rs`, `network.rs`
  - [x] Create `src-tauri/src/subsystems/playback/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/subsystems/capture/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/subsystems/midi/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/subsystems/scheduler/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/subsystems/project/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/subsystems/boot/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/subsystems/monitor/` with placeholder `mod.rs`
  - [x] Create `src-tauri/src/models/` with placeholder modules: `scene.rs`, `project.rs`, `dmx.rs`
  - [x] Create `src-tauri/src/errors.rs` with `UserMessage` trait stub
  - [x] Create `src-tauri/src/logging.rs` with stub
  - [x] Wire all new modules into `lib.rs` and `main.rs` with `mod` declarations

- [x] Task 4: Create `src/lib/tauri.ts` IPC boundary file (AC: #3)
  - [x] Create `src/lib/tauri.ts` that imports `invoke` from `@tauri-apps/api/core`
  - [x] Add a single typed stub function as example: `export async function getAppVersion(): Promise<string>`
  - [x] Add `// TODO: add invoke wrappers as commands are implemented` comment
  - [x] Confirm `App.tsx` does NOT call `invoke` directly

- [x] Task 5: Install and configure dependencies
  - [x] Add Tailwind CSS v3: installed via `npm install tailwindcss@3 postcss autoprefixer`
  - [x] Configure `tailwind.config.ts` with `darkMode: 'class'` and content paths covering `./src/**/*.{ts,tsx}`
  - [x] Add Zustand: `npm install zustand`
  - [x] Create `components.json` at project root (shadcn config — manual, TTY not available for `npx shadcn init`)
  - [x] Add `src/index.css` with Tailwind directives and dark Slate CSS variable theme
  - [x] Add `cn()` utility to `src/lib/utils.ts` (clsx + tailwind-merge)

- [x] Task 6: Configure Rust Cargo.toml dependencies (stubs only)
  - [x] Add `tokio`, `thiserror`, `anyhow`, `tracing`, `tracing-appender`, `tauri-plugin-global-shortcut`, `spin_sleep`
  - [x] Register `tauri-plugin-global-shortcut` in `lib.rs` builder
  - [x] Verify `cargo check` passes — ✅ confirmed (npm run tauri dev compiled successfully)

- [x] Task 7: Final validation
  - [x] Run `npm run tauri dev` — ✅ app compiled and opened
  - [x] Run `cargo check` — ✅ confirmed via successful tauri dev build
  - [x] Run `npx tsc --noEmit` — ✅ zero TypeScript errors
  - [x] Confirm directory tree matches architecture spec — ✅ verified

## Dev Notes

### Initialization Command

```bash
npm create tauri-app@latest artnet-tool -- --template react-ts
```

This creates the scaffold using **create-tauri-app v4.6.2** (installed version). The template sets up:
- Vite 6 (frontend bundler + HMR)
- React 19 + TypeScript
- Tauri CLI 2.x
- `src-tauri/tauri.conf.json` with correct app metadata placeholders
- `src-tauri/Cargo.toml` with baseline Tauri deps

**Do NOT** use `create-react-app` or any non-Tauri scaffolder. The Tauri scaffold provides correct IPC wiring and cross-platform build config out of the box.

### Complete Target Directory Structure

```
artnet-tool/
├── .github/
│   └── workflows/
│       └── release.yml              ← Placeholder; populated in Story 1.6
├── .gitignore
├── package.json
├── tsconfig.json
├── tsconfig.node.json
├── vite.config.ts
├── tailwind.config.ts
├── components.json                  ← shadcn/ui config (created in Task 5)
├── index.html
│
├── src/
│   ├── main.tsx
│   ├── App.tsx
│   ├── index.css                    ← Tailwind directives + shadcn CSS variables
│   │
│   ├── features/
│   │   ├── cue-pad/
│   │   │   └── index.ts             ← Placeholder export
│   │   ├── monitor/
│   │   │   └── index.ts
│   │   └── settings/
│   │       └── index.ts
│   │
│   ├── components/
│   │   ├── ui/                      ← shadcn/ui output (do not manually edit)
│   │   ├── layout/
│   │   └── shared/
│   │
│   ├── stores/                      ← Zustand slices (populated in Story 1.3)
│   ├── lib/
│   │   ├── tauri.ts                 ← ALL invoke() calls live here (NEVER in components)
│   │   └── utils.ts
│   └── types/
│       ├── events.ts
│       ├── scene.ts
│       └── project.ts
│
└── src-tauri/
    ├── Cargo.toml
    ├── build.rs
    ├── tauri.conf.json
    ├── icons/
    └── src/
        ├── main.rs
        ├── lib.rs
        ├── errors.rs
        ├── logging.rs
        ├── commands/
        │   ├── playback.rs
        │   ├── capture.rs
        │   ├── keyboard.rs
        │   ├── midi.rs
        │   ├── scheduler.rs
        │   ├── project.rs
        │   ├── system.rs
        │   └── network.rs
        ├── subsystems/
        │   ├── playback/mod.rs
        │   ├── capture/mod.rs
        │   ├── midi/mod.rs
        │   ├── scheduler/mod.rs
        │   ├── project/mod.rs
        │   ├── boot/mod.rs
        │   └── monitor/mod.rs
        └── models/
            ├── scene.rs
            ├── project.rs
            └── dmx.rs
```

### Critical Architecture Rules — MUST Follow

**The IPC Boundary Rule (most important rule in the whole project):**
- ALL `invoke()` calls MUST live in `src/lib/tauri.ts` — this is NON-NEGOTIABLE
- Components NEVER call `invoke()` directly
- Stores NEVER call `invoke()` directly
- This is enforced architecturally: if something needs to call Rust, it goes through `src/lib/tauri.ts`

**Tauri command return types (for when commands are added in later stories):**
```rust
// ALWAYS this pattern — never raw errors to frontend
#[tauri::command]
fn my_command(state: State<AppState>) -> Result<MyType, String> {
    state.subsystem.do_thing().map_err(|e| e.to_user_message())
}
```

**Tauri event naming (for when events are added in later stories):**
- Format: `kebab-case`, pattern `{subsystem}-{event}`
- ✅ `playback-state-changed`, `monitor-update`, `capture-started`
- ❌ `playbackStateChanged`, `PLAYBACK_STATE_CHANGED`

**Rust naming:**
- Modules/functions/variables: `snake_case` (compiler enforces)
- Types/structs/enums/traits: `PascalCase` (compiler enforces)
- Constants: `SCREAMING_SNAKE_CASE`

**TypeScript naming:**
- Variables/functions: `camelCase`
- React components and files: `PascalCase` (e.g., `CuePad.tsx`)
- TypeScript types: `PascalCase` — NO `I` prefix (❌ `ISceneConfig`, ✅ `SceneConfig`)
- JSON project file fields: `snake_case` via `#[serde(rename_all = "snake_case")]`

### Rust placeholder module pattern

Each placeholder module should follow this pattern to pass `cargo check`:

```rust
// src-tauri/src/commands/playback.rs
// Placeholder — implementation in Story 1.4 and Epic 3

pub fn register_commands() -> Vec<Box<dyn std::any::Any>> {
    vec![]
}
```

Or simpler — just an empty file with a comment. All that matters is `mod playback;` in the parent compiles.

For `errors.rs`, add the trait stub since it's referenced by the architecture:

```rust
// src-tauri/src/errors.rs
pub trait UserMessage {
    fn to_user_message(&self) -> String;
}
```

### `src/lib/tauri.ts` Starter

```typescript
// src/lib/tauri.ts
// ALL Tauri invoke() and listen() calls go through this file.
// NEVER call invoke() from components or stores directly.
import { invoke } from '@tauri-apps/api/core';

// TODO: add typed wrappers as commands are implemented in later stories
// Example pattern for future use:
// export async function triggerScene(sceneId: number): Promise<PlaybackStatus> {
//   return invoke<PlaybackStatus>('trigger_scene', { sceneId });
// }

export async function getAppVersion(): Promise<string> {
  return invoke<string>('plugin:app|version');
}
```

### Zustand dependency

Install now but DO NOT create store slices yet — that's Story 1.3. Just `npm install zustand` so the dep is locked.

### shadcn/ui initialization

Run `npx shadcn@latest init` and select:
- Style: **Default**
- Base color: **Slate**
- CSS variables: **Yes**
- Dark mode: configured via `darkMode: 'class'` in `tailwind.config.ts`

This creates `components.json` and sets up `src/lib/utils.ts` with the `cn()` helper. Do NOT manually edit files in `src/components/ui/` — always regenerate via the shadcn CLI.

### Cargo.toml note

Add deps as stubs now. Do NOT implement any logic using them — that happens in later stories. The purpose is to lock versions early and ensure `cargo check` passes with all deps present.

For `pcap` and `midir` — do NOT add them in this story. They require system libraries (libpcap, ALSA) that complicate the dev setup. They are added in Story 2.1 (capture) and Story 4.2 (MIDI) respectively when their system dependencies are also documented.

For `tauri-plugin-global-shortcut` — add to `Cargo.toml` AND register in `main.rs` as: `.plugin(tauri_plugin_global_shortcut::Builder::new().build())` — this is required by Tauri 2 plugin architecture.

### `vite.config.ts` — no changes needed

The Tauri scaffold generates a correct `vite.config.ts`. Do NOT change the Vite port or devtools settings.

### Project Structure Notes

- The scaffold puts `src/App.tsx` and `src/main.tsx` at the top level — keep them there (do not move to subdirs)
- `index.html` is at the project root (not `public/`) — this is correct for Vite + Tauri
- `src-tauri/icons/` will contain app icons — use the scaffold's default icons for now; icon replacement is out of scope
- `.gitignore` from scaffold already covers `node_modules/`, `dist/`, `src-tauri/target/` — do NOT add `src-tauri/target/` manually as it may already be there

### References

- Initialization command and starter rationale: [Source: _bmad-output/planning-artifacts/architecture.md#Starter Template Evaluation]
- Complete directory structure: [Source: _bmad-output/planning-artifacts/architecture.md#Complete Project Directory Structure]
- Naming conventions: [Source: _bmad-output/planning-artifacts/architecture.md#Naming Patterns]
- IPC boundary pattern (`src/lib/tauri.ts`): [Source: _bmad-output/planning-artifacts/architecture.md#Communication Patterns]
- Anti-patterns to avoid: [Source: _bmad-output/planning-artifacts/architecture.md#Anti-Patterns to Avoid]
- `UserMessage` trait pattern: [Source: _bmad-output/planning-artifacts/architecture.md#Rust Error Handling]
- Story AC source: [Source: _bmad-output/planning-artifacts/epics.md#Story 1.1]
- Epic 1 goal: [Source: _bmad-output/planning-artifacts/epics.md#Epic 1: Foundation & Application Shell]

## Dev Agent Record

### Agent Model Used

claude-sonnet-4-6

### Debug Log References

- `npm create tauri-app` required `--yes` flag to bypass TTY requirement in non-interactive shell
- Installed `tailwindcss@3` explicitly (v4 has no `init` command)
- `npx shadcn init` requires TTY — created `components.json` and CSS vars manually instead
- `tailwind.config.js` auto-generated by init — replaced with `tailwind.config.ts` per architecture spec
- Added `@types/node` for `path` import in `vite.config.ts` (path alias support)

### Completion Notes List

- ✅ Scaffold created at `artnet-tool/` using create-tauri-app v4.6.2
- ✅ Full frontend directory structure created (features/cue-pad, monitor, settings; stores; lib; types; components/ui,layout,shared)
- ✅ Full Rust backend structure created (commands/×8, subsystems/×7 with mod.rs, models/×3, errors.rs, logging.rs)
- ✅ `src/lib/tauri.ts` created — only file with `import { invoke }` — IPC boundary enforced
- ✅ `App.tsx` rewritten — zero direct `invoke()` calls
- ✅ Tailwind CSS v3 + dark Slate theme configured (tailwind.config.ts, index.css with CSS vars)
- ✅ Zustand installed
- ✅ `components.json` created for shadcn/ui
- ✅ `cn()` utility in `src/lib/utils.ts` (clsx + tailwind-merge)
- ✅ `vite.config.ts` updated with `@/` path alias
- ✅ `tsconfig.json` updated with `paths: { "@/*": ["./src/*"] }`
- ✅ Cargo.toml updated: tokio, thiserror, anyhow, tracing, tracing-appender, tauri-plugin-global-shortcut, spin_sleep
- ✅ `tauri-plugin-global-shortcut` registered in `lib.rs` builder
- ✅ TypeScript: `npx tsc --noEmit` → zero errors
- ⚠️ BLOCKED: `cargo check` and `npm run tauri dev` require Rust — install from https://rustup.rs
- ✅ [Code Review] H1: Removed broken `getAppVersion()` stub (plugin:app not registered) — replaced with `_IPC_BOUNDARY` placeholder
- ✅ [Code Review] H2: Installed `@tauri-apps/plugin-global-shortcut` npm package (was missing from package.json)
- ✅ [Code Review] H3: Added `.gitkeep` to stores/, components/ui/, components/layout/, components/shared/
- ✅ [Code Review] M1: Deleted `src/App.css` (scaffold leftover with conflicting light-theme `:root` styles)
- ✅ [Code Review] M2: Set proper CSP in `tauri.conf.json` (was `null`); also fixed window title to "ARTNET TOOL" and default size to 1000×700
- ✅ [Code Review] M3: Updated git init task description to include `git init` command explicitly
- ✅ [Code Review] L2: Added `class="dark"` to `<html>` in `index.html` — activates Tailwind `dark:` variants consistently with `darkMode: 'class'`
- ✅ [Code Review] L3: Fixed `listen` import module note in `src/lib/tauri.ts` (listen is in `@tauri-apps/api/event`, not `core`)
- ✅ AC #1 fully verified: Rust installed by user, `npm run tauri dev` compiled and opened app window successfully (2026-03-13)

### File List

artnet-tool/.github/workflows/release.yml
artnet-tool/.gitignore (scaffold)
artnet-tool/components.json
artnet-tool/index.html (modified — class="dark", title, scaffold)
artnet-tool/package.json (modified — deps added)
artnet-tool/package-lock.json
artnet-tool/postcss.config.js
artnet-tool/tailwind.config.ts
artnet-tool/tsconfig.json (modified — paths alias)
artnet-tool/tsconfig.node.json (scaffold)
artnet-tool/vite.config.ts (modified — path alias)
artnet-tool/src/App.tsx (replaced)
artnet-tool/src/index.css
artnet-tool/src/main.tsx (modified — added index.css import)
artnet-tool/src/features/cue-pad/index.ts
artnet-tool/src/features/monitor/index.ts
artnet-tool/src/features/settings/index.ts
artnet-tool/src/lib/tauri.ts (modified — removed broken getAppVersion stub, fixed listen comment)
artnet-tool/src/lib/utils.ts
artnet-tool/src/types/events.ts
artnet-tool/src/types/scene.ts
artnet-tool/src/types/project.ts
artnet-tool/src/stores/.gitkeep
artnet-tool/src/components/ui/.gitkeep
artnet-tool/src/components/layout/.gitkeep
artnet-tool/src/components/shared/.gitkeep
artnet-tool/src-tauri/Cargo.toml (modified — deps added)
artnet-tool/src-tauri/tauri.conf.json (modified — CSP, title, window size)
artnet-tool/src-tauri/src/lib.rs (replaced)
artnet-tool/src-tauri/src/errors.rs
artnet-tool/src-tauri/src/logging.rs
artnet-tool/src-tauri/src/commands/playback.rs
artnet-tool/src-tauri/src/commands/capture.rs
artnet-tool/src-tauri/src/commands/keyboard.rs
artnet-tool/src-tauri/src/commands/midi.rs
artnet-tool/src-tauri/src/commands/scheduler.rs
artnet-tool/src-tauri/src/commands/project.rs
artnet-tool/src-tauri/src/commands/system.rs
artnet-tool/src-tauri/src/commands/network.rs
artnet-tool/src-tauri/src/subsystems/playback/mod.rs
artnet-tool/src-tauri/src/subsystems/capture/mod.rs
artnet-tool/src-tauri/src/subsystems/midi/mod.rs
artnet-tool/src-tauri/src/subsystems/scheduler/mod.rs
artnet-tool/src-tauri/src/subsystems/project/mod.rs
artnet-tool/src-tauri/src/subsystems/boot/mod.rs
artnet-tool/src-tauri/src/subsystems/monitor/mod.rs
artnet-tool/src-tauri/src/models/scene.rs
artnet-tool/src-tauri/src/models/project.rs
artnet-tool/src-tauri/src/models/dmx.rs
DELETED: artnet-tool/src/App.css
