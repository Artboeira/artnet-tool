// errors.rs — UserMessage trait and per-subsystem error enums
// All command handlers return Result<T, String> by calling .map_err(|e| e.to_user_message())
// This ensures users NEVER see raw Rust errors, stack traces, or error codes.

use thiserror::Error;

/// Converts a domain error into a plain-language user-facing message.
/// Every error type used at the Tauri command boundary must implement this.
pub trait UserMessage {
    fn to_user_message(&self) -> String;
}

// ── Playback errors ──────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum PlaybackError {
    #[error("Playback engine is not ready")]
    NotReady,
    #[error("Scene not found: {0}")]
    InvalidScene(String),
    #[error("Invalid playback speed: {0}")]
    InvalidSpeed(f32),
}

impl UserMessage for PlaybackError {
    fn to_user_message(&self) -> String {
        match self {
            PlaybackError::NotReady => {
                "Playback is not ready. Please wait for the engine to initialize and try again."
                    .to_string()
            }
            PlaybackError::InvalidScene(id) => {
                format!(
                    "Scene '{}' was not found. Please select a valid scene from the cue pad.",
                    id
                )
            }
            PlaybackError::InvalidSpeed(speed) => {
                format!(
                    "Speed '{:.2}' is out of range. Please choose a value between 0.25 and 4.0.",
                    speed
                )
            }
        }
    }
}

// ── Capture errors ───────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum CaptureError {
    #[error("Network interface not found: {0}")]
    InterfaceNotFound(String),
    #[error("Capture is already running")]
    AlreadyRunning,
    #[error("Capture is not running")]
    NotRunning,
    #[error("Failed to open capture device: {0}")]
    DeviceOpenFailed(String),
}

impl UserMessage for CaptureError {
    fn to_user_message(&self) -> String {
        match self {
            CaptureError::InterfaceNotFound(name) => format!(
                "Network interface '{}' was not found. Please select a different interface in Settings.",
                name
            ),
            CaptureError::AlreadyRunning => {
                "Capture is already running. Stop the current capture before starting a new one."
                    .to_string()
            }
            CaptureError::NotRunning => {
                "No capture is currently running. Start a capture session first.".to_string()
            }
            CaptureError::DeviceOpenFailed(reason) => format!(
                "Failed to open the network interface: {}. Check that the interface is available and you have the required permissions.",
                reason
            ),
        }
    }
}

// ── MIDI errors ──────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum MidiError {
    #[error("MIDI device not found: {0}")]
    DeviceNotFound(String),
    #[error("Failed to connect to MIDI device: {0}")]
    ConnectionFailed(String),
    #[error("Invalid MIDI mapping: {0}")]
    InvalidMapping(String),
}

impl UserMessage for MidiError {
    fn to_user_message(&self) -> String {
        match self {
            MidiError::DeviceNotFound(name) => format!(
                "MIDI device '{}' was not found. Check that the device is connected and recognized by the system.",
                name
            ),
            MidiError::ConnectionFailed(reason) => format!(
                "Could not connect to the MIDI device: {}. Try disconnecting and reconnecting the device.",
                reason
            ),
            MidiError::InvalidMapping(details) => format!(
                "The MIDI mapping is invalid: {}. Please review and correct the mapping in Settings.",
                details
            ),
        }
    }
}

// ── Scheduler errors ─────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Invalid schedule configuration: {0}")]
    InvalidSchedule(String),
    #[error("Schedule not found: {0}")]
    NotFound(String),
    #[error("Schedule conflict: {0}")]
    Conflict(String),
}

impl UserMessage for SchedulerError {
    fn to_user_message(&self) -> String {
        match self {
            SchedulerError::InvalidSchedule(details) => format!(
                "The schedule configuration is invalid: {}. Please review the time settings and try again.",
                details
            ),
            SchedulerError::NotFound(id) => format!(
                "Schedule '{}' was not found. It may have already been removed.",
                id
            ),
            SchedulerError::Conflict(details) => format!(
                "Schedule conflict detected: {}. Adjust the timing to avoid overlapping schedules.",
                details
            ),
        }
    }
}

// ── Project errors ───────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Failed to load project from '{0}'")]
    LoadFailed(String),
    #[error("Failed to save project to '{0}'")]
    SaveFailed(String),
    #[error("Project file is corrupted: {0}")]
    Corrupted(String),
    #[error("Unsupported project file version: {0}")]
    UnsupportedVersion(String),
}

impl UserMessage for ProjectError {
    fn to_user_message(&self) -> String {
        match self {
            ProjectError::LoadFailed(path) => format!(
                "Could not load the project file at '{}'. Check that the file exists and you have permission to read it.",
                path
            ),
            ProjectError::SaveFailed(path) => format!(
                "Could not save the project to '{}'. Check that the directory exists and you have write permission.",
                path
            ),
            ProjectError::Corrupted(details) => format!(
                "The project file appears to be corrupted: {}. Try restoring from a backup.",
                details
            ),
            ProjectError::UnsupportedVersion(version) => format!(
                "Project file version '{}' is not supported by this version of ARTNET-TOOL. Please update the application.",
                version
            ),
        }
    }
}

// ── Boot errors ──────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum BootError {
    #[error("Auto-start is not supported on this platform")]
    PlatformNotSupported,
    #[error("Failed to register auto-start: {0}")]
    RegistrationFailed(String),
    #[error("Failed to remove auto-start: {0}")]
    RemovalFailed(String),
}

impl UserMessage for BootError {
    fn to_user_message(&self) -> String {
        match self {
            BootError::PlatformNotSupported => {
                "Auto-start is not supported on this operating system.".to_string()
            }
            BootError::RegistrationFailed(reason) => format!(
                "Could not enable auto-start: {}. Try running the application with administrator privileges.",
                reason
            ),
            BootError::RemovalFailed(reason) => format!(
                "Could not disable auto-start: {}. Try running the application with administrator privileges.",
                reason
            ),
        }
    }
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ---- PlaybackError tests ----

    #[test]
    fn playback_not_ready_has_plain_language_message() {
        let msg = PlaybackError::NotReady.to_user_message();
        assert!(!msg.contains("PlaybackError"));
        assert!(!msg.contains("NotReady"));
        assert!(msg.len() > 20);
    }

    #[test]
    fn playback_invalid_scene_includes_scene_id() {
        let err = PlaybackError::InvalidScene("scene-99".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("PlaybackError"));
        assert!(!msg.contains("InvalidScene"));
        assert!(msg.contains("scene-99"));
    }

    #[test]
    fn playback_invalid_speed_includes_value() {
        let err = PlaybackError::InvalidSpeed(5.0);
        let msg = err.to_user_message();
        assert!(!msg.contains("PlaybackError"));
        assert!(msg.contains("5.00"));
    }

    // ---- CaptureError tests ----

    #[test]
    fn capture_interface_not_found_includes_name() {
        let err = CaptureError::InterfaceNotFound("eth0".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("CaptureError"));
        assert!(msg.contains("eth0"));
    }

    #[test]
    fn capture_already_running_has_actionable_message() {
        let msg = CaptureError::AlreadyRunning.to_user_message();
        assert!(!msg.contains("CaptureError"));
        assert!(msg.len() > 20);
    }

    // ---- MidiError tests ----

    #[test]
    fn midi_device_not_found_includes_device_name() {
        let err = MidiError::DeviceNotFound("Arturia KeyStep".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("MidiError"));
        assert!(msg.contains("Arturia KeyStep"));
    }

    // ---- SchedulerError tests ----

    #[test]
    fn scheduler_invalid_schedule_has_plain_message() {
        let err = SchedulerError::InvalidSchedule("start time is after end time".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("SchedulerError"));
        assert!(msg.contains("start time is after end time"));
    }

    // ---- ProjectError tests ----

    #[test]
    fn project_save_failed_includes_path() {
        let err = ProjectError::SaveFailed("/home/user/project.json".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("ProjectError"));
        assert!(!msg.contains("SaveFailed"));
        assert!(msg.contains("/home/user/project.json"));
        assert!(msg.len() > 20);
    }

    #[test]
    fn project_load_failed_includes_path() {
        let err = ProjectError::LoadFailed("/tmp/test.json".to_string());
        let msg = err.to_user_message();
        assert!(!msg.contains("ProjectError"));
        assert!(msg.contains("/tmp/test.json"));
    }

    // ---- BootError tests ----

    #[test]
    fn boot_platform_not_supported_has_message() {
        let msg = BootError::PlatformNotSupported.to_user_message();
        assert!(!msg.contains("BootError"));
        assert!(!msg.contains("PlatformNotSupported"));
        assert!(msg.len() > 10);
    }

    // ---- IPC boundary contract ----

    #[test]
    fn ipc_boundary_map_err_produces_plain_string() {
        // Validates AC2: simulates the command handler pattern
        //   domain_result.map_err(|e| e.to_user_message())
        // ensuring errors cross the IPC boundary as plain strings, never raw Rust types.
        fn simulate_failing_command() -> Result<String, String> {
            let domain_err = PlaybackError::InvalidScene("scene-42".to_string());
            Err(domain_err).map_err(|e: PlaybackError| e.to_user_message())
        }
        let result = simulate_failing_command();
        assert!(result.is_err(), "simulated command should return Err");
        let err_str = result.unwrap_err();
        assert!(!err_str.contains("PlaybackError"), "raw type leaked: {err_str}");
        assert!(!err_str.contains("InvalidScene"), "variant name leaked: {err_str}");
        assert!(err_str.contains("scene-42"), "payload lost in message: {err_str}");
        assert!(err_str.len() > 20, "message too short to be actionable: {err_str}");
    }

    // ---- General contract: no raw type names reach users ----

    #[test]
    fn no_error_enum_name_leaks_to_user_message() {
        // Exhaustive: every variant is tested, including all string-interpolated ones
        // whose embedded payloads could theoretically carry forbidden words.
        let errors: Vec<Box<dyn UserMessage>> = vec![
            Box::new(PlaybackError::NotReady),
            Box::new(PlaybackError::InvalidScene("scene-1".to_string())),
            Box::new(PlaybackError::InvalidSpeed(9.9)),
            Box::new(CaptureError::InterfaceNotFound("eth0".to_string())),
            Box::new(CaptureError::AlreadyRunning),
            Box::new(CaptureError::NotRunning),
            Box::new(CaptureError::DeviceOpenFailed("permission denied".to_string())),
            Box::new(MidiError::DeviceNotFound("KeyStep".to_string())),
            Box::new(MidiError::ConnectionFailed("timeout".to_string())),
            Box::new(MidiError::InvalidMapping("bad cc".to_string())),
            Box::new(SchedulerError::InvalidSchedule("start after end".to_string())),
            Box::new(SchedulerError::NotFound("sched-1".to_string())),
            Box::new(SchedulerError::Conflict("overlapping slots".to_string())),
            Box::new(ProjectError::LoadFailed("/tmp/test.json".to_string())),
            Box::new(ProjectError::SaveFailed("/tmp/test.json".to_string())),
            Box::new(ProjectError::Corrupted("bad header".to_string())),
            Box::new(ProjectError::UnsupportedVersion("v99".to_string())),
            Box::new(BootError::PlatformNotSupported),
            Box::new(BootError::RegistrationFailed("access denied".to_string())),
            Box::new(BootError::RemovalFailed("access denied".to_string())),
        ];
        let forbidden = ["Error", "Err(", "unwrap", "panic", "thread"];
        for err in &errors {
            let msg = err.to_user_message();
            for word in &forbidden {
                assert!(
                    !msg.contains(word),
                    "Message '{}' contains forbidden word '{}'",
                    msg,
                    word
                );
            }
        }
    }
}
