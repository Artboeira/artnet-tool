// subsystems/playback/types.rs — Shared types for the playback subsystem
//
// PlaybackCommand: sent TO the engine thread via mpsc channel (internal, no Serialize needed)
// PlaybackStatus:  returned FROM Tauri commands at the IPC boundary (must be Serialize)
// PlaybackMode:    per-scene configuration stored in project files (Serialize + Deserialize)

use serde::{Deserialize, Serialize};

// ── Commands ─────────────────────────────────────────────────────────────────

/// Commands sent to the playback engine thread via `mpsc::Sender<PlaybackCommand>`.
///
/// The engine processes these non-blocking on each ~1ms tick via `try_recv()`.
/// This type is internal — it never crosses the IPC boundary (no Serialize needed).
#[derive(Debug)]
pub enum PlaybackCommand {
    /// Trigger a scene by ID. No-op if scene not found (error reported via Tauri event in Story 3.2).
    TriggerScene(u32),
    /// Stop the currently playing scene and return to idle state.
    StopPlayback,
    /// Set playback speed multiplier. Valid range: 0.25–4.0. Applied to current and future playback.
    SetSpeed(f32),
    /// Graceful shutdown — thread exits the loop and the OS thread terminates.
    Shutdown,
}

// ── Status ───────────────────────────────────────────────────────────────────

/// Current playback engine state. Returned by Tauri commands at the IPC boundary.
///
/// The `#[serde(tag = "state")]` produces JSON like `{"state":"idle"}` or
/// `{"state":"playing","scene_id":1,"mode":"loop","speed":1.0}` — suitable for TypeScript.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum PlaybackStatus {
    Idle,
    Playing {
        scene_id: u32,
        mode: PlaybackMode,
        speed: f32,
    },
    Stopping,
}

// ── Mode ─────────────────────────────────────────────────────────────────────

/// Playback mode per scene. Stored in the project file (both Serialize and Deserialize needed).
///
/// - `Loop`     — repeat scene indefinitely until stopped
/// - `OneShot`  — play once and return to idle
/// - `PingPong` — play forward then reverse, repeat
/// - `Reverse`  — play scene backwards, then idle (OneShot in reverse)
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaybackMode {
    Loop,
    OneShot,
    PingPong,
    Reverse,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playback_command_variants_instantiate() {
        let _trigger = PlaybackCommand::TriggerScene(1);
        let _stop = PlaybackCommand::StopPlayback;
        let _speed = PlaybackCommand::SetSpeed(1.5);
        let _shutdown = PlaybackCommand::Shutdown;
    }

    #[test]
    fn playback_status_variants_instantiate() {
        let _idle = PlaybackStatus::Idle;
        let _playing = PlaybackStatus::Playing {
            scene_id: 42,
            mode: PlaybackMode::Loop,
            speed: 1.0,
        };
        let _stopping = PlaybackStatus::Stopping;
    }

    #[test]
    fn playback_mode_variants_instantiate() {
        let _loop_mode = PlaybackMode::Loop;
        let _one_shot = PlaybackMode::OneShot;
        let _ping_pong = PlaybackMode::PingPong;
        let _reverse = PlaybackMode::Reverse;
    }

    #[test]
    fn playback_mode_is_copy_and_eq() {
        let mode = PlaybackMode::Loop;
        let mode_copy = mode; // Copy — no move
        assert_eq!(mode, mode_copy);
    }

    #[test]
    fn playback_status_partial_eq() {
        assert_eq!(PlaybackStatus::Idle, PlaybackStatus::Idle);
        assert_eq!(PlaybackStatus::Stopping, PlaybackStatus::Stopping);
        assert_ne!(PlaybackStatus::Idle, PlaybackStatus::Stopping);
        let a = PlaybackStatus::Playing { scene_id: 1, mode: PlaybackMode::Loop, speed: 1.0 };
        let b = PlaybackStatus::Playing { scene_id: 1, mode: PlaybackMode::Loop, speed: 1.0 };
        assert_eq!(a, b);
    }

    // ── IPC serialization contract ────────────────────────────────────────────

    #[test]
    fn playback_status_idle_serializes_to_tagged_json() {
        let json = serde_json::to_string(&PlaybackStatus::Idle).expect("serialize Idle");
        assert_eq!(json, r#"{"state":"idle"}"#);
    }

    #[test]
    fn playback_status_playing_serializes_to_tagged_json() {
        let status = PlaybackStatus::Playing {
            scene_id: 42,
            mode: PlaybackMode::Loop,
            speed: 1.5,
        };
        let json = serde_json::to_string(&status).expect("serialize Playing");
        assert!(json.contains(r#""state":"playing""#), "missing state tag: {json}");
        assert!(json.contains(r#""scene_id":42"#), "missing scene_id: {json}");
        assert!(json.contains(r#""speed":1.5"#), "missing speed: {json}");
        assert!(json.contains(r#""mode":"loop""#), "missing mode: {json}");
    }

    #[test]
    fn playback_status_stopping_serializes_to_tagged_json() {
        let json = serde_json::to_string(&PlaybackStatus::Stopping).expect("serialize Stopping");
        assert_eq!(json, r#"{"state":"stopping"}"#);
    }

    #[test]
    fn playback_mode_serializes_as_snake_case() {
        assert_eq!(serde_json::to_string(&PlaybackMode::Loop).unwrap(), r#""loop""#);
        assert_eq!(serde_json::to_string(&PlaybackMode::OneShot).unwrap(), r#""one_shot""#);
        assert_eq!(serde_json::to_string(&PlaybackMode::PingPong).unwrap(), r#""ping_pong""#);
        assert_eq!(serde_json::to_string(&PlaybackMode::Reverse).unwrap(), r#""reverse""#);
    }

    #[test]
    fn playback_mode_roundtrips_through_json() {
        for mode in [PlaybackMode::Loop, PlaybackMode::OneShot, PlaybackMode::PingPong, PlaybackMode::Reverse] {
            let json = serde_json::to_string(&mode).expect("serialize");
            let decoded: PlaybackMode = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(mode, decoded);
        }
    }
}
