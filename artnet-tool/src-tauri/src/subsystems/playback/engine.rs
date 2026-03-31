// subsystems/playback/engine.rs — Isolated OS thread loop for the playback engine
//
// ┌─────────────────────────────────────────────────────────────────────────┐
// │  ARCHITECTURE CRITICAL: This code runs on a dedicated std::thread.     │
// │  NEVER call async functions or Tokio primitives from this module.       │
// │  spin_sleep maintains ±1ms tick precision (NFR1) without blocking       │
// │  the Tokio runtime (NFR5).                                              │
// └─────────────────────────────────────────────────────────────────────────┘

use std::sync::mpsc::{Receiver, TryRecvError};
use std::time::Duration;

use super::types::PlaybackCommand;

/// Main loop for the playback engine thread.
///
/// Runs at a ~1ms tick rate using `spin_sleep` for timing precision (NFR1).
/// This is a scaffold — the tick body is intentionally minimal (Story 1.5).
/// Full ArtNet/sACN output is added in Story 3.2.
///
/// # Exit conditions
/// The loop exits cleanly (without panicking) when:
/// - [`PlaybackCommand::Shutdown`] is received through the channel, or
/// - The [`Sender`] end of the channel is dropped (detected as `TryRecvError::Disconnected`)
///
/// [`Sender`]: std::sync::mpsc::Sender
pub(super) fn run_loop(receiver: Receiver<PlaybackCommand>) {
    // ±1ms tick duration.
    // spin_sleep uses OS sleep for the bulk of the wait + CPU spin for the last
    // sub-millisecond fraction, achieving ±1ms jitter without burning CPU (NFR4).
    let tick = Duration::from_millis(1);

    loop {
        // Process one pending command per tick (non-blocking — never blocks the OS thread).
        match receiver.try_recv() {
            Ok(PlaybackCommand::Shutdown) => {
                // Graceful shutdown requested — exit the loop cleanly.
                break;
            }
            Ok(PlaybackCommand::TriggerScene(_scene_id)) => {
                // TODO Story 3.2: load scene from Arc<RwLock<ProjectState>>, start playback loop
            }
            Ok(PlaybackCommand::StopPlayback) => {
                // TODO Story 3.2: stop current playback, emit playback-state-changed event
            }
            Ok(PlaybackCommand::SetSpeed(_speed)) => {
                // TODO Story 3.2: update playback speed multiplier for current playback
            }
            Err(TryRecvError::Disconnected) => {
                // The Sender stored in AppState was dropped — normal app shutdown path.
                // Exit cleanly; do not panic.
                break;
            }
            Err(TryRecvError::Empty) => {
                // No commands pending — normal steady state, continue to next tick.
            }
        }

        // Maintain ~1ms tick rate using spin_sleep (NFR1, NFR5).
        spin_sleep::sleep(tick);
    }
}
