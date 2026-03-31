// commands/system.rs — Thin Tauri command wrappers for system-level operations
// No business logic here — delegates to subsystems/ or reads environment.
//
// Commands in this story:
//   get_app_version — returns the application version string (IPC pattern demonstrator)
//
// Commands added in later stories:
//   Story 6.1: enable_autostart, disable_autostart
//   Story 7.1: get_log_path, open_log_file
//   Story 7.3: check_update

/// Returns the application version from the Cargo package metadata.
///
/// This command serves as the IPC foundation demonstrator for Story 1.4:
/// it exercises the full `Result<T, String>` return type contract and
/// verifies that the TypeScript `invoke()` wrapper in `src/lib/tauri.ts` works.
#[tauri::command]
pub fn get_app_version() -> Result<String, String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_app_version_returns_ok_with_non_empty_version() {
        let version = get_app_version().expect("get_app_version should return Ok");
        assert!(!version.is_empty(), "Version string should not be empty");
    }

    #[test]
    fn get_app_version_matches_cargo_pkg_version() {
        let result = get_app_version().unwrap();
        assert_eq!(result, env!("CARGO_PKG_VERSION"));
    }
}
