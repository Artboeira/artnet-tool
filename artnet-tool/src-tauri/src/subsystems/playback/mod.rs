// subsystems/playback/ — Frame-accurate ArtNet/sACN playback engine
// Isolated OS thread with spin_sleep for ±1ms timing (NFR1, NFR5)
// Receives commands via std::sync::mpsc channel — fully isolated from Tokio runtime
//
// Public API:
//   spawn_thread(receiver) → JoinHandle   call from lib.rs run() on startup
//   PlaybackCommand        → send via AppState.playback_sender from commands/
//   PlaybackStatus         → returned by Tauri commands at the IPC boundary (Story 3.2)
//   PlaybackMode           → stored in project file per-scene (models/scene.rs in future)

mod engine;
pub mod types;

pub use types::{PlaybackCommand, PlaybackMode, PlaybackStatus};

/// Spawns the playback engine on a dedicated OS thread.
///
/// The thread runs an isolated ~1ms loop using `spin_sleep` for timing precision.
/// It exits cleanly when:
/// - `PlaybackCommand::Shutdown` is sent through the channel, or
/// - The `Sender<PlaybackCommand>` stored in `AppState` is dropped (normal app exit)
///
/// # Usage in `lib.rs`
/// ```rust,ignore
/// let (playback_sender, playback_receiver) =
///     std::sync::mpsc::channel::<subsystems::playback::PlaybackCommand>();
/// let _playback_handle = subsystems::playback::spawn_thread(playback_receiver);
/// // Drop _playback_handle to detach — thread self-exits when sender drops with AppState
/// ```
///
/// # Panics
/// Panics only if the OS thread cannot be allocated (extremely rare system-level failure).
pub fn spawn_thread(
    receiver: std::sync::mpsc::Receiver<PlaybackCommand>,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("playback-engine".to_string())
        .spawn(move || engine::run_loop(receiver))
        .expect("failed to spawn playback engine thread")
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn playback_thread_exits_cleanly_on_shutdown() {
        let (sender, receiver) = mpsc::channel::<PlaybackCommand>();
        let handle = spawn_thread(receiver);
        sender
            .send(PlaybackCommand::Shutdown)
            .expect("send Shutdown should succeed before thread exits");
        let result = handle.join();
        assert!(result.is_ok(), "playback thread panicked on shutdown: {:?}", result);
    }

    #[test]
    fn playback_thread_exits_on_sender_drop() {
        let (sender, receiver) = mpsc::channel::<PlaybackCommand>();
        let handle = spawn_thread(receiver);
        drop(sender); // thread detects TryRecvError::Disconnected and exits
        let result = handle.join();
        assert!(
            result.is_ok(),
            "playback thread panicked when sender was dropped: {:?}",
            result
        );
    }
}
