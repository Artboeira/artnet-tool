// src/lib/tauri.ts
// ============================================================
// ALL Tauri invoke() and listen() calls go through this file.
// NEVER call invoke() from components or stores directly.
// invoke() → import from '@tauri-apps/api/core'
// listen() → import from '@tauri-apps/api/event'
// ============================================================
import { invoke } from '@tauri-apps/api/core';

// TODO: Add typed wrappers as commands are implemented in later stories.
//
// Pattern for invoke wrappers:
//   import { invoke } from '@tauri-apps/api/core';
//   export async function commandName(param: ParamType): Promise<ReturnType> {
//     try {
//       return await invoke<ReturnType>('command_name', { param });
//     } catch (err: unknown) {
//       // err is always a string — plain language from Rust .to_user_message()
//       // useErrorStore.getState().setError(err as string);
//       throw err;
//     }
//   }
//
// Pattern for event listeners:
//   import { listen, type UnlistenFn } from '@tauri-apps/api/event'; // NOTE: 'event', not 'core'
//   export function onPlaybackStateChanged(
//     cb: (payload: PlaybackStateChangedPayload) => void
//   ): Promise<UnlistenFn> {
//     return listen<PlaybackStateChangedPayload>('playback-state-changed', (e) => cb(e.payload));
//   }

// Placeholder export so this module is non-empty until real commands are added.
// Remove once first real command wrapper is added.
export const _IPC_BOUNDARY = true;

// NOTE: invoke is imported above for use in future command wrappers.
// Suppress unused-import lint warning until first wrapper is added.
void invoke;
