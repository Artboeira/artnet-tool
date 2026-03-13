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

// ── App entry point ──────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        // TODO: register command handlers as subsystems are implemented
        // .invoke_handler(tauri::generate_handler![
        //     commands::network::list_interfaces,
        //     commands::capture::start_capture,
        //     ...
        // ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
