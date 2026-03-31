// lib.rs — AppState, module declarations, public API surface
// Business logic lives in subsystems/; command handlers in commands/; shared types in models/

// ── Module declarations ──────────────────────────────────────────────────────

pub mod errors;
pub mod logging;

pub mod models {
    pub mod dmx;
    pub mod project;
    pub mod scene;
}

pub mod subsystems {
    pub mod boot;
    pub mod capture;
    pub mod midi;
    pub mod monitor;
    pub mod playback;
    pub mod project;
    pub mod scheduler;
}

pub mod commands {
    pub mod capture;
    pub mod keyboard;
    pub mod midi;
    pub mod network;
    pub mod playback;
    pub mod project;
    pub mod scheduler;
    pub mod system;
}

// ── AppState ─────────────────────────────────────────────────────────────────
// Central Tauri managed state — subsystem handles added as each epic is implemented.
// Accessed in command handlers via `state: tauri::State<'_, AppState>`.

pub struct AppState {
    /// Sender half of the playback engine command channel.
    /// Send `PlaybackCommand::Shutdown` for a clean thread exit, or simply drop AppState —
    /// the engine thread detects `TryRecvError::Disconnected` and exits without panicking.
    pub playback_sender: Option<std::sync::mpsc::Sender<subsystems::playback::PlaybackCommand>>,
    // Story 2.x: capture_handle: Option<subsystems::capture::CaptureHandle>
    // Story 4.2: midi_handle: Option<subsystems::midi::MidiHandle>
    // Story 4.3: scheduler_handle: Option<subsystems::scheduler::SchedulerHandle>
    // Story 5.x: project_manager: Option<subsystems::project::ProjectManager>
    // Story 6.x: boot_manager: Option<subsystems::boot::BootManager>
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            playback_sender: None,
        }
    }
}

// ── App entry point ──────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Spawn the playback engine on its own OS thread before the Tauri builder.
    // The thread is isolated from the Tokio async runtime (NFR1, NFR5).
    // _playback_handle: dropping detaches the thread — it self-exits when
    // playback_sender is dropped with AppState on app shutdown.
    let (playback_sender, playback_receiver) =
        std::sync::mpsc::channel::<subsystems::playback::PlaybackCommand>();
    let _playback_handle = subsystems::playback::spawn_thread(playback_receiver);

    tauri::Builder::default()
        .manage(AppState { playback_sender: Some(playback_sender) })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::system::get_app_version,
            // Story 2.1: commands::network::list_interfaces,
            //            commands::network::set_capture_interface,
            // Story 2.2: commands::capture::start_capture,
            //            commands::capture::stop_capture,
            // Story 3.2: commands::playback::trigger_scene,
            //            commands::playback::stop_playback,
            //            commands::playback::set_speed,
            // Story 4.1: commands::keyboard::register_shortcut,
            // Story 4.2: commands::midi::get_midi_devices,
            //            commands::midi::assign_midi_trigger,
            // Story 4.3: commands::scheduler::set_schedule,
            // Story 5.x: commands::project::load_project,
            //            commands::project::save_project,
            // Story 6.x: commands::system::enable_autostart,
            //            commands::system::disable_autostart,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
