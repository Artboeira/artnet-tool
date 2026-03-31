// src/lib/tauri.ts
// ============================================================
// ALL Tauri invoke() and listen() calls go through this file.
// NEVER call invoke() from components or stores directly.
// invoke() → import from '@tauri-apps/api/core'
// listen() → import from '@tauri-apps/api/event'
// ============================================================
import { invoke } from '@tauri-apps/api/core';
import { useErrorStore } from '@/stores';

// ── Types ────────────────────────────────────────────────────────────────────

export type AppVersion = string;

// ── System commands ──────────────────────────────────────────────────────────

/**
 * Returns the application version string (e.g., "0.1.0").
 * Demonstrates the full IPC pattern: typed wrapper, error surface, error store wiring.
 * Implemented in: src-tauri/src/commands/system.rs
 */
export async function getAppVersion(): Promise<AppVersion> {
  try {
    return await invoke<AppVersion>('get_app_version');
  } catch (err: unknown) {
    // err is always a string — plain language from Rust .to_user_message()
    useErrorStore.getState().setError(err as string);
    throw err;
  }
}

// ── Template: adding future commands ─────────────────────────────────────────
//
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
//   import { listen, type UnlistenFn } from '@tauri-apps/api/event'; // NOTE: 'event', not 'core'
//   export function onEventName(
//     cb: (payload: EventPayload) => void
//   ): Promise<UnlistenFn> {
//     return listen<EventPayload>('event-name', (e) => cb(e.payload));
//   }
//
// Story-specific TODOs:
//   Story 2.1: listInterfaces(), setInterface()  → commands/network.rs
//   Story 2.2: startCapture(), stopCapture()     → commands/capture.rs
//   Story 2.3: onMonitorUpdate()                 → listen 'monitor-update'
//   Story 3.2: triggerScene(), stopPlayback()    → commands/playback.rs
//   Story 4.1: registerShortcut()                → commands/keyboard.rs
//   Story 4.2: getMidiDevices()                  → commands/midi.rs
//   Story 5.x: loadProject(), saveProject()      → commands/project.rs
//   Story 6.1: enableAutostart()                 → commands/system.rs
